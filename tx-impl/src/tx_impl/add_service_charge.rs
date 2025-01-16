use anyhow;
use chrono::NaiveDate;
use log::trace;

use crate::AddServiceCharge;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::MemberId;
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
impl<T> AddServiceCharge for AddServiceChargeTx<T>
where
    T: EmployeeDao,
{
    fn get_member_id(&self) -> MemberId {
        self.member_id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_amount(&self) -> f32 {
        self.amount
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddServiceChargeTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("AddServiceChargeTx::execute called");
        AddServiceCharge::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
