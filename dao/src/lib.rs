use thiserror::Error;

use payroll_domain::{Emp, EmpId};

#[derive(Debug, Clone, Error)]
pub enum DaoError {
    #[error("emp_id={0} not found")]
    NotFound(EmpId),
}

// Dao のインターフェース (AddEmpTx にはこちらにだけ依存させる)
pub trait EmpDao {
    type Ctx<'a>;

    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, DaoError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

    fn get<'a>(&self, id: EmpId) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Emp, Err = DaoError>;
    fn save<'a>(&self, emp: Emp) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
}

pub trait HaveEmpDao {
    type Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>>;
}
