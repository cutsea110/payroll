use anyhow;
use chrono::NaiveDate;
use log::trace;

use crate::AddTimeCard;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use tx_app::{Response, Transaction};

// ユースケース: AddTimeCard トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    date: NaiveDate,
    hours: f32,

    dao: T,
}
impl<T> AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, date: NaiveDate, hours: f32, dao: T) -> Self {
        Self {
            id,
            date,
            hours,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> AddTimeCard for AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_hours(&self) -> f32 {
        self.hours
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("AddTimeCardTx::execute called");
        AddTimeCard::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
