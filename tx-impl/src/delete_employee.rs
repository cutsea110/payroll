use anyhow;
use log::trace;

use abstract_tx::{DeleteEmployee, UsecaseError};
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
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
    fn run_tx<'a, G, R>(&'a self, f: G) -> Result<R, UsecaseError>
    where
        G: FnOnce(Self::Ctx<'a>) -> Result<R, DaoError>,
    {
        trace!("run_tx called");
        // 今は DB しかないのでサービスレベルトランザクションが DB のトランザクションと同一視されている
        self.dao()
            .run_tx(f)
            .map_err(UsecaseError::AddEmployeeFailed)
    }

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
        trace!("execute called");
        DeleteEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
