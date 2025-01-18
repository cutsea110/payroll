use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChgMethod;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentMethod};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeMail トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeMailTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    id: EmployeeId,
    address: String,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeMailTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    pub fn new(id: EmployeeId, address: &str, dao: T, payroll_factory: F) -> Self {
        Self {
            id,
            address: address.to_string(),
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeMailTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChgMethod for ChangeMailTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        self.payroll_factory.mk_mail_method(&self.address)
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeMailTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeMailTx::execute called");
        ChgMethod::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
