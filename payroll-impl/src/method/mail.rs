use std::any::Any;

use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone, PartialEq)]
pub struct MailMethod {
    address: String,
}
impl MailMethod {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }
}
impl PaymentMethod for MailMethod {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn pay(&self, pc: &Paycheck) {
        println!("MailMethod to {}: {:#?}", self.address, pc);
    }
}
