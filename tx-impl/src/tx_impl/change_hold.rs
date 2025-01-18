use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChgMethod;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentMethod};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeHold トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeHoldTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    id: EmployeeId,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeHoldTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
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
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChgMethod for ChangeHoldTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        self.payroll_factory.mk_hold_method()
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeHoldTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeHoldTx::execute called");
        ChgMethod::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
