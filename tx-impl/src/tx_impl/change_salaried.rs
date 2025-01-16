use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChangeClassification;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentClassification};
use payroll_impl::{MonthlySchedule, SalariedClassification};
use tx_app::{Response, Transaction};

// ユースケース: ChangeSalaried トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeSalariedTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    salary: f32,

    dao: T,
}
impl<T> ChangeSalariedTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, salary: f32, dao: T) -> Self {
        Self { id, salary, dao }
    }
}

impl<T> HaveEmployeeDao for ChangeSalariedTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeClassification for ChangeSalariedTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::new(RefCell::new(SalariedClassification::new(self.salary)))
    }
    fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
        Rc::new(RefCell::new(MonthlySchedule))
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeSalariedTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeSalariedTx::execute called");
        ChangeClassification::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
