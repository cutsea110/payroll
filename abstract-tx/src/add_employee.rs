use log::{debug, trace};
use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{
    Affiliation, Employee, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
};

// ユースケース: AddEmployee トランザクション(抽象レベルのビジネスロジック)
pub trait AddEmployee: HaveEmployeeDao {
    fn get_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>>;
    fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("AddEmployee::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("AddEmployee::run_tx called");
                let emp = Employee::new(
                    self.get_id(),
                    self.get_name(),
                    self.get_address(),
                    self.get_classification(),
                    self.get_schedule(),
                    self.get_method(),
                    self.get_affiliation(),
                );
                debug!("AddEmployee::execute: emp={:?}", emp);
                self.dao().add(emp).run(&mut ctx)
            })
            .map(|_| ())
            .map_err(UsecaseError::AddEmployeeFailed)
    }
}
