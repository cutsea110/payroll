use log::{debug, trace};
use std::{cell::RefCell, rc::Rc};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, NoAffiliation};
use payroll_impl::UnionAffiliation;

// ユースケース: ChangeNoMember トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeNoMember: HaveEmployeeDao {
    fn get_emp_id(&self) -> EmployeeId;

    fn record_membership<'a>(&self, ctx: &mut Self::Ctx<'a>) -> Result<(), DaoError> {
        trace!("record_membership called");
        let emp = self.dao().fetch(self.get_emp_id()).run(ctx)?;
        let member_id = emp
            .affiliation()
            .borrow()
            .as_any()
            .downcast_ref::<UnionAffiliation>()
            .ok_or(DaoError::UnexpectedError("didn't union affiliation".into()))?
            .member_id();

        self.dao().remove_union_member(member_id).run(ctx)
    }

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("ChangeNoMember::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("ChangeNoMember::run_tx called");
                self.record_membership(&mut ctx)?;

                let mut emp = self.dao().fetch(self.get_emp_id()).run(&mut ctx)?;
                debug!(
                    "changing emp member: {:?} -> NoAffiliation",
                    emp.affiliation()
                );
                emp.set_affiliation(Rc::new(RefCell::new(NoAffiliation)));
                debug!("changed emp member={:?}", emp.affiliation());
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::ChangeNoMemberFailed)
    }
}
