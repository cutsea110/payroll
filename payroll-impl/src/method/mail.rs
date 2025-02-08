use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone)]
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
    fn pay(&self, pc: &Paycheck) {
        println!("MailMethod to {}: {:#?}", self.address, pc);
    }
}
