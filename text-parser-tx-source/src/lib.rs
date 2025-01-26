use log::{debug, trace, warn};
use serde_json;
use std::{cell::RefCell, collections::VecDeque, fs, path::Path, rc::Rc};

use tx_app::{Transaction, Tx, TxSource};
use tx_factory::TxFactory;

mod parser;

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
    pub fn new(tx_factory: F) -> Self {
        Self {
            txs: Rc::new(RefCell::new(VecDeque::new())),
            tx_factory,
        }
    }
    pub fn clear_txs(&self) {
        trace!("TextParserTxSource::clear_txs called");
        self.txs.borrow_mut().clear();
    }
    pub fn load_from_script<P>(&self, file_path: P)
    where
        P: AsRef<Path>,
    {
        trace!("TextParserTxSource::load_from_script called");
        let script = fs::read_to_string(file_path).expect("Failed to read file");
        let txs = parser::read_txs(&script);
        debug!("txs: {:?}", txs);
        if txs.is_empty() {
            warn!("parsed script is empty");
        }
        self.txs.borrow_mut().extend(txs);
    }
    pub fn load_from_json<P>(&self, file_path: P)
    where
        P: AsRef<Path>,
    {
        trace!("TextParserTxSource::load_from_json called");
        let json = fs::read_to_string(file_path).expect("Failed to read file");
        let txs: VecDeque<Tx> = serde_json::from_str(&json).expect("Failed to deserialize");
        debug!("txs: {:?}", txs);
        if txs.is_empty() {
            warn!("parsed json is empty");
        }
        self.txs.borrow_mut().extend(txs);
    }
    pub fn store_to_json<P>(&self, file_path: P)
    where
        P: AsRef<Path>,
    {
        trace!("TextParserTxSource::store_to_json called");
        let txs = self.txs.borrow().clone();
        let json = serde_json::to_string(&txs).expect("Failed to serialize");
        debug!("json: {:?}", json);
        fs::write(file_path, json).expect("Failed to write file");
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
    fn get_tx_source(&self) -> Option<Box<dyn Transaction>> {
        trace!("TextParserTxSource::get_tx_source called");
        self.txs.borrow_mut().pop_front().map(|tx| {
            debug!("tx_src={:?}", tx);
            self.dispatch(tx)
        })
    }
}
