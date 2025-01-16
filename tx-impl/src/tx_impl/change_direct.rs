use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChgMethod;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentMethod};
use payroll_impl::DirectMethod;
use tx_app::{Response, Transaction};

// ユースケース: ChangeDirect トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeDirectTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    bank: String,
    account: String,

    dao: T,
}
impl<T> ChangeDirectTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, bank: &str, account: &str, dao: T) -> Self {
        Self {
            id,
            bank: bank.to_string(),
            account: account.to_string(),
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for ChangeDirectTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChgMethod for ChangeDirectTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        Rc::new(RefCell::new(DirectMethod::new(&self.bank, &self.account)))
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeDirectTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeDirectTx::execute called");
        ChgMethod::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
