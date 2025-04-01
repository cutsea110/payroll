use log::{debug, trace};
use serde::{Deserialize, Serialize};
use serde_json;
use std::any::Any;

use payroll_domain::{EmployeeId, Paycheck, PaymentMethod};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectPay {
    emp_id: u32,

    bank: String,
    account: String,

    gross_pay: f32,
    deductions: f32,
    net_pay: f32,
}

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
    fn pay(&self, emp_id: EmployeeId, pc: &Paycheck) {
        trace!("DirectMethod::pay called");
        let direct_pay = DirectPay {
            emp_id: emp_id.into(),

            bank: self.bank.clone(),
            account: self.account.clone(),

            gross_pay: pc.gross_pay(),
            deductions: pc.deductions(),
            net_pay: pc.net_pay(),
        };
        let json = serde_json::to_string(&direct_pay).expect("serialize DirectPay as JSON");
        debug!("DirectMethod::pay: {}", json);
        println!("{}", json);
    }
}
