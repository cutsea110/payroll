use anyhow;
use chrono::NaiveDate;
use log::{debug, trace};
use std::{cell::RefCell, rc::Rc};

use abstract_tx::ChangeAffiliation;
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
    fn get_member_id(&self) -> MemberId {
        self.member_id
    }
    fn change(&self, aff: Rc<RefCell<dyn Affiliation>>) -> Result<(), DaoError> {
        trace!("AddServiceChargeTx::change called");
        aff.borrow_mut()
            .as_any_mut()
            .downcast_mut::<UnionAffiliation>()
            .ok_or(DaoError::UnexpectedError("didn't union affiliation".into()))?
            .add_service_charge(self.date, self.amount);
        debug!("service charge added: {:?}", aff.borrow());
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddServiceChargeTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("AddServiceChargeTx::execute called");
        ChangeAffiliation::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
