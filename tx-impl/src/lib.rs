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
                        debug!(
                            r#"changing emp name: "{}" -> "{}""#,
                            emp.name(),
                            self.get_new_name()
                        );
                        emp.set_name(self.get_new_name());
                        debug!(r#"changed emp name="{}""#, emp.name());
                        self.dao().save(emp).run(&mut ctx)
                    })
                    .map_err(UsecaseError::ChgEmpNameFailed)
            }
        }
    }
    pub use chg_name::*;

    mod chg_address {
        use log::{debug, trace};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;

        // ユースケース: ChgEmpAddress トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgEmpAddress: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_new_address(&self) -> &str;
            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("ChgEmpAddress::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("ChgEmpAddress::run_tx called");
                        let mut emp = self.dao().get(self.get_id()).run(&mut ctx)?;
                        debug!(
                            r#"changing emp address: "{}" -> "{}""#,
                            emp.address(),
                            self.get_new_address()
                        );
                        emp.set_address(self.get_new_address());
                        debug!(r#"changed emp address="{}""#, emp.address());
                        self.dao().save(emp).run(&mut ctx)
                    })
                    .map_err(UsecaseError::ChgEmpNameFailed)
            }
        }
    }
    pub use chg_address::*;

    mod chg_classification {
        use log::{debug, trace};
        use std::{cell::RefCell, rc::Rc};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentClassification, PaymentSchedule};

        // ユースケース: ChgClassification トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgClassification: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("ChgClassification::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("ChgClassification::run_tx called");
                        let mut emp = self.dao().get(self.get_id()).run(&mut ctx)?;
                        debug!(
                            "changing emp classification: {:?} -> {:?}",
                            emp.classification(),
                            self.get_classification()
                        );
                        emp.set_classification(self.get_classification());
                        debug!("changed emp classification={:?}", emp.classification());
                        debug!(
                            "changing emp schedule: {:?} -> {:?}",
                            emp.schedule(),
                            self.get_schedule()
                        );
                        emp.set_schedule(self.get_schedule());
                        debug!("changed emp schedule={:?}", emp.schedule());
                        self.dao().save(emp).run(&mut ctx)
                    })
                    .map_err(UsecaseError::ChgEmpNameFailed)
            }
        }
    }
    pub use chg_classification::*;

    mod chg_method {
        use log::{debug, trace};
        use std::{cell::RefCell, rc::Rc};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentMethod};

        // ユースケース: ChgMethod トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgMethod: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>>;
            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("ChgMethod::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("ChgMethod::run_tx called");
                        let mut emp = self.dao().get(self.get_id()).run(&mut ctx)?;
                        debug!(
                            "changing emp method: {:?} -> {:?}",
                            emp.method(),
                            self.get_method()
                        );
                        emp.set_method(self.get_method());
                        debug!("changed emp method={:?}", emp.method());
                        self.dao().save(emp).run(&mut ctx)
                    })
                    .map_err(UsecaseError::ChgEmpNameFailed)
            }
        }
    }
    pub use chg_method::*;
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

    mod add_hourly_emp_tx {
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
        use payroll_impl::{HoldMethod, HourlyClassification, WeeklySchedule};
        use tx_app::{Response, Transaction};

        // ユースケース: AddSalariedEmp トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddHourlyEmpTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            name: String,
            address: String,
            hourly_rate: f32,

            db: T,
        }
        impl<T> AddHourlyEmpTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, name: &str, address: &str, hourly_rate: f32, dao: T) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    hourly_rate,

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddHourlyEmpTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddEmp for AddHourlyEmpTx<T>
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
                Rc::new(RefCell::new(HourlyClassification::new(self.hourly_rate)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(WeeklySchedule))
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
                Rc::new(RefCell::new(NoAffiliation))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddHourlyEmpTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddHourlyEmpTx::execute called");
                AddEmp::execute(self)
                    .map(|_| Response::EmpId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_hourly_emp_tx::*;

    mod add_commissioned_emp_tx {
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
        use payroll_impl::{BiweeklySchedule, CommissionedClassification, HoldMethod};
        use tx_app::{Response, Transaction};

        // ユースケース: AddSalariedEmp トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddCommissionedEmpTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            name: String,
            address: String,
            salary: f32,
            commission_rate: f32,

            db: T,
        }
        impl<T> AddCommissionedEmpTx<T>
        where
            T: EmpDao,
        {
            pub fn new(
                id: EmpId,
                name: &str,
                address: &str,
                salary: f32,
                commission_rate: f32,
                dao: T,
            ) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    salary,
                    commission_rate,

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddCommissionedEmpTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddEmp for AddCommissionedEmpTx<T>
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
                Rc::new(RefCell::new(CommissionedClassification::new(
                    self.salary,
                    self.commission_rate,
                )))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(BiweeklySchedule))
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
                Rc::new(RefCell::new(NoAffiliation))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddCommissionedEmpTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddCommissionedEmpTx::execute called");
                AddEmp::execute(self)
                    .map(|_| Response::EmpId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_commissioned_emp_tx::*;

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

    mod chg_address_tx {
        use anyhow;
        use log::trace;

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::ChgEmpAddress;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            new_address: String,
            db: T,
        }
        impl<T> ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, new_address: &str, dao: T) -> Self {
                Self {
                    id,
                    new_address: new_address.to_string(),
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgEmpAddress for ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_new_address(&self) -> &str {
                &self.new_address
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgEmpAddressTx::execute called");
                ChgEmpAddress::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_address_tx::*;

    mod chg_salary_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgClassification;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentClassification};
        use payroll_impl::{MonthlySchedule, SalariedClassification};
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            salary: f32,

            db: T,
        }
        impl<T> ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, salary: f32, dao: T) -> Self {
                Self {
                    id,
                    salary,
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgClassification for ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(SalariedClassification::new(self.salary)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
                Rc::new(RefCell::new(MonthlySchedule))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgSalariedTx::execute called");
                ChgClassification::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_salary_tx::*;

    mod chg_hourly_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgClassification;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentClassification};
        use payroll_impl::{HourlyClassification, WeeklySchedule};
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            hourly_rate: f32,

            db: T,
        }
        impl<T> ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, hourly_rate: f32, dao: T) -> Self {
                Self {
                    id,
                    hourly_rate,
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgClassification for ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(HourlyClassification::new(self.hourly_rate)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
                Rc::new(RefCell::new(WeeklySchedule))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgHourlyTx::execute called");
                ChgClassification::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_hourly_tx::*;

    mod chg_commissioned_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgClassification;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentClassification};
        use payroll_impl::{BiweeklySchedule, CommissionedClassification};
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            salary: f32,
            commission_rate: f32,

            db: T,
        }
        impl<T> ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, salary: f32, commission_rate: f32, dao: T) -> Self {
                Self {
                    id,
                    salary,
                    commission_rate,

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgClassification for ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(CommissionedClassification::new(
                    self.salary,
                    self.commission_rate,
                )))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
                Rc::new(RefCell::new(BiweeklySchedule))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgCommissionedTx::execute called");
                ChgClassification::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_commissioned_tx::*;

    mod chg_hold_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgMethod;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentMethod};
        use payroll_impl::HoldMethod;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgHoldTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,

            db: T,
        }
        impl<T> ChgHoldTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, dao: T) -> Self {
                Self { id, db: dao }
            }
        }

        impl<T> HaveEmpDao for ChgHoldTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgMethod for ChgHoldTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgHoldTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgHoldTx::execute called");
                ChgMethod::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_hold_tx::*;

    mod chg_direct_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgMethod;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentMethod};
        use payroll_impl::DirectMethod;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgDirectTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            bank: String,
            account: String,

            db: T,
        }
        impl<T> ChgDirectTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, bank: &str, account: &str, dao: T) -> Self {
                Self {
                    id,
                    bank: bank.to_string(),
                    account: account.to_string(),
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgDirectTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgMethod for ChgDirectTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(DirectMethod::new(&self.bank, &self.account)))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgDirectTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgDirectTx::execute called");
                ChgMethod::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_direct_tx::*;

    mod chg_mail_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgMethod;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentMethod};
        use payroll_impl::MailMethod;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgMailTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            address: String,

            db: T,
        }
        impl<T> ChgMailTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, address: &str, dao: T) -> Self {
                Self {
                    id,
                    address: address.to_string(),

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgMailTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgMethod for ChgMailTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(MailMethod::new(&self.address)))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgMailTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgMailTx::execute called");
                ChgMethod::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_mail_tx::*;
}
pub use tx_impl::*;
