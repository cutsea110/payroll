use chrono::NaiveDate;
use log::{debug, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::Paycheck;

// ユースケース: Payday トランザクション(抽象レベルのビジネスロジック)
pub trait Payday: HaveEmployeeDao {
    // TODO: このレイヤはユースケースで、本来 run_tx はサービスレベルにあるべき
    // そしてサービスレベルの実装は EmployeeDao トレイトではなく具体的な Db 構造体を相手に run_tx を実装するべき
    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, UsecaseError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

    fn get_pay_date(&self) -> NaiveDate;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("execute called");
        self.run_tx(|mut ctx| {
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
    }
}
