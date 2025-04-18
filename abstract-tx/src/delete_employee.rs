use log::{debug, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;

// ユースケース: DeleteEmployee トランザクション(抽象レベルのビジネスロジック)
pub trait DeleteEmployee: HaveEmployeeDao {
    fn get_id(&self) -> EmployeeId;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("run_tx called");
                let emp_id = self.get_id();
                debug!("execute: emp_id={}", emp_id);
                self.dao().delete(emp_id).run(&mut ctx)
            })
            .map(|_| ())
            .map_err(UsecaseError::DeleteEmployeeFailed)
    }
}
