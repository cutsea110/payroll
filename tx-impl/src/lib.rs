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
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{Emp, EmpId};

        // ユースケース: AddEmp トランザクション(抽象レベルのビジネスロジック)
        pub trait AddEmp: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_name(&self) -> &str;
            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("AddEmp::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("AddEmp::run_tx called");
                        let emp = Emp::new(self.get_id(), self.get_name());
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
    mod add_emp_tx {
        use anyhow;
        use log::trace;

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::AddEmp;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;
        use tx_app::{Response, Transaction};

        // ユースケース: AddEmp トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddEmpTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            name: String,
            db: T,
        }
        impl<T> AddEmpTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, name: &str, dao: T) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddEmpTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddEmp for AddEmpTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_name(&self) -> &str {
                &self.name
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddEmpTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddEmpTx::execute called");
                AddEmp::execute(self)
                    .map(|_| Response::EmpId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_emp_tx::*;

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
