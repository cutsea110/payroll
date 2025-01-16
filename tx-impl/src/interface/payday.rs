use chrono::NaiveDate;
use log::trace;
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::Paycheck;

// ユースケース: Payday トランザクション(抽象レベルのビジネスロジック)
pub trait Payday: HaveEmployeeDao {
    fn get_pay_date(&self) -> NaiveDate;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("Payday::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("Payday::run_tx called");
                let mut emps = self.dao().fetch_all().run(&mut ctx)?;
                for (emp_id, emp) in emps.iter_mut() {
                    if emp.is_pay_date(self.get_pay_date()) {
                        let period = emp.get_pay_period(self.get_pay_date());
                        let mut pc = Paycheck::new(period);
                        emp.payday(&mut pc);
                        self.dao().record_paycheck(*emp_id, pc).run(&mut ctx)?;
                    }
                }
                Ok(())
            })
            .map_err(UsecaseError::PaydayFailed)
    }
}
