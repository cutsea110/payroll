use crate::parser;
use log::{debug, trace};
use serde::Deserialize;

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
    pub fn parse(line_no: usize, line: &str) -> Result<Self, String> {
        trace!("parse called");
        debug!("parse: L{}, line={}", line_no, line);
        parser::read_verify(line_no, line)
    }
}
