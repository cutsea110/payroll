use chrono::NaiveDate;
use log::trace;
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use payroll_impl::HourlyClassification;

// ユースケース: AddTimeCard トランザクション(抽象レベルのビジネスロジック)
pub trait AddTimeCard: HaveEmployeeDao {
    fn get_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_hours(&self) -> f32;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("AddTimeCard::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("AddTimeCard::run_tx called");
                let emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                emp.classification()
                    .borrow_mut()
                    .as_any_mut()
                    .downcast_mut::<HourlyClassification>()
                    .ok_or(DaoError::UnexpectedError(
                        "classification is not HourlyClassification".into(),
                    ))?
                    .add_timecard(self.get_date(), self.get_hours());
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::AddTimeCardFailed)
    }
}
