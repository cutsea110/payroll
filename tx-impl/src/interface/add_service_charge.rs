use chrono::NaiveDate;
use log::trace;
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::MemberId;
use payroll_impl::UnionAffiliation;

// ユースケース: AddServiceCharge トランザクション(抽象レベルのビジネスロジック)
pub trait AddServiceCharge: HaveEmployeeDao {
    fn get_member_id(&self) -> MemberId;
    fn get_date(&self) -> NaiveDate;
    fn get_amount(&self) -> f32;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("AddServiceCharge::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("AddServiceCharge::run_tx called");
                let emp_id = self
                    .dao()
                    .find_union_member(self.get_member_id())
                    .run(&mut ctx)?;
                let emp = self.dao().fetch(emp_id).run(&mut ctx)?;
                emp.affiliation()
                    .borrow_mut()
                    .as_any_mut()
                    .downcast_mut::<UnionAffiliation>()
                    .ok_or(DaoError::UnexpectedError("didn't union affiliation".into()))?
                    .add_service_charge(self.get_date(), self.get_amount());
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::AddServiceChargeFailed)
    }
}
