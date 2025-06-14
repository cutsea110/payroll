use log::{debug, trace};
use std::sync::{Arc, Mutex};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{
    Affiliation, Employee, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
};

// ユースケース: AddEmployee トランザクション(抽象レベルのビジネスロジック)
pub trait AddEmployee: HaveEmployeeDao {
    // サービスレベルトランザクション
    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, UsecaseError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

    fn get_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_classification(&self) -> Arc<Mutex<dyn PaymentClassification>>;
    fn get_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>>;
    fn get_method(&self) -> Arc<Mutex<dyn PaymentMethod>>;
    fn get_affiliation(&self) -> Arc<Mutex<dyn Affiliation>>;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("execute called");
        self.run_tx(|mut ctx| {
            trace!("run_tx called");
            let emp = Employee::new(
                self.get_id(),
                self.get_name(),
                self.get_address(),
                self.get_classification(),
                self.get_schedule(),
                self.get_method(),
                self.get_affiliation(),
            );
            debug!("execute: emp={:?}", emp);
            self.dao().add(emp).run(&mut ctx).map(|_| ())
        })
    }
}
