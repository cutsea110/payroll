use log::{debug, trace};
use std::sync::{Arc, Mutex};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Affiliation, MemberId};

// ユースケース: ChangeAffiliation トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeAffiliation: HaveEmployeeDao {
    // TODO: このレイヤはユースケースで、本来 run_tx はサービスレベルにあるべき
    // そしてサービスレベルの実装は EmployeeDao トレイトではなく具体的な Db 構造体を相手に run_tx を実装するべき
    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, UsecaseError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

    fn get_member_id(&self) -> MemberId;
    fn change(&self, aff: Arc<Mutex<dyn Affiliation>>) -> Result<(), DaoError>;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("execute called");
        self.run_tx(|mut ctx| {
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
    }
}
