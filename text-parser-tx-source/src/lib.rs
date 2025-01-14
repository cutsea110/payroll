// TxSource の具体的な実装
use chrono::NaiveDate;
use log::{debug, trace};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// tx_app にのみ依存
use payroll_domain::{EmployeeId, MemberId};
use tx_app::{Transaction, TxSource};
use tx_factory::TxFactory;

pub struct TextParserTxSource<F>
where
    F: TxFactory,
{
    txs: Rc<RefCell<VecDeque<Tx>>>,
    tx_factory: F,
}
impl<F> TextParserTxSource<F>
where
    F: TxFactory,
{
    pub fn new(input: &str, tx_factory: F) -> Self {
        Self {
            txs: Rc::new(RefCell::new(parser::read_txs(input))),
            tx_factory,
        }
    }
    fn dispatch(&self, tx: Tx) -> Box<dyn Transaction> {
        match tx {
            Tx::AddHourlyEmployee(id, name, address, hourly_rate) => self
                .tx_factory
                .mk_add_hourly_employee_tx(id, &name, &address, hourly_rate),
            Tx::AddSalariedEmployee(id, name, address, salary) => self
                .tx_factory
                .mk_add_salaried_employee_tx(id, &name, &address, salary),
            Tx::AddCommissionedEmployee(id, name, address, salary, commission_rate) => self
                .tx_factory
                .mk_add_commissioned_employee_tx(id, &name, &address, salary, commission_rate),
            Tx::DeleteEmployee(id) => self.tx_factory.mk_delete_employee_tx(id),
            Tx::AddTimeCard(id, date, hours) => self.tx_factory.mk_add_timecard_tx(id, date, hours),
            Tx::AddSalesReceipt(id, date, amount) => {
                self.tx_factory.mk_add_sales_receipt_tx(id, date, amount)
            }
            Tx::AddServiceCharge(member_id, date, amount) => self
                .tx_factory
                .mk_add_service_charge_tx(member_id, date, amount),
            Tx::ChangeEmployeeName(id, new_name) => {
                self.tx_factory.mk_change_employee_name_tx(id, &new_name)
            }
            Tx::ChangeEmployeeAddress(id, new_address) => self
                .tx_factory
                .mk_change_employee_address_tx(id, &new_address),
            Tx::ChangeEmployeeHourly(id, hourly_rate) => self
                .tx_factory
                .mk_change_employee_hourly_tx(id, hourly_rate),
            Tx::ChangeEmployeeSalaried(id, salary) => {
                self.tx_factory.mk_change_employee_salaried_tx(id, salary)
            }
            Tx::ChangeEmployeeCommissioned(id, salary, commission_rate) => self
                .tx_factory
                .mk_change_employee_commissioned_tx(id, salary, commission_rate),
            Tx::ChangeEmployeeHold(id) => self.tx_factory.mk_change_employee_hold_tx(id),
            Tx::ChangeEmployeeDirect(id, bank, account) => self
                .tx_factory
                .mk_change_employee_direct_tx(id, &bank, &account),
            Tx::ChangeEmployeeMail(id, address) => {
                self.tx_factory.mk_change_employee_mail_tx(id, &address)
            }
            Tx::ChangeEmployeeMember(emp_id, member_id, dues) => self
                .tx_factory
                .mk_change_employee_member_tx(emp_id, member_id, dues),
            Tx::ChangeEmployeeNoMember(id) => self.tx_factory.mk_change_employee_no_member_tx(id),
            Tx::Payday(date) => self.tx_factory.mk_payday_tx(date),
        }
    }
}

impl<F> TxSource for TextParserTxSource<F>
where
    F: TxFactory,
{
    fn get_tx_source(&self) -> Option<Box<dyn Transaction + 'static>> {
        trace!("TextParserTxSource::get_tx_source called");
        self.txs.borrow_mut().pop_front().map(|tx| {
            debug!("tx_src={:?}", tx);
            self.dispatch(tx)
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Tx {
    AddHourlyEmployee(EmployeeId, String, String, f32),
    AddSalariedEmployee(EmployeeId, String, String, f32),
    AddCommissionedEmployee(EmployeeId, String, String, f32, f32),
    DeleteEmployee(EmployeeId),
    AddTimeCard(EmployeeId, NaiveDate, f32),
    AddSalesReceipt(EmployeeId, NaiveDate, f32),
    AddServiceCharge(MemberId, NaiveDate, f32),
    ChangeEmployeeName(EmployeeId, String),
    ChangeEmployeeAddress(EmployeeId, String),
    ChangeEmployeeHourly(EmployeeId, f32),
    ChangeEmployeeSalaried(EmployeeId, f32),
    ChangeEmployeeCommissioned(EmployeeId, f32, f32),
    ChangeEmployeeHold(EmployeeId),
    ChangeEmployeeDirect(EmployeeId, String, String),
    ChangeEmployeeMail(EmployeeId, String),
    ChangeEmployeeMember(EmployeeId, MemberId, f32),
    ChangeEmployeeNoMember(EmployeeId),
    Payday(NaiveDate),
}

mod parser {
    use log::{debug, trace};
    use std::collections::VecDeque;

    use chrono::NaiveDate;
    use parsec_rs::{char, float32, int32, keyword, pred, spaces, string, uint32, Parser};

    use super::Tx;

    pub fn read_txs(script: &str) -> VecDeque<Tx> {
        trace!("read_txs called");
        let txs: VecDeque<Tx> = transactions()
            .parse(script)
            .map(|p| p.0.into())
            .unwrap_or_default();
        debug!("txs={:?}", txs);
        txs
    }

    fn transactions() -> impl Parser<Item = Vec<Tx>> {
        transaction().many0()
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
                .or(payday()),
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
                    Tx::AddHourlyEmployee(42, "Bob".to_string(), "Home".to_string(), 1000.0),
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
                    Tx::AddSalariedEmployee(42, "Bob".to_string(), "Home".to_string(), 1000.0),
                    ""
                ))
            );
        }
        #[test]
        fn test_add_commissioned_emp() {
            let input = r#"AddEmp 42 "Bob" "Home" C 1000.0 0.1"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tx::AddCommissionedEmployee(
                        42,
                        "Bob".to_string(),
                        "Home".to_string(),
                        1000.0,
                        0.1
                    ),
                    ""
                ))
            );
        }
        #[test]
        fn test_del_emp() {
            let input = r#"DelEmp 42"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Tx::DeleteEmployee(42), "")));
        }
        #[test]
        fn test_time_card() {
            let input = r#"TimeCard 42 2021-01-01 8.0"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tx::AddTimeCard(42, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), 8.0),
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
                    Tx::AddSalesReceipt(42, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), 1000.0),
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
                    Tx::AddServiceCharge(42, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), 1000.0),
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
                Ok((Tx::ChangeEmployeeName(42, "Bob".to_string()), ""))
            );
        }
        #[test]
        fn test_chg_address() {
            let input = r#"ChgEmp 42 Address "123 Wall St.""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tx::ChangeEmployeeAddress(42, "123 Wall St.".to_string()),
                    ""
                ))
            );
        }
        #[test]
        fn test_chg_hourly() {
            let input = r#"ChgEmp 42 Hourly 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Tx::ChangeEmployeeHourly(42, 1000.0), "")));
        }
        #[test]
        fn test_chg_salaried() {
            let input = r#"ChgEmp 42 Salaried 1000.0"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Tx::ChangeEmployeeSalaried(42, 1000.0), "")));
        }
        #[test]
        fn test_chg_commissioned() {
            let input = r#"ChgEmp 42 Commissioned 1000.0 0.1"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((Tx::ChangeEmployeeCommissioned(42, 1000.0, 0.1), ""))
            );
        }
        #[test]
        fn test_chg_hold() {
            let input = r#"ChgEmp 42 Hold"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Tx::ChangeEmployeeHold(42), "")));
        }
        #[test]
        fn test_chg_direct() {
            let input = r#"ChgEmp 42 Direct "mufg" "1234567""#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tx::ChangeEmployeeDirect(42, "mufg".to_string(), "1234567".to_string()),
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
                Ok((Tx::ChangeEmployeeMail(42, "bob@gmail.com".to_string()), ""))
            );
        }
        #[test]
        fn test_chg_member() {
            let input = r#"ChgEmp 42 Member 7234 Dues 9.45"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Tx::ChangeEmployeeMember(42, 7234, 9.45,), "",)));
        }
        #[test]
        fn test_no_member() {
            let input = r#"ChgEmp 42 NoMember"#;
            let result = transaction().parse(input);
            assert_eq!(result, Ok((Tx::ChangeEmployeeNoMember(42), "")));
        }
        #[test]
        fn test_payday() {
            let input = r#"Payday 2021-01-01"#;
            let result = transaction().parse(input);
            assert_eq!(
                result,
                Ok((Tx::Payday(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()), ""))
            );
        }
    }

    fn go_through() -> impl Parser<Item = ()> {
        let comment = char('#').skip(pred(|c| c != '\n').many0().with(char('\n')));
        let space_comment = spaces().skip(comment).map(|_| ());
        let ignore = space_comment.many1().map(|_| ()).or(spaces().map(|_| ()));

        spaces().skip(ignore).skip(spaces()).map(|_| ())
    }

    fn add_hourly_emp() -> impl Parser<Item = Tx> {
        let prefix = keyword("AddEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = string().with(spaces());
        let address = string().with(spaces());
        let hourly_rate = char('H').skip(spaces()).skip(float32());

        prefix
            .skip(emp_id)
            .join(name)
            .join(address)
            .join(hourly_rate)
            .map(|(((emp_id, name), address), hourly_rate)| {
                debug!(
                    "parsed AddHourlyEmployee: emp_id={}, name={}, address={}, hourly_rate={}",
                    emp_id, name, address, hourly_rate
                );
                Tx::AddHourlyEmployee(emp_id, name, address, hourly_rate)
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
                    Tx::AddHourlyEmployee(1, "Bob".to_string(), "Home".to_string(), 1000.0),
                    ""
                ))
            );
        }
    }

    fn add_salary_emp() -> impl Parser<Item = Tx> {
        let prefix = keyword("AddEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = string().with(spaces());
        let address = string().with(spaces());
        let monthly_rate = char('S').skip(spaces()).skip(float32());

        prefix
            .skip(emp_id)
            .join(name)
            .join(address)
            .join(monthly_rate)
            .map(|(((emp_id, name), address), salary)| {
                debug!(
                    "parsed AddSalariedEmployee: emp_id={}, name={}, address={}, salary={}",
                    emp_id, name, address, salary
                );
                Tx::AddSalariedEmployee(emp_id, name, address, salary)
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
                    Tx::AddSalariedEmployee(1, "Bob".to_string(), "Home".to_string(), 1000.0),
                    ""
                ))
            );
        }
    }

    fn add_commissioned_emp() -> impl Parser<Item = Tx> {
        let prefix = keyword("AddEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = string().with(spaces());
        let address = string().with(spaces());
        let salary = char('C').skip(spaces()).skip(float32()).with(spaces());
        let commission_rate = float32();

        prefix
            .skip(emp_id)
            .join(name)
            .join(address)
            .join(salary)
            .join(commission_rate)
            .map(|((((emp_id, name), address), salary), commission_rate)| {
		debug!(
		    "parsed AddCommissionedEmployee: emp_id={}, name={}, address={}, salary={}, commission_rate={}",
		    emp_id,
		    name,
		    address,
		    salary,
		    commission_rate
		);
                Tx::AddCommissionedEmployee(emp_id, name, address, salary, commission_rate)
            })
    }
    #[cfg(test)]
    mod test_add_commissioned_emp {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"AddEmp 1 "Bob" "Home" C 1000.0 0.1"#;
            let result = add_commissioned_emp().parse(input);
            assert_eq!(
                result,
                Ok((
                    Tx::AddCommissionedEmployee(
                        1,
                        "Bob".to_string(),
                        "Home".to_string(),
                        1000.0,
                        0.1
                    ),
                    ""
                ))
            );
        }
    }

    fn del_emp() -> impl Parser<Item = Tx> {
        let prefix = keyword("DelEmp").skip(spaces());
        let emp_id = uint32();

        prefix.skip(emp_id).map(|emp_id| {
            debug!("parsed DeleteEmployee: emp_id={}", emp_id);
            Tx::DeleteEmployee(emp_id)
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
            assert_eq!(result, Ok((Tx::DeleteEmployee(1), "")));
        }
    }

    fn date() -> impl Parser<Item = NaiveDate> {
        let year = int32().with(char('-'));
        let month = uint32().with(char('-'));
        let day = uint32();

        year.join(month).join(day).map(|((y, m), d)| {
            debug!("parsed date: {}-{}-{}", y, m, d);
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
        let emp_id = uint32().with(spaces());
        let date = date().with(spaces());
        let hours = float32();

        prefix
            .skip(emp_id)
            .join(date)
            .join(hours)
            .map(|((emp_id, date), hours)| {
                debug!(
                    "parsed TimeCard: emp_id={}, date={}, hours={}",
                    emp_id, date, hours
                );
                Tx::AddTimeCard(emp_id, date, hours)
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
                    Tx::AddTimeCard(1, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), 8.0),
                    ""
                ))
            );
        }
    }

    fn sales_receipt() -> impl Parser<Item = Tx> {
        let prefix = keyword("SalesReceipt").skip(spaces());
        let emp_id = uint32().with(spaces());
        let date = date().with(spaces());
        let amount = float32();

        prefix
            .skip(emp_id)
            .join(date)
            .join(amount)
            .map(|((emp_id, date), amount)| {
                debug!(
                    "parsed SalesReceipt: emp_id={}, date={}, amount={}",
                    emp_id, date, amount
                );
                Tx::AddSalesReceipt(emp_id, date, amount)
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
                    Tx::AddSalesReceipt(1, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), 1000.0),
                    ""
                ))
            );
        }
    }

    fn service_charge() -> impl Parser<Item = Tx> {
        let prefix = keyword("ServiceCharge").skip(spaces());
        let member_id = uint32().with(spaces());
        let date = date().with(spaces());
        let amount = float32();

        prefix
            .skip(member_id)
            .join(date)
            .join(amount)
            .map(|((member_id, date), amount)| {
                debug!(
                    "parsed ServiceCharge: member_id={}, date={}, amount={}",
                    member_id, date, amount
                );
                Tx::AddServiceCharge(member_id, date, amount)
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
                    Tx::AddServiceCharge(1, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), 1000.0),
                    ""
                ))
            );
        }
    }

    fn chg_name() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let name = keyword("Name").skip(spaces()).skip(string());

        prefix.skip(emp_id).join(name).map(|(emp_id, name)| {
            debug!(
                "parsed ChangeEmployeeName: emp_id={}, name={}",
                emp_id, name
            );
            Tx::ChangeEmployeeName(emp_id, name)
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
                Ok((Tx::ChangeEmployeeName(1, "Bob".to_string()), ""))
            );
        }
    }

    fn chg_address() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let address = keyword("Address").skip(spaces()).skip(string());

        prefix.skip(emp_id).join(address).map(|(emp_id, address)| {
            debug!(
                "parsed ChangeEmployeeAddress: emp_id={}, address={}",
                emp_id, address
            );
            Tx::ChangeEmployeeAddress(emp_id, address)
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
                Ok((Tx::ChangeEmployeeAddress(1, "123 Main St".to_string()), ""))
            );
        }
    }

    fn chg_hourly() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let hourly_rate = keyword("Hourly").skip(spaces()).skip(float32());

        prefix
            .skip(emp_id)
            .join(hourly_rate)
            .map(|(emp_id, hourly_rate)| {
                debug!(
                    "parsed ChangeEmployeeHourly: emp_id={}, hourly_rate={}",
                    emp_id, hourly_rate
                );
                Tx::ChangeEmployeeHourly(emp_id, hourly_rate)
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
            assert_eq!(result, Ok((Tx::ChangeEmployeeHourly(1, 13.78), "")));
        }
    }

    fn chg_salaried() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let salaried = keyword("Salaried").skip(spaces()).skip(float32());

        prefix.skip(emp_id).join(salaried).map(|(emp_id, salary)| {
            debug!(
                "parsed ChangeEmployeeSalaried: emp_id={}, salary={}",
                emp_id, salary
            );
            Tx::ChangeEmployeeSalaried(emp_id, salary)
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
            assert_eq!(result, Ok((Tx::ChangeEmployeeSalaried(1, 1023.456), "")));
        }
    }

    fn chg_commissioned() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let salary = keyword("Commissioned")
            .skip(spaces())
            .skip(float32())
            .with(spaces());
        let commission_rate = float32();

        prefix.skip(emp_id).join(salary).join(commission_rate).map(
            |((emp_id, salary), commission_rate)| {
                debug!(
                    "parsed ChangeEmployeeCommissioned: emp_id={}, salary={}, commission_rate={}",
                    emp_id, salary, commission_rate
                );
                Tx::ChangeEmployeeCommissioned(emp_id, salary, commission_rate)
            },
        )
    }
    #[cfg(test)]
    mod test_chg_commissioned {
        use super::*;
        use parsec_rs::Parser;

        #[test]
        fn test() {
            let input = r#"ChgEmp 1 Commissioned 1018.91 0.19"#;
            let result = chg_commissioned().parse(input);
            assert_eq!(
                result,
                Ok((Tx::ChangeEmployeeCommissioned(1, 1018.91, 0.19), ""))
            );
        }
    }

    fn chg_hold() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let hold = keyword("Hold");

        prefix.skip(emp_id).with(hold).map(|emp_id| {
            debug!("parsed ChangeEmployeeHold: emp_id={}", emp_id);
            Tx::ChangeEmployeeHold(emp_id)
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
            assert_eq!(result, Ok((Tx::ChangeEmployeeHold(1), "")));
        }
    }

    fn chg_direct() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let bank = keyword("Direct")
            .skip(spaces())
            .skip(string())
            .with(spaces());
        let account = string();

        prefix
            .skip(emp_id)
            .join(bank)
            .join(account)
            .map(|((emp_id, bank), account)| {
                debug!(
                    "parsed ChangeEmployeeDirect: emp_id={}, bank={}, account={}",
                    emp_id, bank, account
                );
                Tx::ChangeEmployeeDirect(emp_id, bank, account)
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
                    Tx::ChangeEmployeeDirect(1, "Bank".to_string(), "Account".to_string()),
                    ""
                ))
            );
        }
    }

    fn chg_mail() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let address = keyword("Mail").skip(spaces()).skip(string());

        prefix.skip(emp_id).join(address).map(|(emp_id, address)| {
            debug!(
                "parsed ChangeEmployeeMail: emp_id={}, address={}",
                emp_id, address
            );
            Tx::ChangeEmployeeMail(emp_id, address)
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
                Ok((Tx::ChangeEmployeeMail(1, "bob@gmail.com".to_string()), ""))
            );
        }
    }

    fn chg_member() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let member_id = keyword("Member")
            .skip(spaces())
            .skip(uint32())
            .with(spaces());
        let dues = keyword("Dues").skip(spaces()).skip(float32());

        prefix
            .skip(emp_id)
            .join(member_id)
            .join(dues)
            .map(|((emp_id, member_id), dues)| {
                debug!(
                    "parsed ChangeEmployeeMember: emp_id={}, member_id={}, dues={}",
                    emp_id, member_id, dues
                );
                Tx::ChangeEmployeeMember(emp_id, member_id, dues)
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
            assert_eq!(result, Ok((Tx::ChangeEmployeeMember(1, 2, 100.0), "")));
        }
    }

    fn chg_no_member() -> impl Parser<Item = Tx> {
        let prefix = keyword("ChgEmp").skip(spaces());
        let emp_id = uint32().with(spaces());
        let no_member = keyword("NoMember");

        prefix.skip(emp_id).with(no_member).map(|emp_id| {
            debug!("parsed ChangeEmployeeNoMember: emp_id={}", emp_id);
            Tx::ChangeEmployeeNoMember(emp_id)
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
            assert_eq!(result, Ok((Tx::ChangeEmployeeNoMember(1), "")));
        }
    }

    fn payday() -> impl Parser<Item = Tx> {
        let prefix = keyword("Payday").skip(spaces());
        let date = date();

        prefix.skip(date).map(|pay_date| {
            debug!("parsed Payday: pay_date={}", pay_date);
            Tx::Payday(pay_date)
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
                Ok((Tx::Payday(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()), ""))
            );
        }
    }
}
