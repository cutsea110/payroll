use chrono::NaiveDate;
use dyn_clone::DynClone;
use log::{debug, trace};
use std::{
    any::Any,
    fmt::Debug,
    ops::RangeInclusive,
    sync::{Arc, Mutex},
};

mod types;
pub use types::*;

#[derive(Debug, Clone)]
pub struct Employee {
    id: EmployeeId,
    name: String,
    address: String,

    classification: Arc<Mutex<dyn PaymentClassification>>,
    schedule: Arc<Mutex<dyn PaymentSchedule>>,
    method: Arc<Mutex<dyn PaymentMethod>>,
    affiliation: Arc<Mutex<dyn Affiliation>>,
}

impl Employee {
    pub fn new(
        id: EmployeeId,
        name: &str,
        address: &str,
        classification: Arc<Mutex<dyn PaymentClassification>>,
        schedule: Arc<Mutex<dyn PaymentSchedule>>,
        method: Arc<Mutex<dyn PaymentMethod>>,
        affiliation: Arc<Mutex<dyn Affiliation>>,
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
    pub fn classification(&self) -> Arc<Mutex<dyn PaymentClassification>> {
        Arc::clone(&self.classification)
    }
    pub fn schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>> {
        Arc::clone(&self.schedule)
    }
    pub fn method(&self) -> Arc<Mutex<dyn PaymentMethod>> {
        Arc::clone(&self.method)
    }
    pub fn affiliation(&self) -> Arc<Mutex<dyn Affiliation>> {
        Arc::clone(&self.affiliation)
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }
    pub fn set_classification(&mut self, classification: Arc<Mutex<dyn PaymentClassification>>) {
        self.classification = classification;
    }
    pub fn set_schedule(&mut self, schedule: Arc<Mutex<dyn PaymentSchedule>>) {
        self.schedule = schedule;
    }
    pub fn set_method(&mut self, method: Arc<Mutex<dyn PaymentMethod>>) {
        self.method = method;
    }
    pub fn set_affiliation(&mut self, affiliation: Arc<Mutex<dyn Affiliation>>) {
        self.affiliation = affiliation;
    }
    pub fn is_pay_date(&self, date: NaiveDate) -> bool {
        self.schedule.lock().unwrap().is_pay_date(date)
    }
    pub fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        self.schedule.lock().unwrap().get_pay_period(pay_date)
    }
    pub fn payday(&self, pc: &mut Paycheck) {
        trace!("payday called");
        let gross_pay = self.classification.lock().unwrap().calculate_pay(pc);
        debug!("gross_pay: {}", gross_pay);
        let deductions = self.affiliation.lock().unwrap().calculate_deductions(pc);
        debug!("deductions: {}", deductions);
        let net_pay = gross_pay - deductions;
        debug!("net_pay: {}", net_pay);
        pc.set_gross_pay(gross_pay);
        pc.set_deductions(deductions);
        pc.set_net_pay(net_pay);
        debug!("updated paycheck: {:?}", pc);
        self.method.lock().unwrap().pay(self.id, pc);
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
    fn pay(&self, emp_id: EmployeeId, pc: &Paycheck);
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
