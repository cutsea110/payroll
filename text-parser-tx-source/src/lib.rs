use log::{debug, error, trace, warn};
use std::io::BufRead;

use tx_app::{Transaction, Tx, TxSource};
use tx_factory::{
    AddCommissionedEmployeeTxFactory, AddHourlyEmployeeTxFactory, AddSalariedEmployeeTxFactory,
    AddSalesReceiptTxFactory, AddServiceChargeTxFactory, AddTimecardTxFactory,
    ChangeEmployeeAddressTxFactory, ChangeEmployeeCommissionedTxFactory,
    ChangeEmployeeDirectTxFactory, ChangeEmployeeHoldTxFactory, ChangeEmployeeHourlyTxFactory,
    ChangeEmployeeMailTxFactory, ChangeEmployeeMemberTxFactory, ChangeEmployeeNameTxFactory,
    ChangeEmployeeNoMemberTxFactory, ChangeEmployeeSalariedTxFactory, DeleteEmployeeTxFactory,
    PaydayTxFactory,
};

mod parser;

pub struct TextParserTxSource<F> {
    tx_factory: F,
    reader: Box<dyn BufRead>,
}
impl<F> TextParserTxSource<F>
where
    F: AddSalariedEmployeeTxFactory
        + AddHourlyEmployeeTxFactory
        + AddCommissionedEmployeeTxFactory
        + DeleteEmployeeTxFactory
        + AddTimecardTxFactory
        + AddSalesReceiptTxFactory
        + AddServiceChargeTxFactory
        + ChangeEmployeeNameTxFactory
        + ChangeEmployeeAddressTxFactory
        + ChangeEmployeeSalariedTxFactory
        + ChangeEmployeeHourlyTxFactory
        + ChangeEmployeeCommissionedTxFactory
        + ChangeEmployeeHoldTxFactory
        + ChangeEmployeeDirectTxFactory
        + ChangeEmployeeMailTxFactory
        + ChangeEmployeeMemberTxFactory
        + ChangeEmployeeNoMemberTxFactory
        + PaydayTxFactory,
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
            } => AddHourlyEmployeeTxFactory::mk_tx(
                &self.tx_factory,
                id,
                &name,
                &address,
                hourly_rate,
            ),
            Tx::AddSalariedEmployee {
                id,
                name,
                address,
                salary,
            } => AddSalariedEmployeeTxFactory::mk_tx(&self.tx_factory, id, &name, &address, salary),
            Tx::AddCommissionedEmployee {
                id,
                name,
                address,
                salary,
                commission_rate,
            } => AddCommissionedEmployeeTxFactory::mk_tx(
                &self.tx_factory,
                id,
                &name,
                &address,
                salary,
                commission_rate,
            ),
            Tx::DeleteEmployee { id } => DeleteEmployeeTxFactory::mk_tx(&self.tx_factory, id),
            Tx::AddTimeCard { id, date, hours } => {
                AddTimecardTxFactory::mk_tx(&self.tx_factory, id, date, hours)
            }
            Tx::AddSalesReceipt { id, date, amount } => {
                AddSalesReceiptTxFactory::mk_tx(&self.tx_factory, id, date, amount)
            }
            Tx::AddServiceCharge {
                member_id,
                date,
                amount,
            } => AddServiceChargeTxFactory::mk_tx(&self.tx_factory, member_id, date, amount),
            Tx::ChangeEmployeeName { id, new_name } => {
                ChangeEmployeeNameTxFactory::mk_tx(&self.tx_factory, id, &new_name)
            }
            Tx::ChangeEmployeeAddress { id, new_address } => {
                ChangeEmployeeAddressTxFactory::mk_tx(&self.tx_factory, id, &new_address)
            }
            Tx::ChangeEmployeeHourly { id, hourly_rate } => {
                ChangeEmployeeHourlyTxFactory::mk_tx(&self.tx_factory, id, hourly_rate)
            }
            Tx::ChangeEmployeeSalaried { id, salary } => {
                ChangeEmployeeSalariedTxFactory::mk_tx(&self.tx_factory, id, salary)
            }
            Tx::ChangeEmployeeCommissioned {
                id,
                salary,
                commission_rate,
            } => ChangeEmployeeCommissionedTxFactory::mk_tx(
                &self.tx_factory,
                id,
                salary,
                commission_rate,
            ),
            Tx::ChangeEmployeeHold { id } => {
                ChangeEmployeeHoldTxFactory::mk_tx(&self.tx_factory, id)
            }
            Tx::ChangeEmployeeDirect { id, bank, account } => {
                ChangeEmployeeDirectTxFactory::mk_tx(&self.tx_factory, id, &bank, &account)
            }
            Tx::ChangeEmployeeMail { id, address } => {
                ChangeEmployeeMailTxFactory::mk_tx(&self.tx_factory, id, &address)
            }
            Tx::ChangeEmployeeMember {
                emp_id,
                member_id,
                dues,
            } => ChangeEmployeeMemberTxFactory::mk_tx(&self.tx_factory, emp_id, member_id, dues),
            Tx::ChangeEmployeeNoMember { emp_id } => {
                ChangeEmployeeNoMemberTxFactory::mk_tx(&self.tx_factory, emp_id)
            }
            Tx::Payday { date } => PaydayTxFactory::mk_tx(&self.tx_factory, date),
        }
    }
}

impl<F> TxSource for TextParserTxSource<F>
where
    F: AddSalariedEmployeeTxFactory
        + AddHourlyEmployeeTxFactory
        + AddCommissionedEmployeeTxFactory
        + DeleteEmployeeTxFactory
        + AddTimecardTxFactory
        + AddSalesReceiptTxFactory
        + AddServiceChargeTxFactory
        + ChangeEmployeeNameTxFactory
        + ChangeEmployeeAddressTxFactory
        + ChangeEmployeeSalariedTxFactory
        + ChangeEmployeeHourlyTxFactory
        + ChangeEmployeeCommissionedTxFactory
        + ChangeEmployeeHoldTxFactory
        + ChangeEmployeeDirectTxFactory
        + ChangeEmployeeMailTxFactory
        + ChangeEmployeeMemberTxFactory
        + ChangeEmployeeNoMemberTxFactory
        + PaydayTxFactory,
{
    fn get_tx_source(&mut self) -> Option<Box<dyn Transaction>> {
        trace!("get_tx_source called");
        loop {
            let mut buf = String::new();
            let line = self.reader.read_line(&mut buf);
            debug!("Got line: {:?}", buf);

            // The case of empty line or comment line.
            // In this case, parser::read_tx will return an error, but we'd like to ignore it.
            if parser::ignoreable(&buf) {
                debug!("Ignoring line: {:?}", buf);
                continue;
            }

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
