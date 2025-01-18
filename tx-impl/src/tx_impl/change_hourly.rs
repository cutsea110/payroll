use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChangeClassification;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentClassification};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeHourly トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeHourlyTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    id: EmployeeId,
    hourly_rate: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeHourlyTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    pub fn new(id: EmployeeId, hourly_rate: f32, dao: T, payroll_factory: F) -> Self {
        Self {
            id,
            hourly_rate,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeHourlyTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeClassification for ChangeHourlyTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        self.payroll_factory
            .mk_hourly_classification(self.hourly_rate)
    }
    fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
        self.payroll_factory.mk_weekly_schedule()
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeHourlyTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeHourlyTx::execute called");
        ChangeClassification::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
