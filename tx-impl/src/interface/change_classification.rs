use log::{debug, trace};
use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};

// ユースケース: ChangeClassification トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeClassification: HaveEmployeeDao {
    fn get_id(&self) -> EmployeeId;
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("ChangeClassification::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("ChangeClassification::run_tx called");
                let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                debug!(
                    "changing emp classification: {:?} -> {:?}",
                    emp.classification(),
                    self.get_classification()
                );
                emp.set_classification(self.get_classification());
                debug!("changed emp classification={:?}", emp.classification());
                debug!(
                    "changing emp schedule: {:?} -> {:?}",
                    emp.schedule(),
                    self.get_schedule()
                );
                emp.set_schedule(self.get_schedule());
                debug!("changed emp schedule={:?}", emp.schedule());
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::ChangePaymentClassificationFailed)
    }
}
