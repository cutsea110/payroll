use crate::parser;
use log::{debug, trace};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Paycheck {
    pub emp_id: u32,
    pub gross_pay: f32,
    pub deductions: f32,
    pub net_pay: f32,
}

type Location = (usize, String);

#[derive(Debug, Clone, PartialEq)]
pub enum Verify {
    GrossPay {
        emp_id: u32,
        gross_pay: f32,
        loc: Location,
    },
    Deductions {
        emp_id: u32,
        deductions: f32,
        loc: Location,
    },
    NetPay {
        emp_id: u32,
        net_pay: f32,
        loc: Location,
    },
}
impl Verify {
    pub fn parse(line_no: usize, line: &str) -> Result<Self, String> {
        trace!("parse called");
        debug!("parse: L{}, line={}", line_no, line);
        parser::read_verify(line_no, line)
    }
    fn emp_id(&self) -> u32 {
        match self {
            Verify::GrossPay { emp_id, .. } => *emp_id,
            Verify::Deductions { emp_id, .. } => *emp_id,
            Verify::NetPay { emp_id, .. } => *emp_id,
        }
    }
    fn line_num(&self) -> usize {
        match self {
            Verify::GrossPay { loc, .. } => loc.0,
            Verify::Deductions { loc, .. } => loc.0,
            Verify::NetPay { loc, .. } => loc.0,
        }
    }
    fn line(&self) -> &str {
        match self {
            Verify::GrossPay { loc, .. } => &loc.1,
            Verify::Deductions { loc, .. } => &loc.1,
            Verify::NetPay { loc, .. } => &loc.1,
        }
    }
    pub fn verify(&self, outputs: &HashMap<u32, Paycheck>) -> bool {
        assert!(
            outputs.contains_key(&self.emp_id()),
            "not found emp_id: {}",
            self.emp_id()
        );
        let actual = outputs.get(&self.emp_id()).expect("get paycheck");
        let info = format!("L{}: '{}'", self.line_num(), self.line());
        match self {
            Verify::GrossPay { gross_pay, .. } => {
                assert_eq!(actual.gross_pay, *gross_pay, "gross_pay mismatch {}", info);
            }
            Verify::Deductions { deductions, .. } => {
                assert_eq!(
                    actual.deductions, *deductions,
                    "deduction mismatch {}",
                    info
                );
            }
            Verify::NetPay { net_pay, .. } => {
                assert_eq!(actual.net_pay, *net_pay, "net_pay mismatch {}", info);
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_for_empty_test() {
        let verify = Verify::GrossPay {
            emp_id: 1234,
            gross_pay: 1000.0,
            loc: (1, "L1".to_string()),
        };
        let outputs = HashMap::new();
        assert_ne!(verify.verify(&outputs), false);
    }

    #[test]
    fn test_verify_gross_pay() {
        let verify = Verify::GrossPay {
            emp_id: 1234,
            gross_pay: 1000.0,
            loc: (1, "L1".to_string()),
        };
        let mut outputs = HashMap::new();
        outputs.insert(
            1234,
            Paycheck {
                emp_id: 1234,
                gross_pay: 1000.0,
                deductions: 200.0,
                net_pay: 800.0,
            },
        );
        assert_eq!(verify.verify(&outputs), true);
    }

    #[test]
    fn test_verify_deductions() {
        let verify = Verify::Deductions {
            emp_id: 1234,
            deductions: 200.0,
            loc: (1, "L1".to_string()),
        };
        let mut outputs = HashMap::new();
        outputs.insert(
            1234,
            Paycheck {
                emp_id: 1234,
                gross_pay: 1000.0,
                deductions: 200.0,
                net_pay: 800.0,
            },
        );
        assert_eq!(verify.verify(&outputs), true);
    }

    #[test]
    fn test_verify_net_pay() {
        let verify = Verify::NetPay {
            emp_id: 1234,
            net_pay: 800.0,
            loc: (1, "L1".to_string()),
        };
        let mut outputs = HashMap::new();
        outputs.insert(
            1234,
            Paycheck {
                emp_id: 1234,
                gross_pay: 1000.0,
                deductions: 200.0,
                net_pay: 800.0,
            },
        );
        assert_eq!(verify.verify(&outputs), true);
    }
}
