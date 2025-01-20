use anyhow;
use log::trace;
use tx_rs::Tx;

use abstract_tx::ChangeMember;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use payroll_factory::PayrollFactory;
use payroll_impl::UnionAffiliation;
use tx_app::{Response, Transaction};

// ユースケース: ChangeNoMember トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeNoMemberTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    emp_id: EmployeeId,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeNoMemberTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
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
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeMember for ChangeNoMemberTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_affiliation(&self) -> std::rc::Rc<std::cell::RefCell<dyn payroll_domain::Affiliation>> {
        self.payroll_factory.mk_no_affiliation()
    }
    fn record_membership<'a>(
        &self,
        ctx: &mut <Self as HaveEmployeeDao>::Ctx<'a>,
    ) -> Result<(), dao::DaoError> {
        trace!("record_membership called");
        let emp = self.dao().fetch(self.emp_id).run(ctx)?;
        let member_id = emp
            .affiliation()
            .borrow()
            .as_any()
            .downcast_ref::<UnionAffiliation>()
            .ok_or(DaoError::UnexpectedError("didn't union affiliation".into()))?
            .member_id();

        self.dao().remove_union_member(member_id).run(ctx)
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeNoMemberTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeNoMemberTx::execute called");
        ChangeMember::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
