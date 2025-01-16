use chrono::NaiveDate;
use log::trace;
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use payroll_impl::CommissionedClassification;

// ユースケース: AddSalesReceipt トランザクション(抽象レベルのビジネスロジック)
pub trait AddSalesReceipt: HaveEmployeeDao {
    fn get_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_amount(&self) -> f32;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("AddSalesReceipt::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("AddSalesReceipt::run_tx called");
                let emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                emp.classification()
                    .borrow_mut()
                    .as_any_mut()
                    .downcast_mut::<CommissionedClassification>()
                    .ok_or(DaoError::UnexpectedError(
                        "classification is not CommissionedClassification".into(),
                    ))?
                    .add_sales_receipt(self.get_date(), self.get_amount());
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::AddSalesReceiptFailed)
    }
}
