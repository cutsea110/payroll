use anyhow;
use chrono::NaiveDate;
use log::trace;

use crate::AddSalesReceipt;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use tx_app::{Response, Transaction};

// ユースケース: AddSalesReceipt トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    date: NaiveDate,
    amount: f32,

    dao: T,
}
impl<T> AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, date: NaiveDate, amount: f32, dao: T) -> Self {
        Self {
            id,
            date,
            amount,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> AddSalesReceipt for AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_date(&self) -> NaiveDate {
        self.date
    }
    fn get_amount(&self) -> f32 {
        self.amount
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("AddSalesReceiptTx::execute called");
        AddSalesReceipt::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
