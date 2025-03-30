use log::{debug, trace};
use serde::{Deserialize, Serialize};
use serde_json;
use std::any::Any;

use payroll_domain::{EmployeeId, Paycheck, PaymentMethod};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MailPay {
    emp_id: u32,
    name: String,

    address: String,

    gross_pay: f32,
    deductions: f32,
    net_pay: f32,
}

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
    fn pay(&self, emp_id: EmployeeId, name: &str, pc: &Paycheck) {
        trace!("MailMethod::pay called");
        let mail_pay = MailPay {
            emp_id: emp_id.into(),
            name: name.to_string(),

            address: self.address.clone(),

            gross_pay: pc.gross_pay(),
            deductions: pc.deductions(),
            net_pay: pc.net_pay(),
        };
        let json = serde_json::to_string(&mail_pay).expect("serialize MailPay as JSON");
        debug!("MailMethod::pay: {}", json);
        println!("{}", json);
    }
}
