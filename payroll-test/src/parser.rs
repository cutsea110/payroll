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

pub fn is_verify(line: &str) -> bool {
    trace!("is_verify called");
    spaces().skip(keyword("Verify")).parse(line).is_ok()
}

pub fn is_payday(line: &str) -> bool {
    trace!("is_payday called");
    spaces().skip(keyword("Payday")).parse(line).is_ok()
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

fn employee_id() -> impl Parser<Item = u32> {
    uint32()
        .map(Into::into)
        .with(spaces())
        .label("<employee_id>".into())
}

fn amount() -> impl Parser<Item = f32> {
    float32().with(spaces()).label("<amount>".into())
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
