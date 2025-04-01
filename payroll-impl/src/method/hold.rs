use log::{debug, trace};
use serde::{Deserialize, Serialize};
use serde_json;
use std::any::Any;

use payroll_domain::{EmployeeId, Paycheck, PaymentMethod};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoldPay {
    emp_id: u32,

    gross_pay: f32,
    deductions: f32,
    net_pay: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HoldMethod;
impl PaymentMethod for HoldMethod {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn pay(&self, emp_id: EmployeeId, pc: &Paycheck) {
        trace!("HoldMethod::pay called");
        let hold_pay = HoldPay {
            emp_id: emp_id.into(),

            gross_pay: pc.gross_pay(),
            deductions: pc.deductions(),
            net_pay: pc.net_pay(),
        };
        let json = serde_json::to_string(&hold_pay).expect("serialize HoldPay as JSON");
        debug!("HoldMethod::pay: {}", json);
        println!("{}", json);
    }
}
