use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChangeMember;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, MemberId};
use payroll_impl::UnionAffiliation;
use tx_app::{Response, Transaction};

// ユースケース: ChangeMember トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeMemberTx<T>
where
    T: EmployeeDao,
{
    member_id: MemberId,
    emp_id: EmployeeId,
    dues: f32,

    dao: T,
}
impl<T> ChangeMemberTx<T>
where
    T: EmployeeDao,
{
    pub fn new(member_id: MemberId, emp_id: EmployeeId, dues: f32, dao: T) -> Self {
        Self {
            member_id,
            emp_id,
            dues,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for ChangeMemberTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeMember for ChangeMemberTx<T>
where
    T: EmployeeDao,
{
    fn get_member_id(&self) -> MemberId {
        self.member_id
    }
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_dues(&self) -> f32 {
        self.dues
    }
    fn get_affiliation(&self) -> Rc<RefCell<dyn payroll_domain::Affiliation>> {
        Rc::new(RefCell::new(UnionAffiliation::new(
            self.get_member_id(),
            self.get_dues(),
        )))
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeMemberTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeMemberTx::execute called");
        ChangeMember::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
