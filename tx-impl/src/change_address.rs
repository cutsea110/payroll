use anyhow;
use log::trace;

use abstract_tx::{ChangeEmployee, UsecaseError};
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use tx_app::{Response, Transaction};

// ユースケース: ChangeEmployeeAddress トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeEmployeeAddressTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    new_address: String,

    dao: T,
}
impl<T> ChangeEmployeeAddressTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, new_address: &str, dao: T) -> Self {
        Self {
            id,
            new_address: new_address.to_string(),
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for ChangeEmployeeAddressTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeEmployee for ChangeEmployeeAddressTx<T>
where
    T: EmployeeDao,
{
    fn run_tx<'a, G, R>(&'a self, f: G) -> Result<R, UsecaseError>
    where
        G: FnOnce(Self::Ctx<'a>) -> Result<R, DaoError>,
    {
        trace!("run_tx called");
        // 今は DB しかないのでサービスレベルトランザクションが DB のトランザクションと同一視されている
        // TODO: ただしここはサービスレベルではなくユースケースレベルであるからサービスレベルに移動したい
        self.dao()
            .run_tx(f)
            .map_err(UsecaseError::ChangeEmployeeFailed)
    }

    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError> {
        trace!("change called");
        emp.set_address(&self.new_address);
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeEmployeeAddressTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("execute called");
        ChangeEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
