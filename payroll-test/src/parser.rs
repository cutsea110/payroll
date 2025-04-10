use log::{debug, trace};
use parsec_rs::{float32, keyword, spaces, uint32, Parser};

use crate::Verify;

#[derive(Debug, Clone, PartialEq)]
pub enum TxType {
    Payday,
    Verify,
    Other,
}
pub fn tx_type(line: &str) -> TxType {
    trace!("tx_type called");
    if is_verify(line) {
        TxType::Verify
    } else if is_payday(line) {
        TxType::Payday
    } else {
        TxType::Other
    }
}
#[cfg(test)]
mod tx_type_test {
    use super::*;

    #[test]
    fn test_tx_type() {
        assert_eq!(
            tx_type("Verify Paycheck EmpId 123 GrossPay 1000.00"),
            TxType::Verify
        );
        assert_eq!(tx_type("Payday"), TxType::Payday);
        assert_eq!(tx_type("Some other line"), TxType::Other);
    }
}

pub fn is_verify(line: &str) -> bool {
    trace!("is_verify called");
    spaces().skip(keyword("Verify")).parse(line).is_ok()
}
#[cfg(test)]
mod is_verify_test {
    use super::*;

    #[test]
    fn test_is_verify() {
        assert!(is_verify("Verify Paycheck EmpId 123 GrossPay 1000.00"));
        assert!(!is_verify("Payday"));
        assert!(!is_verify("Some other line"));
    }
}

pub fn is_payday(line: &str) -> bool {
    trace!("is_payday called");
    spaces().skip(keyword("Payday")).parse(line).is_ok()
}
#[cfg(test)]
mod is_payday_test {
    use super::*;

    #[test]
    fn test_is_payday() {
        assert!(is_payday("Payday"));
        assert!(!is_payday("Verify Paycheck EmpId 123 GrossPay 1000.00"));
        assert!(!is_payday("Some other line"));
    }
}

pub fn read_verify(line_num: usize, line: &str) -> Result<Verify, String> {
    trace!("read_verify called");
    verify(line_num, line.to_string())
        .parse(line)
        .map(|(v, _)| v)
        .map_err(|e| {
            format!(
                "Parse error at L{}:\n{}\n{}^ {}",
                line_num,
                line,
                " ".repeat(e.position()),
                e.expected().join(" or ") + " expected"
            )
        })
}
#[cfg(test)]
mod read_verify_test {
    use super::*;

    #[test]
    fn test_read_verify() {
        let line = "Verify Paycheck EmpId 123 GrossPay 1000.00";
        let result = read_verify(1, line);
        assert!(result.is_ok());
        let verify = result.unwrap();
        assert_eq!(
            verify,
            Verify::GrossPay {
                emp_id: 123,
                gross_pay: 1000.0,
                loc: (1, line.to_string())
            }
        );
    }

    #[test]
    fn test_read_verify_invalid() {
        let line = "Verify Paycheck EmpId 123 InvalidField 1000.00";
        let result = read_verify(1, line);
        assert!(result.is_err());
    }
}

fn employee_id() -> impl Parser<Item = u32> {
    uint32()
        .map(Into::into)
        .with(spaces())
        .label("<employee_id>".into())
}
#[cfg(test)]
mod employee_id_test {
    use super::*;

    #[test]
    fn test_employee_id() {
        let line = "123";
        let result = employee_id().parse(line);
        assert!(result.is_ok());
        let (id, _) = result.unwrap();
        assert_eq!(id, 123);
    }

    #[test]
    fn test_employee_id_invalid() {
        let line = "abc";
        let result = employee_id().parse(line);
        assert!(result.is_err());
    }
}

fn amount() -> impl Parser<Item = f32> {
    float32().with(spaces()).label("<amount>".into())
}
#[cfg(test)]
mod amount_test {
    use super::*;

    #[test]
    fn test_amount() {
        let line = "1000.00";
        let result = amount().parse(line);
        assert!(result.is_ok());
        let (amt, _) = result.unwrap();
        assert_eq!(amt, 1000.0);
    }

    #[test]
    fn test_amount_invalid() {
        let line = "abc";
        let result = amount().parse(line);
        assert!(result.is_err());
    }
}

fn verify(line_num: usize, line: String) -> impl Parser<Item = Verify> {
    let verify = keyword("Verify").with(spaces()).label("Verify".into());
    let paycheck = keyword("Paycheck").with(spaces()).label("Paycheck".into());
    let empid = keyword("EmpId").with(spaces()).label("EmpId".into());
    let field = keyword("GrossPay")
        .label("GrossPay".into())
        .or(keyword("Deductions").label("Deductions".into()))
        .or(keyword("NetPay").label("NetPay".into()));

    verify
        .skip(paycheck)
        .skip(empid)
        .skip(employee_id())
        .join(field.with(spaces()))
        .join(amount())
        .map(move |((emp_id, key), amount)| {
            debug!("key: {}, emp_id: {}, amount: {}", key, emp_id, amount);
            match key {
                "GrossPay" => Verify::GrossPay {
                    emp_id,
                    gross_pay: amount,
                    loc: (line_num, line),
                },
                "Deductions" => Verify::Deductions {
                    emp_id,
                    deductions: amount,
                    loc: (line_num, line),
                },
                "NetPay" => Verify::NetPay {
                    emp_id,
                    net_pay: amount,
                    loc: (line_num, line),
                },
                _ => panic!("unexpected key: {}", key),
            }
        })
}
#[cfg(test)]
mod verify_test {
    use super::*;

    #[test]
    fn test_verify() {
        let line = "Verify Paycheck EmpId 123 GrossPay 1000.00";
        let result = verify(1, line.to_string()).parse(line);
        assert!(result.is_ok());
        let (verify, _) = result.unwrap();
        assert_eq!(
            verify,
            Verify::GrossPay {
                emp_id: 123,
                gross_pay: 1000.0,
                loc: (1, line.to_string())
            }
        );
    }

    #[test]
    fn test_verify_invalid() {
        let line = "Verify Paycheck EmpId 123 InvalidField 1000.00";
        let result = verify(1, line.to_string()).parse(line);
        assert!(result.is_err());
    }
}
