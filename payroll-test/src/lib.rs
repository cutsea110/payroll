mod types {
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
        pub fn parse(line: &str) -> Result<Self, &str> {
            trace!("parse called");
            debug!("parse: line={}", line);
            super::read_verify(line)
        }
    }
}
pub use types::*;

mod parser {
    use super::Verify;
    use log::{debug, trace};
    use parsec_rs::{float32, keyword, spaces, uint32, Parser};

    pub fn read_verify(line: &str) -> Result<Verify, &str> {
        trace!("read_verify called");
        verify()
            .parse(line)
            .map(|(v, _)| v)
            .map_err(|_| "parse error")
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

    fn verify() -> impl Parser<Item = Verify> {
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
            .map(|((emp_id, key), amount)| {
                debug!("key: {}, emp_id: {}, amount: {}", key, emp_id, amount);
                match key {
                    "GrossPay" => Verify::GrossPay {
                        emp_id,
                        gross_pay: amount,
                    },
                    "Deductions" => Verify::Deductions {
                        emp_id,
                        deductions: amount,
                    },
                    "NetPay" => Verify::NetPay {
                        emp_id,
                        net_pay: amount,
                    },
                    _ => panic!("unexpected key: {}", key),
                }
            })
    }
}
pub use parser::*;
