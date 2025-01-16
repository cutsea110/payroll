use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::AddEmployee;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{
    Affiliation, EmployeeId, NoAffiliation, PaymentClassification, PaymentMethod, PaymentSchedule,
};
use payroll_impl::{HoldMethod, HourlyClassification, WeeklySchedule};
use tx_app::{Response, Transaction};

// ユースケース: AddHourlyEmployee トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddHourlyEmployeeTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    name: String,
    address: String,
    hourly_rate: f32,

    dao: T,
}
impl<T> AddHourlyEmployeeTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, name: &str, address: &str, hourly_rate: f32, dao: T) -> Self {
        Self {
            id,
            name: name.to_string(),
            address: address.to_string(),
            hourly_rate,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for AddHourlyEmployeeTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> AddEmployee for AddHourlyEmployeeTx<T>
where
    T: EmployeeDao,
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
        Rc::new(RefCell::new(HourlyClassification::new(self.hourly_rate)))
    }
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        Rc::new(RefCell::new(WeeklySchedule))
    }
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        Rc::new(RefCell::new(HoldMethod))
    }
    fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
        Rc::new(RefCell::new(NoAffiliation))
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddHourlyEmployeeTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("AddHourlyEmployeeTx::execute called");
        AddEmployee::execute(self)
            .map(|_| Response::EmployeeId(self.id))
            .map_err(Into::into)
    }
}
