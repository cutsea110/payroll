use chrono::NaiveDate;
use log::{debug, error, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, Paycheck};

// テストフレームワーク: VerifyPaycheck トランザクション(抽象レベルのビジネスロジック)
pub trait VerifyPaycheck: HaveEmployeeDao {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_pay_date(&self) -> NaiveDate;
    fn expected(&self) -> f32;
    fn actual(&self, pc: &Paycheck) -> f32;

    fn execute<'a>(&self) -> Result<bool, UsecaseError> {
        trace!("Payday::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                let emp_id = self.get_emp_id();
                let pay_date = self.get_pay_date();
                let paycheck = self.dao().find_paycheck(emp_id, pay_date).run(&mut ctx)?;
                debug!("found paycheck={:?}", paycheck);
                let pass = self.expected() == self.actual(&paycheck);
                if !pass {
                    error!("Failed to verify paycheck: {:?}", paycheck);
                }
                // assert_eq!(self.expected(), self.actual(&paycheck));
                Ok(pass)
            })
            .map_err(UsecaseError::FetchPaycheckFailed)
    }
}
