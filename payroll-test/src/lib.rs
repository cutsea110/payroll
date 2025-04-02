use log::{debug, trace};
use serde::Deserialize;

mod parser;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Paycheck {
    pub emp_id: u32,
    pub gross_pay: f32,
    pub deductions: f32,
    pub net_pay: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Verify {
    GrossPay { emp_id: u32, gross_pay: f32 },
    Deductions { emp_id: u32, deductions: f32 },
    NetPay { emp_id: u32, net_pay: f32 },
}
impl Verify {
    pub fn parse(line: &str) -> Result<Self, &str> {
        trace!("parse called");
        debug!("parse: line={}", line);
        parser::read_verify(line)
    }
    pub fn is_verify(line: &str) -> bool {
        trace!("is_verify called");
        debug!("is_verify: line={}", line);
        parser::is_verify(line)
    }
}
