use log::{debug, trace};
use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentMethod};

// ユースケース: ChgMethod トランザクション(抽象レベルのビジネスロジック)
pub trait ChgMethod: HaveEmployeeDao {
    fn get_id(&self) -> EmployeeId;
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>>;
    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("ChgMethod::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("ChgMethod::run_tx called");
                let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                debug!(
                    "changing emp method: {:?} -> {:?}",
                    emp.method(),
                    self.get_method()
                );
                emp.set_method(self.get_method());
                debug!("changed emp method={:?}", emp.method());
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::ChangePaymentMethodFailed)
    }
}
