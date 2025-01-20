use anyhow;
use log::trace;

use abstract_tx::DeleteEmployee;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use tx_app::{Response, Transaction};

// ユースケース: DeleteEmployee トランザクションの実装 (struct)
#[derive(Debug)]
pub struct DeleteEmployeeTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    dao: T,
}
impl<T> DeleteEmployeeTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, dao: T) -> Self {
        Self { id, dao }
    }
}

impl<T> HaveEmployeeDao for DeleteEmployeeTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> DeleteEmployee for DeleteEmployeeTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
}
// 共通インターフェースの実装
impl<T> Transaction for DeleteEmployeeTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("DeleteEmployeeTx::execute called");
        DeleteEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
