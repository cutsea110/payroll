// ユースケースのトランザクションのインターフェース
mod interface {
    use thiserror::Error;

    // dao にのみ依存
    use dao::DaoError;

    #[derive(Debug, Clone, Error)]
    pub enum UsecaseError {
        #[error("add employee failed: {0}")]
        AddEmpFailed(DaoError),
        #[error("change employee name failed: {0}")]
        ChgEmpNameFailed(DaoError),
    }

    mod add_emp {
        use log::{debug, trace};
        use std::{cell::RefCell, rc::Rc};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{
            Affiliation, Emp, EmpId, PaymentClassification, PaymentMethod, PaymentSchedule,
        };

        // ユースケース: AddEmp トランザクション(抽象レベルのビジネスロジック)
        pub trait AddEmp: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_name(&self) -> &str;
            fn get_address(&self) -> &str;
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>>;
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;

            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("AddEmp::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("AddEmp::run_tx called");
                        let emp = Emp::new(
                            self.get_id(),
                            self.get_name(),
                            self.get_address(),
                            self.get_classification(),
                            self.get_schedule(),
                            self.get_method(),
                            self.get_affiliation(),
                        );
                        debug!("AddEmp::execute: emp={:?}", emp);
                        self.dao().save(emp).run(&mut ctx)
                    })
                    .map_err(UsecaseError::AddEmpFailed)
            }
        }
    }
    pub use add_emp::*;

    mod chg_name {
        use log::{debug, trace};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;

        // ユースケース: ChgEmpName トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgEmpName: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_new_name(&self) -> &str;
            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("ChgEmpName::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("ChgEmpName::run_tx called");
                        let mut emp = self.dao().get(self.get_id()).run(&mut ctx)?;
                        debug!("changing emp name: emp={:?}", emp);
                        emp.set_name(self.get_new_name());
                        debug!("changed emp name: emp={:?}", emp);
                        self.dao().save(emp).run(&mut ctx)
                    })
                    .map_err(UsecaseError::ChgEmpNameFailed)
            }
        }
    }
    pub use chg_name::*;
}
pub use interface::*;

// ユースケースのトランザクションの実装
mod tx_impl {
    mod add_salaried_emp_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::AddEmp;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{
            Affiliation, EmpId, NoAffiliation, PaymentClassification, PaymentMethod,
            PaymentSchedule,
        };
        use payroll_impl::{HoldMethod, MonthlySchedule, SalariedClassification};
        use tx_app::{Response, Transaction};

        // ユースケース: AddSalariedEmp トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            name: String,
            address: String,
            salary: f32,

            db: T,
        }
        impl<T> AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, name: &str, address: &str, salary: f32, dao: T) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    salary,

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddEmp for AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_name(&self) -> &str {
                &self.name
            }
            fn get_address(&self) -> &str {
                &self.address
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(SalariedClassification::new(self.salary)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(MonthlySchedule))
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
                Rc::new(RefCell::new(NoAffiliation))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddSalariedEmpTx::execute called");
                AddEmp::execute(self)
                    .map(|_| Response::EmpId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_salaried_emp_tx::*;

    mod chg_name_tx {
        use anyhow;
        use log::trace;

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::ChgEmpName;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            new_name: String,
            db: T,
        }
        impl<T> ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, new_name: &str, dao: T) -> Self {
                Self {
                    id,
                    new_name: new_name.to_string(),
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgEmpName for ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_new_name(&self) -> &str {
                &self.new_name
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgEmpNameTx::execute called");
                ChgEmpName::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_name_tx::*;
}
pub use tx_impl::*;
