mod error;
pub use error::DaoError;

use payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};

pub trait PayrollDao<Ctx> {
    fn insert(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
    fn delete(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn fetch(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = Employee, Err = DaoError>;
    fn update(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    fn fetch_all(&self) -> impl tx_rs::Tx<Ctx, Item = Vec<Employee>, Err = DaoError>;
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
        pc: Paycheck,
    ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
}

pub trait HavePayrollDao<Ctx> {
    fn dao(&self) -> &impl PayrollDao<Ctx>;
}
