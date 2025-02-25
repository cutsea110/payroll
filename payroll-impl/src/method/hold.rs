use std::any::Any;

use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone, PartialEq)]
pub struct HoldMethod;
impl PaymentMethod for HoldMethod {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn pay(&self, pc: &Paycheck) {
        println!("HoldMethod.pay: {:?}", pc);
    }
}
