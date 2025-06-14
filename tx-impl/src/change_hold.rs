use anyhow;
use log::{debug, trace};

use abstract_tx::{ChangeEmployee, UsecaseError};
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use payroll_factory::HoldMethodFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeHold トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeHoldTx<T, F>
where
    T: EmployeeDao,
{
    id: EmployeeId,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeHoldTx<T, F>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, dao: T, payroll_factory: F) -> Self {
        Self {
            id,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeHoldTx<T, F>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeEmployee for ChangeHoldTx<T, F>
where
    T: EmployeeDao,
    F: HoldMethodFactory,
{
    fn run_tx<'a, G, R>(&'a self, f: G) -> Result<R, UsecaseError>
    where
        G: FnOnce(Self::Ctx<'a>) -> Result<R, DaoError>,
    {
        trace!("run_tx called");
        // 今は DB しかないのでサービスレベルトランザクションが DB のトランザクションと同一視されている
        self.dao()
            .run_tx(f)
            .map_err(UsecaseError::ChangeEmployeeFailed)
    }

    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError> {
        trace!("change called");
        emp.set_method(self.payroll_factory.mk_method());
        debug!("method changed: {:?}", emp.method());
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeHoldTx<T, F>
where
    T: EmployeeDao,
    F: HoldMethodFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("execute called");
        ChangeEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
