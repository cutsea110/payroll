use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChangeClassification;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, PaymentClassification};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeSalaried トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    id: EmployeeId,
    salary: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    pub fn new(id: EmployeeId, salary: f32, dao: T, payroll_factory: F) -> Self {
        Self {
            id,
            salary,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeClassification for ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        self.payroll_factory.mk_salaried_classification(self.salary)
    }
    fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
        self.payroll_factory.mk_monthly_schedule()
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeSalariedTx::execute called");
        ChangeClassification::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
