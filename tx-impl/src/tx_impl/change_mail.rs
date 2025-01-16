use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChgMethod;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentMethod};
use payroll_impl::MailMethod;
use tx_app::{Response, Transaction};

// ユースケース: ChangeMail トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeMailTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    address: String,

    dao: T,
}
impl<T> ChangeMailTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, address: &str, dao: T) -> Self {
        Self {
            id,
            address: address.to_string(),
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for ChangeMailTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChgMethod for ChangeMailTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        Rc::new(RefCell::new(MailMethod::new(&self.address)))
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeMailTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeMailTx::execute called");
        ChgMethod::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
