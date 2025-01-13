use chrono::NaiveDate;
use dyn_clone::DynClone;
use std::{any::Any, cell::RefCell, fmt::Debug, ops::RangeInclusive, rc::Rc};

pub type EmpId = u32;
pub type MemberId = u32;

#[derive(Debug, Clone)]
pub struct Emp {
    id: EmpId,
    name: String,
    address: String,

    classification: Rc<RefCell<dyn PaymentClassification>>,
    schedule: Rc<RefCell<dyn PaymentSchedule>>,
    method: Rc<RefCell<dyn PaymentMethod>>,
    affiliation: Rc<RefCell<dyn Affiliation>>,
}

impl Emp {
    pub fn new(
        id: EmpId,
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

    pub fn id(&self) -> EmpId {
        self.id
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
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
    pub fn set_gross_pay(&mut self, gross_pay: f32) {
        self.gross_pay = gross_pay;
    }
    pub fn set_deductions(&mut self, deductions: f32) {
        self.deductions = deductions;
    }
    pub fn set_net_pay(&mut self, net_pay: f32) {
        self.net_pay = net_pay;
    }
}

pub trait PaymentClassification: Debug + DynClone {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn calculate_pay(&self, pc: &Paycheck) -> f32;
}
dyn_clone::clone_trait_object!(PaymentClassification);

pub trait PaymentSchedule: Debug + DynClone {
    fn is_pay_date(&self, date: NaiveDate) -> bool;
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate>;
}
dyn_clone::clone_trait_object!(PaymentSchedule);

pub trait PaymentMethod: Debug + DynClone {
    // TODO: return type
    fn pay(&self, pc: &Paycheck);
}
dyn_clone::clone_trait_object!(PaymentMethod);

pub trait Affiliation: Debug + DynClone {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn calculate_deductions(&self, pc: &Paycheck) -> f32;
}

#[derive(Debug, Clone)]
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
