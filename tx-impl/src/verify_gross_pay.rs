use anyhow;
use chrono::NaiveDate;
use log::trace;

use abstract_tx::VerifyPaycheck;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, Paycheck};
use tx_app::{Response, Transaction};

// ユースケース: VerifyGrossPay トランザクションの実装 (struct)
#[derive(Debug)]
pub struct VerifyGrossPayTx<T>
where
    T: EmployeeDao,
{
    emp_id: EmployeeId,
    pay_date: NaiveDate,
    gross_pay: f32,

    dao: T,
}
impl<T> VerifyGrossPayTx<T>
where
    T: EmployeeDao,
{
    pub fn new(emp_id: EmployeeId, pay_date: NaiveDate, gross_pay: f32, dao: T) -> Self {
        Self {
            emp_id,
            pay_date,
            gross_pay,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for VerifyGrossPayTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> VerifyPaycheck for VerifyGrossPayTx<T>
where
    T: EmployeeDao,
{
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_pay_date(&self) -> NaiveDate {
        self.pay_date
    }
    fn expected(&self) -> f32 {
        self.gross_pay
    }
    fn actual(&self, pc: &Paycheck) -> f32 {
        pc.gross_pay()
    }
}
// 共通インターフェースの実装
impl<T> Transaction for VerifyGrossPayTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("VerifyGrossPayTx::execute called");
        VerifyPaycheck::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
