use chrono::NaiveDate;
use thiserror::Error;

use payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};

#[derive(Debug, Clone, Error)]
pub enum DaoError {
    #[error("emp_id={0} already exists")]
    EmployeeAlreadyExists(EmployeeId),
    #[error("emp_id={0} not found")]
    EmployeeNotFound(EmployeeId),
    #[error("union member_id={0} emp_id={1} already exists")]
    MemberAlreadyExists(MemberId, EmployeeId),
    #[error("union member_id={0} not found")]
    MemberNotFound(MemberId),
    #[error("unexpected error: {0}")]
    UnexpectedError(String),
    #[error("paycheck not found: emp_id={0}, pay_date={1}")]
    PaycheckNotFound(EmployeeId, NaiveDate),
}

pub trait EmployeeDao {
    type Ctx<'a>;

    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, DaoError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

    fn add<'a>(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmployeeId, Err = DaoError>;
    fn delete<'a>(
        &self,
        id: EmployeeId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
    fn fetch<'a>(
        &self,
        id: EmployeeId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Employee, Err = DaoError>;
    fn fetch_all<'a>(
        &self,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Vec<(EmployeeId, Employee)>, Err = DaoError>;
    fn update<'a>(&self, emp: Employee)
        -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
    fn add_union_member<'a>(
        &self,
        member_id: MemberId,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
    fn delete_union_member<'a>(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
    fn find_union_member<'a>(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmployeeId, Err = DaoError>;
    fn record_paycheck<'a>(
        &self,
        emp_id: EmployeeId,
        paycheck: Paycheck,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
}

pub trait HaveEmployeeDao {
    type Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>>;
}
