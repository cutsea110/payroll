use log::{debug, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;

// ユースケース: ChangeEmployeeName トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeEmployeeName: HaveEmployeeDao {
    fn get_id(&self) -> EmployeeId;
    fn get_new_name(&self) -> &str;
    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("ChangeEmployeeName::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("ChangeEmployeeName::run_tx called");
                let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                debug!(
                    r#"changing emp name: "{}" -> "{}""#,
                    emp.name(),
                    self.get_new_name()
                );
                emp.set_name(self.get_new_name());
                debug!(r#"changed emp name="{}""#, emp.name());
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::ChangeEmployeeNameFailed)
    }
}
