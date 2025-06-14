use anyhow;
use log::trace;
use std::sync::{Arc, Mutex};
use tx_rs::Tx;

use abstract_tx::{ChangeMember, UsecaseError};
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Affiliation, EmployeeId, MemberId};
use payroll_factory::UnionAffiliationFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeMember トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeMemberTx<T, F>
where
    T: EmployeeDao,
{
    member_id: MemberId,
    emp_id: EmployeeId,
    dues: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeMemberTx<T, F>
where
    T: EmployeeDao,
{
    pub fn new(
        member_id: MemberId,
        emp_id: EmployeeId,
        dues: f32,
        dao: T,
        payroll_factory: F,
    ) -> Self {
        Self {
            member_id,
            emp_id,
            dues,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeMemberTx<T, F>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeMember for ChangeMemberTx<T, F>
where
    T: EmployeeDao,
    F: UnionAffiliationFactory,
{
    fn run_tx<'a, G, R>(&'a self, f: G) -> Result<R, UsecaseError>
    where
        G: FnOnce(Self::Ctx<'a>) -> Result<R, DaoError>,
    {
        trace!("run_tx called");
        // 今は DB しかないのでサービスレベルトランザクションが DB のトランザクションと同一視されている
        self.dao()
            .run_tx(f)
            .map_err(UsecaseError::ChangeMemberFailed)
    }

    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_affiliation(&self) -> Arc<Mutex<dyn Affiliation>> {
        self.payroll_factory
            .mk_affiliation(self.member_id, self.dues)
    }
    fn record_membership<'a>(&self, ctx: &mut Self::Ctx<'a>) -> Result<(), DaoError> {
        trace!("record_membership called");
        self.dao()
            .add_union_member(self.member_id, self.emp_id)
            .run(ctx)
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeMemberTx<T, F>
where
    T: EmployeeDao,
    F: UnionAffiliationFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("execute called");
        ChangeMember::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
