use log::{debug, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;

// ユースケース: DeleteEmployee トランザクション(抽象レベルのビジネスロジック)
pub trait DeleteEmployee: HaveEmployeeDao {
    // TODO: このレイヤはユースケースで、本来 run_tx はサービスレベルにあるべき
    // そしてサービスレベルの実装は EmployeeDao トレイトではなく具体的な Db 構造体を相手に run_tx を実装するべき
    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, UsecaseError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

    fn get_id(&self) -> EmployeeId;

    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("execute called");
        self.run_tx(|mut ctx| {
            trace!("run_tx called");
            let emp_id = self.get_id();
            debug!("execute: emp_id={}", emp_id);
            self.dao().delete(emp_id).run(&mut ctx)
        })
    }
}
