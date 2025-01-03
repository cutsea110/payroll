use payroll_db;
use service;
use service_impl;

mod payroll_util {
    use chrono::NaiveDate;

    pub fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::payroll_db::PayrollDatabase;
use crate::payroll_util::date;
use crate::service::Transaction;
use crate::service_impl::*;

fn main() {
    env_logger::init();

    let db = Rc::new(RefCell::new(PayrollDatabase::new()));

    let tx: &mut dyn Transaction<T = _> =
        &mut AddSalariedEmployeeTx::new(1, "Bob", "Home", 1000.0, db.clone());
    println!("{:#?}", db);
    tx.execute().expect("register employee Bob");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut ChgEmployeeNameTx::new(1, "Alice", db.clone());
    tx.execute().expect("change employee name");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgEmployeeAddressTx::new(1, "123 Main St.", db.clone());
    tx.execute().expect("change employee address");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut PaydayTx::new(date(2025, 1, 31), db.clone());
    tx.execute().expect("payday");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut ChgHourlyClassificationTx::new(1, 10.0, db.clone());
    tx.execute().expect("change employee to hourly");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddTimecardTx::new(1, date(2025, 1, 1), 8.0, db.clone());
    tx.execute().expect("add timecard");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut PaydayTx::new(date(2025, 1, 3), db.clone());
    tx.execute().expect("payday");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgCommissionedClassificationTx::new(1, 510.0, 0.05, db.clone());
    tx.execute().expect("change employee to commissioned");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddSalesReceiptTx::new(1, date(2025, 1, 1), 35980.0, db.clone());
    tx.execute().expect("add sales receipt");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut PaydayTx::new(date(2025, 1, 10), db.clone());
    tx.execute().expect("payday");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgSalariedClassificationTx::new(1, 1020.0, db.clone());
    tx.execute().expect("change employee to salaried");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgDirectMethodTx::new(1, "mufg", "3-14159265", db.clone());
    tx.execute().expect("change employee to direct method");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgMailMethodTx::new(1, "alice@gmail.com", db.clone());
    tx.execute().expect("change employee to mail method");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut ChgHoldMethodTx::new(1, db.clone());
    tx.execute().expect("change employee to hold method");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut AddUnionMemberTx::new(7463, 1, 100.0, db.clone());
    tx.execute().expect("add union member");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddServiceChargeTx::new(7463, date(2025, 1, 1), 300.5, db.clone());
    tx.execute().expect("add service charge");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut DelUnionMemberTx::new(1, db.clone());
    tx.execute().expect("delete union member");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut DelEmployeeTx::new(1, db.clone());
    tx.execute().expect("delete employee");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddHourlyEmployeeTx::new(2, "Charlie", "Home", 10.0, db.clone());
    tx.execute().expect("register employee Charlie");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddCommissionedEmployeeTx::new(3, "David", "Home", 500.0, 0.5, db.clone());
    tx.execute().expect("register employee David");
    println!("{:#?}", db);
}
