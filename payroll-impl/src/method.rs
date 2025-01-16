use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone)]
pub struct HoldMethod;
impl PaymentMethod for HoldMethod {
    fn pay(&self, pc: &Paycheck) {
        println!("HoldMethod.pay: {:?}", pc);
    }
}

#[derive(Debug, Clone)]
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
    fn pay(&self, pc: &Paycheck) {
        println!("DirectMethod to {} {}: {:#?}", self.bank, self.account, pc);
    }
}

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
