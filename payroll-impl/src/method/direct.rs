use std::any::Any;

use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone, PartialEq)]
pub struct DirectMethod {
    bank: String,
    account: String,
}
impl DirectMethod {
    pub fn new(bank: &str, account: &str) -> Self {
        Self {
            bank: bank.to_string(),
            account: account.to_string(),
        }
    }
}
impl PaymentMethod for DirectMethod {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn pay(&self, pc: &Paycheck) {
        println!("DirectMethod to {} {}: {:#?}", self.bank, self.account, pc);
    }
}
