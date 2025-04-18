use chrono::NaiveDate;
use log::{debug, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::Paycheck;

// ユースケース: Payday トランザクション(抽象レベルのビジネスロジック)
pub trait Payday: HaveEmployeeDao {
    fn get_pay_date(&self) -> NaiveDate;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("run_tx called");
                let mut emps = self.dao().fetch_all().run(&mut ctx)?;
                let paydate = self.get_pay_date();

                for (emp_id, emp) in emps.iter_mut() {
                    if emp.is_pay_date(paydate) {
                        debug!("execute: payday for emp_id={}", emp_id);
                        let period = emp.get_pay_period(paydate);
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
