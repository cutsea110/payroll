use chrono::NaiveDate;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::VecDeque, fs, path::Path, rc::Rc};

use payroll_domain::{EmployeeId, MemberId};
use tx_app::{Transaction, TxSource};
use tx_factory::TxFactory;

mod parser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Tx {
    AddHourlyEmployee {
        id: EmployeeId,
        name: String,
        address: String,
        hourly_rate: f32,
    },
    AddSalariedEmployee {
        id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
    },
    AddCommissionedEmployee {
        id: EmployeeId,
        name: String,
        address: String,
        salary: f32,
        commission_rate: f32,
    },
    DeleteEmployee {
        id: EmployeeId,
    },
    AddTimeCard {
        id: EmployeeId,
        date: NaiveDate,
        hours: f32,
    },
    AddSalesReceipt {
        id: EmployeeId,
        date: NaiveDate,
        amount: f32,
    },
    AddServiceCharge {
        member_id: MemberId,
        date: NaiveDate,
        amount: f32,
    },
    ChangeEmployeeName {
        id: EmployeeId,
        new_name: String,
    },
    ChangeEmployeeAddress {
        id: EmployeeId,
        new_address: String,
    },
    ChangeEmployeeHourly {
        id: EmployeeId,
        hourly_rate: f32,
    },
    ChangeEmployeeSalaried {
        id: EmployeeId,
        salary: f32,
    },
    ChangeEmployeeCommissioned {
        id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    },
    ChangeEmployeeHold {
        id: EmployeeId,
    },
    ChangeEmployeeDirect {
        id: EmployeeId,
        bank: String,
        account: String,
    },
    ChangeEmployeeMail {
        id: EmployeeId,
        address: String,
    },
    ChangeEmployeeMember {
        emp_id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    },
    ChangeEmployeeNoMember {
        emp_id: EmployeeId,
    },
    Payday {
        date: NaiveDate,
    },
}
impl Tx {
    pub fn read_from_script_file<P>(file_path: P) -> VecDeque<Self>
    where
        P: AsRef<Path>,
    {
        trace!("Reading script file: {:?}", file_path.as_ref());
        let script = fs::read_to_string(file_path).expect("Failed to read file");
        let txs = parser::read_txs(&script);

        txs
    }
    pub fn read_from_json_file<P>(file_path: P) -> VecDeque<Self>
    where
        P: AsRef<Path>,
    {
        let json = fs::read_to_string(file_path).expect("Failed to read file");
        let deserialized: VecDeque<Self> =
            serde_json::from_str(&json).expect("Failed to deserialize");

        deserialized
    }
}

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
        self.txs.borrow_mut().clear();
    }
    pub fn load_from_script<P>(&self, file_path: P)
    where
        P: AsRef<Path>,
    {
        let txs = Tx::read_from_script_file(file_path);
        self.txs.borrow_mut().extend(txs);
    }
    pub fn load_from_json<P>(&self, file_path: P)
    where
        P: AsRef<Path>,
    {
        let txs = Tx::read_from_json_file(file_path);
        self.txs.borrow_mut().extend(txs);
    }
    pub fn store_to_json<P>(&self, file_path: P)
    where
        P: AsRef<Path>,
    {
        let txs = self.txs.borrow().clone();
        let json = serde_json::to_string(&txs).expect("Failed to serialize");
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
