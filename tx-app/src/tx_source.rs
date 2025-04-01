use chrono::NaiveDate;

use crate::tx::Transaction;
use payroll_domain::{EmployeeId, MemberId};

#[derive(Debug, Clone, PartialEq)]
pub enum Tx {
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

pub trait TxSource {
    fn get_tx_source(&mut self) -> Option<Box<dyn Transaction>>;
}
