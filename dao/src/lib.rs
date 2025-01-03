use thiserror::Error;

use payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum DaoError {
    #[error("EmployeeId({0}) already exists")]
    AlreadyExists(EmployeeId),
    #[error("EmployeeId({0}) not found")]
    NotFound(EmployeeId),
    #[error("MemberId({0}) is already a union member of EmployeeId({1})")]
    AlreadyUnionMember(MemberId, EmployeeId),
    #[error("MemberId({0}) is not a union member")]
    NotYetUnionMember(MemberId),
}

pub trait EmployeeDao<Ctx> {
    fn insert(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
    fn remove(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn fetch(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = Employee, Err = DaoError>;
    fn fetch_all(&self) -> impl tx_rs::Tx<Ctx, Item = Vec<(EmployeeId, Employee)>, Err = DaoError>;
    fn update(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn add_union_member(
        &self,
        member_id: MemberId,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn remove_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn find_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
    fn record_paycheck(
        &self,
        emp_id: EmployeeId,
        paycheck: Paycheck,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
}

pub trait HaveEmployeeDao<Ctx> {
    fn dao(&self) -> &impl EmployeeDao<Ctx>;
}
