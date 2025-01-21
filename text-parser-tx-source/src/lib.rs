use chrono::NaiveDate;
use log::{debug, trace};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use payroll_domain::{EmployeeId, MemberId};
use tx_app::{Transaction, TxSource};
use tx_factory::TxFactory;

mod parser;

#[derive(Debug, Clone, PartialEq)]
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
    fn get_tx_source(&self) -> Option<Box<dyn Transaction + 'static>> {
        trace!("TextParserTxSource::get_tx_source called");
        self.txs.borrow_mut().pop_front().map(|tx| {
            debug!("tx_src={:?}", tx);
            self.dispatch(tx)
        })
    }
}
