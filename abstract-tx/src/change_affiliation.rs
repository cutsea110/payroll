use log::{debug, trace};
use std::sync::{Arc, Mutex};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Affiliation, MemberId};

// ユースケース: ChangeAffiliation トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeAffiliation: HaveEmployeeDao {
    fn get_member_id(&self) -> MemberId;
    fn change(&self, aff: Arc<Mutex<dyn Affiliation>>) -> Result<(), DaoError>;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("run_tx called");
                let emp_id = self
                    .dao()
                    .find_union_member(self.get_member_id())
                    .run(&mut ctx)?;
                debug!("found emp_id={}", emp_id);
                let emp = self.dao().fetch(emp_id).run(&mut ctx)?;
                debug!("changing emp={:?}", emp);
                self.change(emp.affiliation())?;
                debug!("changed emp={:?}", emp);
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::ChangeAffiliationFailed)
    }
}
