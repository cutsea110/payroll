use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::AddEmployee;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{
    Affiliation, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: AddHourlyEmployee トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    id: EmployeeId,
    name: String,
    address: String,
    hourly_rate: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    pub fn new(
        id: EmployeeId,
        name: &str,
        address: &str,
        hourly_rate: f32,
        dao: T,
        payroll_factory: F,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            address: address.to_string(),
            hourly_rate,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> AddEmployee for AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        self.payroll_factory
            .mk_hourly_classification(self.hourly_rate)
    }
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        self.payroll_factory.mk_weekly_schedule()
    }
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        self.payroll_factory.mk_hold_method()
    }
    fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
        self.payroll_factory.mk_no_affiliation()
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("AddHourlyEmployeeTx::execute called");
        AddEmployee::execute(self)
            .map(|_| Response::EmployeeId(self.id))
            .map_err(Into::into)
    }
}
