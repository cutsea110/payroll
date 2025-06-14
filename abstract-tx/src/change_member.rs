use log::{debug, trace};
use std::sync::{Arc, Mutex};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Affiliation, EmployeeId};

// ユースケース: ChangeMember トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeMember: HaveEmployeeDao {
    // サービスレベルトランザクション
    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, UsecaseError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

    fn get_emp_id(&self) -> EmployeeId;
    fn get_affiliation(&self) -> Arc<Mutex<dyn Affiliation>>;
    fn record_membership<'a>(&self, ctx: &mut Self::Ctx<'a>) -> Result<(), DaoError>;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("execute called");
        self.run_tx(|mut ctx| {
            trace!("run_tx called");
            self.record_membership(&mut ctx)?;

            let mut emp = self.dao().fetch(self.get_emp_id()).run(&mut ctx)?;
            debug!(
                "changing emp member: {:?} -> {:?}",
                emp.affiliation(),
                self.get_affiliation()
            );
            emp.set_affiliation(self.get_affiliation());
            debug!("changed emp member={:?}", emp.affiliation());
            self.dao().update(emp).run(&mut ctx)
        })
    }
}
