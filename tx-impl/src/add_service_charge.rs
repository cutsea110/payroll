use anyhow;
use chrono::NaiveDate;
use log::{debug, trace};
use std::sync::{Arc, Mutex};

use abstract_tx::{ChangeAffiliation, UsecaseError};
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Affiliation, MemberId};
use payroll_impl::UnionAffiliation;
use tx_app::{Response, Transaction};

// ユースケース: AddServiceCharge トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddServiceChargeTx<T>
where
    T: EmployeeDao,
{
    member_id: MemberId,
    date: NaiveDate,
    amount: f32,

    dao: T,
}
impl<T> AddServiceChargeTx<T>
where
    T: EmployeeDao,
{
    pub fn new(member_id: MemberId, date: NaiveDate, amount: f32, dao: T) -> Self {
        Self {
            member_id,
            date,
            amount,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for AddServiceChargeTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeAffiliation for AddServiceChargeTx<T>
where
    T: EmployeeDao,
{
    fn run_tx<'a, G, R>(&'a self, f: G) -> Result<R, UsecaseError>
    where
        G: FnOnce(Self::Ctx<'a>) -> Result<R, DaoError>,
    {
        trace!("run_tx called");
        // 今は DB しかないのでサービスレベルトランザクションが DB のトランザクションと同一視されている
        self.dao()
            .run_tx(f)
            .map_err(UsecaseError::AddEmployeeFailed)
    }

    fn get_member_id(&self) -> MemberId {
        self.member_id
    }
    fn change(&self, aff: Arc<Mutex<dyn Affiliation>>) -> Result<(), DaoError> {
        trace!("change called");
        aff.lock()
            .unwrap()
            .as_any_mut()
            .downcast_mut::<UnionAffiliation>()
            .ok_or(DaoError::UnexpectedError("didn't union affiliation".into()))?
            .add_service_charge(self.date, self.amount);
        debug!("service charge added: {:?}", aff.lock().unwrap());
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddServiceChargeTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("execute called");
        ChangeAffiliation::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
