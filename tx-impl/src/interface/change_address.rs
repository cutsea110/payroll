use log::{debug, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;

// ユースケース: ChangeEmployeeAddress トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeEmployeeAddress: HaveEmployeeDao {
    fn get_id(&self) -> EmployeeId;
    fn get_new_address(&self) -> &str;
    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("ChangeEmployeeAddress::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("ChangeEmployeeAddress::run_tx called");
                let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                debug!(
                    r#"changing emp address: "{}" -> "{}""#,
                    emp.address(),
                    self.get_new_address()
                );
                emp.set_address(self.get_new_address());
                debug!(r#"changed emp address="{}""#, emp.address());
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::ChangeEmployeeAddressFailed)
    }
}
