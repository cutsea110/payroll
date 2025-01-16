use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChgMethod;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentMethod};
use payroll_impl::HoldMethod;
use tx_app::{Response, Transaction};

// ユースケース: ChangeHold トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeHoldTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,

    dao: T,
}
impl<T> ChangeHoldTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, dao: T) -> Self {
        Self { id, dao }
    }
}

impl<T> HaveEmployeeDao for ChangeHoldTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChgMethod for ChangeHoldTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        Rc::new(RefCell::new(HoldMethod))
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeHoldTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeHoldTx::execute called");
        ChgMethod::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
