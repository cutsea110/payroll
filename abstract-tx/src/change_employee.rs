use log::{debug, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};

// ユースケース: ChangeEmployee トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeEmployee: HaveEmployeeDao {
    // サービスレベルトランザクション
    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, UsecaseError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

    fn get_id(&self) -> EmployeeId;
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError>;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("execute called");
        self.run_tx(|mut ctx| {
            trace!("run_tx called");
            let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
            debug!("changing emp={:?}", emp);
            self.change(&mut emp)?;
            debug!("changed emp={:?}", emp);
            self.dao().update(emp).run(&mut ctx)
        })
    }
}
