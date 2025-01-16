use anyhow;
use log::trace;

use crate::ChangeNoMember;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use tx_app::{Response, Transaction};

// ユースケース: ChangeNoMember トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeNoMemberTx<T>
where
    T: EmployeeDao,
{
    emp_id: EmployeeId,

    dao: T,
}
impl<T> ChangeNoMemberTx<T>
where
    T: EmployeeDao,
{
    pub fn new(emp_id: EmployeeId, dao: T) -> Self {
        Self { emp_id, dao }
    }
}

impl<T> HaveEmployeeDao for ChangeNoMemberTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeNoMember for ChangeNoMemberTx<T>
where
    T: EmployeeDao,
{
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeNoMemberTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeNoMemberTx::execute called");
        ChangeNoMember::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
