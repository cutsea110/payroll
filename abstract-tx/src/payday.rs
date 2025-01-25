use std::collections::HashMap;

use chrono::NaiveDate;
use log::{debug, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, Paycheck};

// ユースケース: Payday トランザクション(抽象レベルのビジネスロジック)
pub trait Payday: HaveEmployeeDao {
    fn get_pay_date(&self) -> NaiveDate;

    fn execute<'a>(&self) -> Result<HashMap<EmployeeId, Paycheck>, UsecaseError> {
        trace!("Payday::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("Payday::run_tx called");
                let mut emps = self.dao().fetch_all().run(&mut ctx)?;
                let paydate = self.get_pay_date();

                let mut paychecks = HashMap::new();
                for (emp_id, emp) in emps.iter_mut() {
                    if emp.is_pay_date(paydate) {
                        debug!(
                            "Payday::execute: payday for emp_id={} at date={}",
                            emp_id, paydate
                        );
                        let period = emp.get_pay_period(paydate);
                        let mut pc = Paycheck::new(period);
                        emp.payday(&mut pc);
                        self.dao()
                            .record_paycheck(*emp_id, pc.clone())
                            .run(&mut ctx)?;
                        debug!("Payday::execute: succeed to pay for emp_id={}", emp_id);
                        paychecks.insert(*emp_id, pc);
                    }
                }

                Ok(paychecks)
            })
            .map_err(UsecaseError::PaydayFailed)
    }
}
