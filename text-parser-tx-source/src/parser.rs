use chrono::NaiveDate;
use log::{debug, trace};
use parsec_rs::{char, float32, int32, keyword, pred, spaces, string, uint32, Parser};
use thiserror::Error;

use payroll_domain::{EmployeeId, MemberId};
use tx_app::Tx;

#[derive(Debug, Clone, Error)]
#[error("parse error at position {position}: {message} (found: {found:?})")]
pub struct TextParserError {
    pub position: usize,
    pub message: String,
    pub found: Option<String>,
}
impl From<parsec_rs::ParseError> for TextParserError {
    fn from(e: parsec_rs::ParseError) -> Self {
        Self {
            position: e.position(),
            message: e.expected().join(" or "),
            found: e.found().cloned(),
        }
    }
}

pub fn read_tx(line: &str) -> Result<Tx, TextParserError> {
    trace!("read_tx called");
    match transaction().parse(line) {
        Ok((tx, _)) => Ok(tx),
        Err(e) => Err(e.into()),
    }
}

fn transaction() -> impl Parser<Item = Tx> {
    go_through().skip(
        add_hourly_emp()
            .or(add_salary_emp())
            .or(add_commissioned_emp())
            .or(del_emp())
            .or(time_card())
            .or(sales_receipt())
            .or(service_charge())
            .or(chg_name())
            .or(chg_address())
            .or(chg_hourly())
            .or(chg_salaried())
            .or(chg_commissioned())
            .or(chg_hold())
            .or(chg_direct())
            .or(chg_mail())
            .or(chg_member())
            .or(chg_no_member())
            .or(payday())
            // for testing
            .or(verify_gross_pay())
            .or(verify_deductions())
            .or(verify_net_pay()),
    )
}
#[cfg(test)]
mod test_transaction {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test_go_through() {
        let input = "";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "")));

        let input = "Code";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));

        let input = "# comment\nCode";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));

        let input = "# comment\n#\n# comment\nCode";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));

        let input = " \t\n# comment\n#\nCode";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));

        let input = " \t\n# comment\n#\n \tCode";
        let result = go_through().parse(input);
        assert_eq!(result, Ok(((), "Code")));
    }

    #[test]
    fn test_add_hourly_emp() {
        let input = r#"AddEmp 42 "Bob" "Home" H 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddHourlyEmployee {
                    id: 42.into(),
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    hourly_rate: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_add_salary_emp() {
        let input = r#"AddEmp 42 "Bob" "Home" S 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddSalariedEmployee {
                    id: 42.into(),
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    salary: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_add_commissioned_emp() {
        let input = r#"AddEmp 42 "Bob" "Home" C 1000.0 .1"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddCommissionedEmployee {
                    id: 42.into(),
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    salary: 1000.0,
                    commission_rate: 0.1
                },
                ""
            ))
        );
    }
    #[test]
    fn test_del_emp() {
        let input = r#"DelEmp 42"#;
        let result = transaction().parse(input);
        assert_eq!(result, Ok((Tx::DeleteEmployee { id: 42.into() }, "")));
    }
    #[test]
    fn test_time_card() {
        let input = r#"TimeCard 42 2021-01-01 8.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddTimeCard {
                    id: 42.into(),
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    hours: 8.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_sales_receipt() {
        let input = r#"SalesReceipt 42 2021-01-01 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddSalesReceipt {
                    id: 42.into(),
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    amount: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_service_charge() {
        let input = r#"ServiceCharge 42 2021-01-01 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddServiceCharge {
                    member_id: 42.into(),
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    amount: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_name() {
        let input = r#"ChgEmp 42 Name "Bob""#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeName {
                    id: 42.into(),
                    new_name: "Bob".to_string()
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_address() {
        let input = r#"ChgEmp 42 Address "123 Wall St.""#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeAddress {
                    id: 42.into(),
                    new_address: "123 Wall St.".to_string()
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_hourly() {
        let input = r#"ChgEmp 42 Hourly 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeHourly {
                    id: 42.into(),
                    hourly_rate: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_salaried() {
        let input = r#"ChgEmp 42 Salaried 1000.0"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeSalaried {
                    id: 42.into(),
                    salary: 1000.0
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_commissioned() {
        let input = r#"ChgEmp 42 Commissioned 1000.0 .1"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeCommissioned {
                    id: 42.into(),
                    salary: 1000.0,
                    commission_rate: 0.1
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_hold() {
        let input = r#"ChgEmp 42 Hold"#;
        let result = transaction().parse(input);
        assert_eq!(result, Ok((Tx::ChangeEmployeeHold { id: 42.into() }, "")));
    }
    #[test]
    fn test_chg_direct() {
        let input = r#"ChgEmp 42 Direct "mufg" "1234567""#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeDirect {
                    id: 42.into(),
                    bank: "mufg".to_string(),
                    account: "1234567".to_string()
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_mail() {
        let input = r#"ChgEmp 42 Mail "bob@gmail.com""#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeMail {
                    id: 42.into(),
                    address: "bob@gmail.com".to_string()
                },
                ""
            ))
        );
    }
    #[test]
    fn test_chg_member() {
        let input = r#"ChgEmp 42 Member 7234 Dues 9.45"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeMember {
                    emp_id: 42.into(),
                    member_id: 7234.into(),
                    dues: 9.45
                },
                "",
            ))
        );
    }
    #[test]
    fn test_no_member() {
        let input = r#"ChgEmp 42 NoMember"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((Tx::ChangeEmployeeNoMember { emp_id: 42.into() }, ""))
        );
    }
    #[test]
    fn test_payday() {
        let input = r#"Payday 2021-01-01"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::Payday {
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
                },
                ""
            ))
        );
    }
    #[test]
    fn test_verify_gross_pay() {
        let input = r#"Verify Paycheck 2025-01-31 EmpId 42 GrossPay 2130.55"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::VerifyGrossPay {
                    emp_id: 42.into(),
                    pay_date: NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
                    gross_pay: 2130.55,
                },
                ""
            ))
        );
    }
    #[test]
    fn test_verify_deductions() {
        let input = r#"Verify Paycheck 2025-01-31 EmpId 42 Deductions 118.55"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::VerifyDeductions {
                    emp_id: 42.into(),
                    pay_date: NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
                    deductions: 118.55,
                },
                ""
            ))
        );
    }
    #[test]
    fn test_verify_net_pay() {
        let input = r#"Verify Paycheck 2025-01-31 EmpId 42 NetPay 1989.90"#;
        let result = transaction().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::VerifyNetPay {
                    emp_id: 42.into(),
                    pay_date: NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
                    net_pay: 1989.90,
                },
                ""
            ))
        );
    }
}

fn go_through() -> impl Parser<Item = ()> {
    let comment = char('#').skip(pred(|c| c != '\n').many0().with(char('\n')));
    let space_comment = spaces().skip(comment).map(|_| ());
    let ignore = space_comment.many1().map(|_| ()).or(spaces().map(|_| ()));

    spaces().skip(ignore).skip(spaces()).map(|_| ())
}

fn employee_id() -> impl Parser<Item = EmployeeId> {
    uint32()
        .map(Into::into)
        .with(spaces())
        .label("employee_id".into())
}

fn member_id() -> impl Parser<Item = MemberId> {
    uint32()
        .map(Into::into)
        .with(spaces())
        .label("member_id".into())
}

fn add_hourly_emp() -> impl Parser<Item = Tx> {
    let prefix = keyword("AddEmp").skip(spaces());
    let emp_id = employee_id();
    let name = string().with(spaces()).label("name".into());
    let address = string().with(spaces()).label("address".into());
    let hourly = char('H').skip(spaces()).label("`H'".into());
    let hourly_rate = float32().label("hourly rate".into());

    prefix
        .skip(emp_id)
        .join(name)
        .join(address)
        .join(hourly)
        .join(hourly_rate)
        .map(|((((id, name), address), _), hourly_rate)| {
            debug!(
                "parsed AddHourlyEmployee: id={}, name={}, address={}, hourly_rate={}",
                id, name, address, hourly_rate
            );
            Tx::AddHourlyEmployee {
                id,
                name,
                address,
                hourly_rate,
            }
        })
}
#[cfg(test)]
mod test_add_hourly_emp {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"AddEmp 1 "Bob" "Home" H 1000.0"#;
        let result = add_hourly_emp().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddHourlyEmployee {
                    id: 1.into(),
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    hourly_rate: 1000.0
                },
                ""
            ))
        );
    }
}

fn add_salary_emp() -> impl Parser<Item = Tx> {
    let prefix = keyword("AddEmp").skip(spaces());
    let emp_id = employee_id();
    let name = string().with(spaces()).label("name".into());
    let address = string().with(spaces()).label("address".into());
    let salaried = char('S').skip(spaces()).label("`S'".into());
    let monthly_rate = float32().with(spaces()).label("monthly salary".into());

    prefix
        .skip(emp_id)
        .join(name)
        .join(address)
        .join(salaried)
        .join(monthly_rate)
        .map(|((((id, name), address), _), salary)| {
            debug!(
                "parsed AddSalariedEmployee: id={}, name={}, address={}, salary={}",
                id, name, address, salary
            );
            Tx::AddSalariedEmployee {
                id,
                name,
                address,
                salary,
            }
        })
}
#[cfg(test)]
mod test_add_salary_emp {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"AddEmp 1 "Bob" "Home" S 1000.0"#;
        let result = add_salary_emp().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddSalariedEmployee {
                    id: 1.into(),
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    salary: 1000.0
                },
                ""
            ))
        );
    }
}

fn add_commissioned_emp() -> impl Parser<Item = Tx> {
    let prefix = keyword("AddEmp").skip(spaces());
    let emp_id = employee_id();
    let name = string().with(spaces()).label("name".into());
    let address = string().with(spaces()).label("address".into());
    let salaried = char('C').skip(spaces()).label("`C'".into());
    let salary = float32().label("salary".into()).with(spaces());
    let commission_rate = float32().label("commission rate".into());

    prefix
        .skip(emp_id)
        .join(name)
        .join(address)
        .join(salaried)
        .join(salary)
        .join(commission_rate)
        .map(|(((((id, name), address), _), salary), commission_rate)| {
            debug!(
		    "parsed AddCommissionedEmployee: id={}, name={}, address={}, salary={}, commission_rate={}",
		    id,
		    name,
		    address,
		    salary,
		    commission_rate
		);
            Tx::AddCommissionedEmployee {
                id,
                name,
                address,
                salary,
                commission_rate,
            }
        })
}
#[cfg(test)]
mod test_add_commissioned_emp {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"AddEmp 1 "Bob" "Home" C 1000.0 .1"#;
        let result = add_commissioned_emp().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddCommissionedEmployee {
                    id: 1.into(),
                    name: "Bob".to_string(),
                    address: "Home".to_string(),
                    salary: 1000.0,
                    commission_rate: 0.1
                },
                ""
            ))
        );
    }
}

fn del_emp() -> impl Parser<Item = Tx> {
    let prefix = keyword("DelEmp").skip(spaces());
    let emp_id = employee_id();

    prefix.skip(emp_id).map(|id| {
        debug!("parsed DeleteEmployee: id={}", id);
        Tx::DeleteEmployee { id }
    })
}
#[cfg(test)]
mod test_del_emp {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"DelEmp 1"#;
        let result = del_emp().parse(input);
        assert_eq!(result, Ok((Tx::DeleteEmployee { id: 1.into() }, "")));
    }
}

fn date() -> impl Parser<Item = NaiveDate> {
    let year = int32().with(char('-'));
    let month = uint32().with(char('-'));
    let day = uint32();
    let date = year.join(month).join(day).label("date".into());

    date.map(|((y, m), d)| {
        debug!("parsed date: {}-{:02}-{:02}", y, m, d);
        NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32).expect("date")
    })
}
#[cfg(test)]
mod test_date {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = "2021-01-01";
        let result = date().parse(input);
        assert_eq!(
            result,
            Ok((NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), ""))
        );
    }
}

fn time_card() -> impl Parser<Item = Tx> {
    let prefix = keyword("TimeCard").skip(spaces());
    let emp_id = employee_id();
    let date = date().with(spaces());
    let hours = float32().label("hour".into());

    prefix
        .skip(emp_id)
        .join(date)
        .join(hours)
        .map(|((id, date), hours)| {
            debug!("parsed TimeCard: id={}, date={}, hours={}", id, date, hours);
            Tx::AddTimeCard { id, date, hours }
        })
}
#[cfg(test)]
mod test_time_card {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"TimeCard 1 2021-01-01 8.0"#;
        let result = time_card().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddTimeCard {
                    id: 1.into(),
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    hours: 8.0
                },
                ""
            ))
        );
    }
}

fn sales_receipt() -> impl Parser<Item = Tx> {
    let prefix = keyword("SalesReceipt").skip(spaces());
    let emp_id = employee_id();
    let date = date().with(spaces());
    let amount = float32().label("amount".into());

    prefix
        .skip(emp_id)
        .join(date)
        .join(amount)
        .map(|((id, date), amount)| {
            debug!(
                "parsed SalesReceipt: id={}, date={}, amount={}",
                id, date, amount
            );
            Tx::AddSalesReceipt { id, date, amount }
        })
}
#[cfg(test)]
mod test_sales_receipt {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"SalesReceipt 1 2021-01-01 1000.0"#;
        let result = sales_receipt().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddSalesReceipt {
                    id: 1.into(),
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    amount: 1000.0
                },
                ""
            ))
        );
    }
}

fn service_charge() -> impl Parser<Item = Tx> {
    let prefix = keyword("ServiceCharge").skip(spaces());
    let member_id = member_id();
    let date = date().with(spaces());
    let amount = float32().label("amount".into());

    prefix
        .skip(member_id)
        .join(date)
        .join(amount)
        .map(|((member_id, date), amount)| {
            debug!(
                "parsed ServiceCharge: member_id={}, date={}, amount={}",
                member_id, date, amount
            );
            Tx::AddServiceCharge {
                member_id,
                date,
                amount,
            }
        })
}
#[cfg(test)]
mod test_service_charge {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ServiceCharge 1 2021-01-01 1000.0"#;
        let result = service_charge().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::AddServiceCharge {
                    member_id: 1.into(),
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                    amount: 1000.0
                },
                ""
            ))
        );
    }
}

fn chg_name() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let name = keyword("Name")
        .skip(spaces())
        .skip(string())
        .label("new name".into());

    prefix.skip(emp_id).join(name).map(|(id, new_name)| {
        debug!(
            "parsed ChangeEmployeeName: id={}, new_name={}",
            id, new_name
        );
        Tx::ChangeEmployeeName { id, new_name }
    })
}
#[cfg(test)]
mod test_chg_name {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Name "Bob""#;
        let result = chg_name().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeName {
                    id: 1.into(),
                    new_name: "Bob".to_string()
                },
                ""
            ))
        );
    }
}

fn chg_address() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let address = keyword("Address")
        .skip(spaces())
        .skip(string())
        .label("new address".into());

    prefix.skip(emp_id).join(address).map(|(id, new_address)| {
        debug!(
            "parsed ChangeEmployeeAddress: id={}, new_address={}",
            id, new_address
        );
        Tx::ChangeEmployeeAddress { id, new_address }
    })
}
#[cfg(test)]
mod test_chg_address {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Address "123 Main St""#;
        let result = chg_address().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeAddress {
                    id: 1.into(),
                    new_address: "123 Main St".to_string()
                },
                ""
            ))
        );
    }
}

fn chg_hourly() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let hourly_rate = keyword("Hourly")
        .skip(spaces())
        .skip(float32())
        .label("hourly rate".into());

    prefix
        .skip(emp_id)
        .join(hourly_rate)
        .map(|(id, hourly_rate)| {
            debug!(
                "parsed ChangeEmployeeHourly: id={}, hourly_rate={}",
                id, hourly_rate
            );
            Tx::ChangeEmployeeHourly { id, hourly_rate }
        })
}
#[cfg(test)]
mod test_chg_hourly {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Hourly 13.78"#;
        let result = chg_hourly().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeHourly {
                    id: 1.into(),
                    hourly_rate: 13.78
                },
                ""
            ))
        );
    }
}

fn chg_salaried() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let salaried = keyword("Salaried")
        .skip(spaces())
        .skip(float32())
        .label("monthly salary".into());

    prefix.skip(emp_id).join(salaried).map(|(id, salary)| {
        debug!(
            "parsed ChangeEmployeeSalaried: id={}, salary={}",
            id, salary
        );
        Tx::ChangeEmployeeSalaried { id, salary }
    })
}
#[cfg(test)]
mod test_chg_salaried {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Salaried 1023.456"#;
        let result = chg_salaried().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeSalaried {
                    id: 1.into(),
                    salary: 1023.456
                },
                ""
            ))
        );
    }
}

fn chg_commissioned() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let salary = keyword("Commissioned")
        .skip(spaces())
        .skip(float32())
        .label("salary".into())
        .with(spaces());
    let commission_rate = float32().label("commission rate".into());

    prefix
        .skip(emp_id)
        .join(salary)
        .join(commission_rate)
        .map(|((id, salary), commission_rate)| {
            debug!(
                "parsed ChangeEmployeeCommissioned: id={}, salary={}, commission_rate={}",
                id, salary, commission_rate
            );
            Tx::ChangeEmployeeCommissioned {
                id,
                salary,
                commission_rate,
            }
        })
}
#[cfg(test)]
mod test_chg_commissioned {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Commissioned 1018.91 .19"#;
        let result = chg_commissioned().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeCommissioned {
                    id: 1.into(),
                    salary: 1018.91,
                    commission_rate: 0.19
                },
                ""
            ))
        );
    }
}

fn chg_hold() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let hold = keyword("Hold").label("`Hold'".into());

    prefix.skip(emp_id).with(hold).map(|id| {
        debug!("parsed ChangeEmployeeHold: id={}", id);
        Tx::ChangeEmployeeHold { id }
    })
}
#[cfg(test)]
mod test_chg_hold {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Hold"#;
        let result = chg_hold().parse(input);
        assert_eq!(result, Ok((Tx::ChangeEmployeeHold { id: 1.into() }, "")));
    }
}

fn chg_direct() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let bank = keyword("Direct")
        .label("`Direct'".into())
        .skip(spaces())
        .skip(string())
        .label("bank".into())
        .with(spaces());
    let account = string().label("account".into());

    prefix
        .skip(emp_id)
        .join(bank)
        .join(account)
        .map(|((id, bank), account)| {
            debug!(
                "parsed ChangeEmployeeDirect: id={}, bank={}, account={}",
                id, bank, account
            );
            Tx::ChangeEmployeeDirect { id, bank, account }
        })
}
#[cfg(test)]
mod test_chg_direct {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Direct "Bank" "Account""#;
        let result = chg_direct().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeDirect {
                    id: 1.into(),
                    bank: "Bank".to_string(),
                    account: "Account".to_string()
                },
                ""
            ))
        );
    }
}

fn chg_mail() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let address = keyword("Mail")
        .label("`Mail'".into())
        .skip(spaces())
        .skip(string())
        .label("mail address".into());

    prefix.skip(emp_id).join(address).map(|(id, address)| {
        debug!("parsed ChangeEmployeeMail: id={}, address={}", id, address);
        Tx::ChangeEmployeeMail { id, address }
    })
}
#[cfg(test)]
mod test_chg_mail {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Mail "bob@gmail.com""#;
        let result = chg_mail().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeMail {
                    id: 1.into(),
                    address: "bob@gmail.com".to_string()
                },
                ""
            ))
        );
    }
}

fn chg_member() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let member_id = keyword("Member")
        .label("affiliation".into())
        .skip(spaces())
        .skip(member_id());
    let dues = keyword("Dues")
        .skip(spaces())
        .skip(float32())
        .label("dues".into());

    prefix
        .skip(emp_id)
        .join(member_id)
        .join(dues)
        .map(|((emp_id, member_id), dues)| {
            debug!(
                "parsed ChangeEmployeeMember: emp_id={}, member_id={}, dues={}",
                emp_id, member_id, dues
            );
            Tx::ChangeEmployeeMember {
                emp_id,
                member_id,
                dues,
            }
        })
}
#[cfg(test)]
mod test_chg_member {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 Member 2 Dues 100.0"#;
        let result = chg_member().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::ChangeEmployeeMember {
                    emp_id: 1.into(),
                    member_id: 2.into(),
                    dues: 100.0
                },
                ""
            ))
        );
    }
}

fn chg_no_member() -> impl Parser<Item = Tx> {
    let prefix = keyword("ChgEmp").skip(spaces());
    let emp_id = employee_id();
    let no_member = keyword("NoMember").label("affiliation".into());

    prefix.skip(emp_id).with(no_member).map(|emp_id| {
        debug!("parsed ChangeEmployeeNoMember: emp_id={}", emp_id);
        Tx::ChangeEmployeeNoMember { emp_id }
    })
}
#[cfg(test)]
mod test_chg_no_member {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"ChgEmp 1 NoMember"#;
        let result = chg_no_member().parse(input);
        assert_eq!(
            result,
            Ok((Tx::ChangeEmployeeNoMember { emp_id: 1.into() }, ""))
        );
    }
}

fn payday() -> impl Parser<Item = Tx> {
    let prefix = keyword("Payday").skip(spaces());
    let date = date();

    prefix.skip(date).map(|date| {
        debug!("parsed Payday: date={}", date);
        Tx::Payday { date }
    })
}
#[cfg(test)]
mod test_payday {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"Payday 2021-01-01"#;
        let result = payday().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::Payday {
                    date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
                },
                ""
            ))
        );
    }
}

fn verify_gross_pay() -> impl Parser<Item = Tx> {
    let verify = keyword("Verify").skip(spaces());
    let paycheck = keyword("Paycheck").skip(spaces());
    let pay_date = date().with(spaces());
    let emp_id = keyword("EmpId").skip(spaces()).skip(employee_id());
    let gross_pay = keyword("GrossPay")
        .skip(spaces())
        .skip(float32())
        .label("gross pay".into());

    verify
        .skip(paycheck)
        .skip(pay_date)
        .join(emp_id)
        .join(gross_pay)
        .map(|((pay_date, emp_id), gross_pay)| {
            debug!(
                "parsed VerifyGrossPay: pay_date={}, emp_id={}, gross_pay={}",
                pay_date, emp_id, gross_pay
            );
            Tx::VerifyGrossPay {
                pay_date,
                emp_id,
                gross_pay,
            }
        })
}
#[cfg(test)]
mod test_verify_gross_pay {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"Verify Paycheck 2025-01-31 EmpId 1234 GrossPay 2280.35"#;
        let result = verify_gross_pay().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::VerifyGrossPay {
                    pay_date: NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
                    emp_id: 1234.into(),
                    gross_pay: 2280.35
                },
                ""
            ))
        );
    }
}

fn verify_deductions() -> impl Parser<Item = Tx> {
    let verify = keyword("Verify").skip(spaces());
    let paycheck = keyword("Paycheck").skip(spaces());
    let pay_date = date().with(spaces());
    let emp_id = keyword("EmpId").skip(spaces()).skip(employee_id());
    let deductions = keyword("Deductions")
        .skip(spaces())
        .skip(float32())
        .label("deductions".into());

    verify
        .skip(paycheck)
        .skip(pay_date)
        .join(emp_id)
        .join(deductions)
        .map(|((pay_date, emp_id), deductions)| {
            debug!(
                "parsed VerifyDeductions: pay_date={}, emp_id={}, deductions={}",
                pay_date, emp_id, deductions
            );
            Tx::VerifyDeductions {
                pay_date,
                emp_id,
                deductions,
            }
        })
}
#[cfg(test)]
mod test_verify_deductions {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"Verify Paycheck 2025-01-31 EmpId 1234 Deductions 180.55"#;
        let result = verify_deductions().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::VerifyDeductions {
                    pay_date: NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
                    emp_id: 1234.into(),
                    deductions: 180.55
                },
                ""
            ))
        );
    }
}

fn verify_net_pay() -> impl Parser<Item = Tx> {
    let verify = keyword("Verify").skip(spaces());
    let paycheck = keyword("Paycheck").skip(spaces());
    let pay_date = date().with(spaces());
    let emp_id = keyword("EmpId").skip(spaces()).skip(employee_id());
    let net_pay = keyword("NetPay")
        .skip(spaces())
        .skip(float32())
        .label("net pay".into());

    verify
        .skip(paycheck)
        .skip(pay_date)
        .join(emp_id)
        .join(net_pay)
        .map(|((pay_date, emp_id), net_pay)| {
            debug!(
                "parsed VerifyNetPay: pay_date={}, emp_id={}, net_pay={}",
                pay_date, emp_id, net_pay
            );
            Tx::VerifyNetPay {
                pay_date,
                emp_id,
                net_pay,
            }
        })
}
#[cfg(test)]
mod test_verify_net_pay {
    use super::*;
    use parsec_rs::Parser;

    #[test]
    fn test() {
        let input = r#"Verify Paycheck 2025-01-31 EmpId 1234 NetPay 2099.80"#;
        let result = verify_net_pay().parse(input);
        assert_eq!(
            result,
            Ok((
                Tx::VerifyNetPay {
                    pay_date: NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
                    emp_id: 1234.into(),
                    net_pay: 2099.80
                },
                ""
            ))
        );
    }
}
