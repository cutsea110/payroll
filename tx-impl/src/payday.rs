use anyhow;
use chrono::NaiveDate;
use log::trace;

use abstract_tx::Payday;
use dao::{EmployeeDao, HaveEmployeeDao};
use tx_app::{Response, Transaction};

// ユースケース: Payday トランザクションの実装 (struct)
#[derive(Debug)]
pub struct PaydayTx<T>
where
    T: EmployeeDao,
{
    pay_date: NaiveDate,

    dao: T,
}
impl<T> PaydayTx<T>
where
    T: EmployeeDao,
{
    pub fn new(pay_date: NaiveDate, dao: T) -> Self {
        Self { pay_date, dao }
    }
}

impl<T> HaveEmployeeDao for PaydayTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> Payday for PaydayTx<T>
where
    T: EmployeeDao,
{
    fn get_pay_date(&self) -> NaiveDate {
        self.pay_date
    }
}
// 共通インターフェースの実装
impl<T> Transaction for PaydayTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("PaydayTx::execute called");
        Payday::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
