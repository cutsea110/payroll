use log::{debug, error, trace, warn};
use std::io::BufRead;

use tx_app::{Transaction, Tx, TxSource};
use tx_factory::TxFactory;

mod parser;

pub struct TextParserTxSource<F>
where
    F: TxFactory,
{
    tx_factory: F,
    reader: Box<dyn BufRead>,
}
impl<F> TextParserTxSource<F>
where
    F: TxFactory,
{
    pub fn new(tx_factory: F, reader: Box<dyn BufRead>) -> Self {
        Self { tx_factory, reader }
    }
    fn dispatch(&self, tx: Tx) -> Box<dyn Transaction> {
        match tx {
            Tx::AddHourlyEmployee {
                id,
                name,
                address,
                hourly_rate,
            } => self
                .tx_factory
                .mk_add_hourly_employee_tx(id, &name, &address, hourly_rate),
            Tx::AddSalariedEmployee {
                id,
                name,
                address,
                salary,
            } => self
                .tx_factory
                .mk_add_salaried_employee_tx(id, &name, &address, salary),
            Tx::AddCommissionedEmployee {
                id,
                name,
                address,
                salary,
                commission_rate,
            } => self.tx_factory.mk_add_commissioned_employee_tx(
                id,
                &name,
                &address,
                salary,
                commission_rate,
            ),
            Tx::DeleteEmployee { id } => self.tx_factory.mk_delete_employee_tx(id),
            Tx::AddTimeCard { id, date, hours } => {
                self.tx_factory.mk_add_timecard_tx(id, date, hours)
            }
            Tx::AddSalesReceipt { id, date, amount } => {
                self.tx_factory.mk_add_sales_receipt_tx(id, date, amount)
            }
            Tx::AddServiceCharge {
                member_id,
                date,
                amount,
            } => self
                .tx_factory
                .mk_add_service_charge_tx(member_id, date, amount),
            Tx::ChangeEmployeeName { id, new_name } => {
                self.tx_factory.mk_change_employee_name_tx(id, &new_name)
            }
            Tx::ChangeEmployeeAddress { id, new_address } => self
                .tx_factory
                .mk_change_employee_address_tx(id, &new_address),
            Tx::ChangeEmployeeHourly { id, hourly_rate } => self
                .tx_factory
                .mk_change_employee_hourly_tx(id, hourly_rate),
            Tx::ChangeEmployeeSalaried { id, salary } => {
                self.tx_factory.mk_change_employee_salaried_tx(id, salary)
            }
            Tx::ChangeEmployeeCommissioned {
                id,
                salary,
                commission_rate,
            } => self
                .tx_factory
                .mk_change_employee_commissioned_tx(id, salary, commission_rate),
            Tx::ChangeEmployeeHold { id } => self.tx_factory.mk_change_employee_hold_tx(id),
            Tx::ChangeEmployeeDirect { id, bank, account } => self
                .tx_factory
                .mk_change_employee_direct_tx(id, &bank, &account),
            Tx::ChangeEmployeeMail { id, address } => {
                self.tx_factory.mk_change_employee_mail_tx(id, &address)
            }
            Tx::ChangeEmployeeMember {
                emp_id,
                member_id,
                dues,
            } => self
                .tx_factory
                .mk_change_employee_member_tx(emp_id, member_id, dues),
            Tx::ChangeEmployeeNoMember { emp_id } => {
                self.tx_factory.mk_change_employee_no_member_tx(emp_id)
            }
            Tx::Payday { date } => self.tx_factory.mk_payday_tx(date),
            Tx::VerifyGrossPay {
                emp_id,
                pay_date,
                gross_pay,
            } => self
                .tx_factory
                .mk_verify_gross_pay_tx(emp_id, pay_date, gross_pay),
            Tx::VerifyDeductions {
                emp_id,
                pay_date,
                deductions,
            } => self
                .tx_factory
                .mk_verify_deductions_tx(emp_id, pay_date, deductions),
            Tx::VerifyNetPay {
                emp_id,
                pay_date,
                net_pay,
            } => self
                .tx_factory
                .mk_verify_net_pay_tx(emp_id, pay_date, net_pay),
        }
    }
}

impl<F> TxSource for TextParserTxSource<F>
where
    F: TxFactory,
{
    fn get_tx_source(&mut self) -> Option<Box<dyn Transaction>> {
        trace!("TextParserTxSource::get_tx_source called");
        loop {
            let mut buf = String::new();
            let line = self.reader.read_line(&mut buf);
            debug!("Got line: {:?}", buf);

            match line {
                Ok(0) => {
                    debug!("Got EOS");
                    break;
                }
                Ok(_) => match parser::read_tx(&buf) {
                    Ok(tx) => {
                        debug!("Parsed tx: {:?}", tx);
                        let tx = self.dispatch(tx);
                        return Some(tx);
                    }
                    Err(e) => {
                        warn!("Skip line: {}", e);
                        let indent = " ".repeat(e.position);
                        eprintln!(
                            "Error parsing line: \n{}{}^ {} expected",
                            buf, indent, e.message
                        );
                        continue;
                    }
                },
                Err(ref e) => {
                    error!("Error reading line: {}", e);
                    break;
                }
            }
        }
        None
    }
}
