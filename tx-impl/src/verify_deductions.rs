use anyhow;
use chrono::NaiveDate;
use log::trace;

use abstract_tx::VerifyPaycheck;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, Paycheck};
use tx_app::{Response, Transaction};

// テストフレームワーク: VerifyDeductions トランザクションの実装 (struct)
#[derive(Debug)]
pub struct VerifyDeductionsTx<T>
where
    T: EmployeeDao,
{
    emp_id: EmployeeId,
    pay_date: NaiveDate,
    deductions: f32,

    dao: T,
}
impl<T> VerifyDeductionsTx<T>
where
    T: EmployeeDao,
{
    pub fn new(emp_id: EmployeeId, pay_date: NaiveDate, deductions: f32, dao: T) -> Self {
        Self {
            emp_id,
            pay_date,
            deductions,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for VerifyDeductionsTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> VerifyPaycheck for VerifyDeductionsTx<T>
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
        self.deductions
    }
    fn actual(&self, pc: &Paycheck) -> f32 {
        pc.deductions()
    }
}
// 共通インターフェースの実装
impl<T> Transaction for VerifyDeductionsTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("VerifyDeductionsTx::execute called");
        VerifyPaycheck::execute(self)
            .map(Response::Verified)
            .map_err(Into::into)
    }
}
