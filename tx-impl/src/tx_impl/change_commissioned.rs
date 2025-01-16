use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChangeClassification;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentClassification};
use payroll_impl::{BiweeklySchedule, CommissionedClassification};
use tx_app::{Response, Transaction};

// ユースケース: ChangeCommissioned トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeCommissionedTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    salary: f32,
    commission_rate: f32,

    dao: T,
}
impl<T> ChangeCommissionedTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, salary: f32, commission_rate: f32, dao: T) -> Self {
        Self {
            id,
            salary,
            commission_rate,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for ChangeCommissionedTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeClassification for ChangeCommissionedTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::new(RefCell::new(CommissionedClassification::new(
            self.salary,
            self.commission_rate,
        )))
    }
    fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
        Rc::new(RefCell::new(BiweeklySchedule))
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeCommissionedTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeCommissionedTx::execute called");
        ChangeClassification::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
