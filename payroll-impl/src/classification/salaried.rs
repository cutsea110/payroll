use log::{debug, trace};
use std::any::Any;

use payroll_domain::{Paycheck, PaymentClassification};

#[derive(Debug, Clone)]
pub struct SalariedClassification {
    salary: f32,
}
impl SalariedClassification {
    pub fn new(salary: f32) -> Self {
        Self { salary }
    }
}
impl PaymentClassification for SalariedClassification {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, pc: &Paycheck) -> f32 {
        trace!("SalariedClassification::calculate_pay called");
        let pay_period = pc.get_pay_period();
        debug!("pay_period: {} - {}", pay_period.start(), pay_period.end());
        self.salary
    }
}
