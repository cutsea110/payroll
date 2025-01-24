use log::trace;
use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Affiliation, MemberId};

// ユースケース: ChangeAffiliation トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeAffiliation: HaveEmployeeDao {
    fn get_member_id(&self) -> MemberId;
    fn change(&self, aff: Rc<RefCell<dyn Affiliation>>) -> Result<(), DaoError>;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("ChangeAffiliation::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("ChangeAffiliation::run_tx called");
                let emp_id = self
                    .dao()
                    .find_union_member(self.get_member_id())
                    .run(&mut ctx)?;
                let emp = self.dao().fetch(emp_id).run(&mut ctx)?;
                self.change(emp.affiliation())?;
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::ChangeAffiliationFailed)
    }
}
