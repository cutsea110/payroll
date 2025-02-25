use chrono::NaiveDate;
use dyn_clone::DynClone;
use log::{debug, trace};
use std::{any::Any, cell::RefCell, fmt::Debug, ops::RangeInclusive, rc::Rc};

mod types;
pub use types::*;

#[derive(Debug, Clone)]
pub struct Employee {
    id: EmployeeId,
    name: String,
    address: String,

    classification: Rc<RefCell<dyn PaymentClassification>>,
    schedule: Rc<RefCell<dyn PaymentSchedule>>,
    method: Rc<RefCell<dyn PaymentMethod>>,
    affiliation: Rc<RefCell<dyn Affiliation>>,
}

impl Employee {
    pub fn new(
        id: EmployeeId,
        name: &str,
        address: &str,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
        method: Rc<RefCell<dyn PaymentMethod>>,
        affiliation: Rc<RefCell<dyn Affiliation>>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            address: address.to_string(),
            classification,
            schedule,
            method,
            affiliation,
        }
    }

    pub fn id(&self) -> EmployeeId {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::clone(&self.classification)
    }
    pub fn schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        Rc::clone(&self.schedule)
    }
    pub fn method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        Rc::clone(&self.method)
    }
    pub fn affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
        Rc::clone(&self.affiliation)
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }
    pub fn set_classification(&mut self, classification: Rc<RefCell<dyn PaymentClassification>>) {
        self.classification = classification;
    }
    pub fn set_schedule(&mut self, schedule: Rc<RefCell<dyn PaymentSchedule>>) {
        self.schedule = schedule;
    }
    pub fn set_method(&mut self, method: Rc<RefCell<dyn PaymentMethod>>) {
        self.method = method;
    }
    pub fn set_affiliation(&mut self, affiliation: Rc<RefCell<dyn Affiliation>>) {
        self.affiliation = affiliation;
    }
    pub fn is_pay_date(&self, date: NaiveDate) -> bool {
        self.schedule.borrow().is_pay_date(date)
    }
    pub fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        self.schedule.borrow().get_pay_period(pay_date)
    }
    pub fn payday(&self, pc: &mut Paycheck) {
        trace!("Employee::payday");
        let gross_pay = self.classification.borrow().calculate_pay(pc);
        debug!("gross_pay: {}", gross_pay);
        let deductions = self.affiliation.borrow().calculate_deductions(pc);
        debug!("deductions: {}", deductions);
        let net_pay = gross_pay - deductions;
        debug!("net_pay: {}", net_pay);
        pc.set_gross_pay(gross_pay);
        pc.set_deductions(deductions);
        pc.set_net_pay(net_pay);
        debug!("updated paycheck: {:?}", pc);
        self.method.borrow().pay(pc);
    }
}

#[derive(Debug, Clone)]
pub struct Paycheck {
    period: RangeInclusive<NaiveDate>,

    gross_pay: f32,
    deductions: f32,
    net_pay: f32,
}
impl Paycheck {
    pub fn new(period: RangeInclusive<NaiveDate>) -> Self {
        Self {
            period,
            gross_pay: 0.0,
            deductions: 0.0,
            net_pay: 0.0,
        }
    }
    pub fn get_pay_period(&self) -> RangeInclusive<NaiveDate> {
        self.period.clone()
    }
    pub fn gross_pay(&self) -> f32 {
        self.gross_pay
    }
    pub fn deductions(&self) -> f32 {
        self.deductions
    }
    pub fn net_pay(&self) -> f32 {
        self.net_pay
    }
    pub fn set_gross_pay(&mut self, gross_pay: f32) {
        self.gross_pay = gross_pay;
    }
    pub fn set_deductions(&mut self, deductions: f32) {
        self.deductions = deductions;
    }
    pub fn set_net_pay(&mut self, net_pay: f32) {
        self.net_pay = net_pay;
    }
    pub fn is_pay_date(&self, pay_date: NaiveDate) -> bool {
        self.period.contains(&pay_date)
    }
}

pub trait PaymentClassification: Debug + DynClone {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn calculate_pay(&self, pc: &Paycheck) -> f32;
}
dyn_clone::clone_trait_object!(PaymentClassification);

pub trait PaymentSchedule: Debug + DynClone {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn is_pay_date(&self, date: NaiveDate) -> bool;
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate>;
}
dyn_clone::clone_trait_object!(PaymentSchedule);

pub trait PaymentMethod: Debug + DynClone {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    // TODO: return type
    fn pay(&self, pc: &Paycheck);
}
dyn_clone::clone_trait_object!(PaymentMethod);

pub trait Affiliation: Debug + DynClone {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn calculate_deductions(&self, pc: &Paycheck) -> f32;
}
dyn_clone::clone_trait_object!(Affiliation);

#[derive(Debug, Clone, PartialEq)]
pub struct NoAffiliation;
impl Affiliation for NoAffiliation {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_deductions(&self, _pc: &Paycheck) -> f32 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_service_charge() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
        );
        let aff = NoAffiliation;
        let deductions = aff.calculate_deductions(&pc);
        assert_eq!(deductions, 0.0);
    }
}
