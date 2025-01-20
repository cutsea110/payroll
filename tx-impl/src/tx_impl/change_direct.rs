use anyhow;
use log::trace;

use crate::ChangeEmployee;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeDirect トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeDirectTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    id: EmployeeId,
    bank: String,
    account: String,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeDirectTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    pub fn new(id: EmployeeId, bank: &str, account: &str, dao: T, payroll_factory: F) -> Self {
        Self {
            id,
            bank: bank.to_string(),
            account: account.to_string(),
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeDirectTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeEmployee for ChangeDirectTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) {
        emp.set_method(
            self.payroll_factory
                .mk_direct_method(&self.bank, &self.account),
        );
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeDirectTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeDirectTx::execute called");
        ChangeEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
