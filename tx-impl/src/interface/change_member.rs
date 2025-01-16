use log::{debug, trace};
use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Affiliation, EmployeeId, MemberId};

// ユースケース: ChangeMember トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeMember: HaveEmployeeDao {
    fn get_member_id(&self) -> MemberId;
    fn get_emp_id(&self) -> EmployeeId;
    fn get_dues(&self) -> f32;
    fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;

    fn record_membership<'a>(&self, ctx: &mut Self::Ctx<'a>) -> Result<(), DaoError> {
        trace!("record_membership called");
        self.dao()
            .add_union_member(self.get_member_id(), self.get_emp_id())
            .run(ctx)
    }

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("ChangeMember::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("ChangeMember::run_tx called");
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
            .map_err(UsecaseError::ChangeMemberFailed)
    }
}
