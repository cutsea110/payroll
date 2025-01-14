use thiserror::Error;

use payroll_domain::{Emp, EmpId, MemberId};

#[derive(Debug, Clone, Error)]
pub enum DaoError {
    #[error("emp_id={0} already exists")]
    AlreadyExists(EmpId),
    #[error("emp_id={0} not found")]
    NotFound(EmpId),
    #[error("union member_id={0} emp_id={1} already exists")]
    UnionMemberAlreadyExists(MemberId, EmpId),
    #[error("union member_id={0} not found")]
    UnionMemberNotFound(MemberId),
    #[error("unexpected error: {0}")]
    UnexpectedError(String),
}

// Dao のインターフェース (AddEmpTx にはこちらにだけ依存させる)
pub trait EmpDao {
    type Ctx<'a>;

    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, DaoError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

    fn insert<'a>(&self, emp: Emp) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmpId, Err = DaoError>;
    fn remove<'a>(&self, id: EmpId) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
    fn fetch<'a>(&self, id: EmpId) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Emp, Err = DaoError>;
    fn fetch_all<'a>(
        &self,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Vec<(EmpId, Emp)>, Err = DaoError>;
    fn update<'a>(&self, emp: Emp) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
    fn add_union_member<'a>(
        &self,
        member_id: MemberId,
        emp_id: EmpId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
    fn remove_union_member<'a>(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
    fn find_union_member<'a>(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmpId, Err = DaoError>;
    fn record_paycheck<'a>(
        &self,
        emp_id: EmpId,
        paycheck: payroll_domain::Paycheck,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
}

pub trait HaveEmpDao {
    type Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>>;
}
