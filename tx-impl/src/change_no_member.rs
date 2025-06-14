use anyhow;
use log::{debug, trace};
use std::sync::{Arc, Mutex};
use tx_rs::Tx;

use abstract_tx::ChangeMember;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Affiliation, EmployeeId};
use payroll_factory::NoAffiliationFactory;
use payroll_impl::UnionAffiliation;
use tx_app::{Response, Transaction};

// ユースケース: ChangeNoMember トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeNoMemberTx<T, F>
where
    T: EmployeeDao,
{
    emp_id: EmployeeId,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeNoMemberTx<T, F>
where
    T: EmployeeDao,
{
    pub fn new(emp_id: EmployeeId, dao: T, payroll_factory: F) -> Self {
        Self {
            emp_id,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeNoMemberTx<T, F>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeMember for ChangeNoMemberTx<T, F>
where
    T: EmployeeDao,
    F: NoAffiliationFactory,
{
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_affiliation(&self) -> Arc<Mutex<dyn Affiliation>> {
        self.payroll_factory.mk_affiliation()
    }
    fn record_membership<'a>(
        &self,
        ctx: &mut <Self as HaveEmployeeDao>::Ctx<'a>,
    ) -> Result<(), dao::DaoError> {
        trace!("record_membership called");
        let emp = self.dao().fetch(self.emp_id).run(ctx)?;
        let member_id = emp
            .affiliation()
            .lock()
            .unwrap()
            .as_any()
            .downcast_ref::<UnionAffiliation>()
            .ok_or(DaoError::UnexpectedError("didn't union affiliation".into()))?
            .member_id();
        debug!("delete union member: {}", member_id);
        self.dao().delete_union_member(member_id).run(ctx)
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeNoMemberTx<T, F>
where
    T: EmployeeDao,
    F: NoAffiliationFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("execute called");
        ChangeMember::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
