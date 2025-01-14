fn main() -> Result<(), anyhow::Error> {
    use log::info;
    use std::fs;

    use hs_db::HashDB;
    use text_parser_tx_source::TextParserTxSource;
    use tx_app::TxApp;
    use tx_factory::TxFactoryImpl;
    use tx_impl::{
        AddCommissionedEmployeeTx, AddHourlyEmployeeTx, AddSalariedEmployeeTx, AddSalesReceiptTx,
        AddServiceChargeTx, AddTimeCardTx, ChangeCommissionedTx, ChangeDirectTx,
        ChangeEmployeeAddressTx, ChangeEmployeeNameTx, ChangeHoldTx, ChangeHourlyTx, ChangeMailTx,
        ChangeMemberTx, ChangeNoMemberTx, ChangeSalariedTx, DeleteEmployeeTx, PaydayTx,
    };

    info!("TxApp starting");
    env_logger::init();

    let db = HashDB::new();
    info!("DB initialized: {:?}", db);

    // テストスクリプトを読み込んでシナリオを実行
    let input = fs::read_to_string("script/test.scr")?;
    let tx_source = TextParserTxSource::new(&input);
    let tx_factory = TxFactoryImpl {
        add_hourly_employee: &|id, name, address, hourly_rate| {
            Box::new(AddHourlyEmployeeTx::new(
                id,
                name,
                address,
                hourly_rate,
                db.clone(),
            ))
        },
        add_salaried_employee: &|id, name, address, salary| {
            Box::new(AddSalariedEmployeeTx::new(
                id,
                name,
                address,
                salary,
                db.clone(),
            ))
        },
        add_commissioned_employee: &|id, name, address, salary, commission_rate| {
            Box::new(AddCommissionedEmployeeTx::new(
                id,
                name,
                address,
                salary,
                commission_rate,
                db.clone(),
            ))
        },
        delete_employee: &|id| Box::new(DeleteEmployeeTx::new(id, db.clone())),
        add_timecard: &|id, date, hours| Box::new(AddTimeCardTx::new(id, date, hours, db.clone())),
        add_sales_receipt: &|id, date, amount| {
            Box::new(AddSalesReceiptTx::new(id, date, amount, db.clone()))
        },
        add_service_charge: &|member_id, date, charge| {
            Box::new(AddServiceChargeTx::new(member_id, date, charge, db.clone()))
        },
        change_employee_name: &|id, new_name| {
            Box::new(ChangeEmployeeNameTx::new(id, new_name, db.clone()))
        },
        change_employee_address: &|id, new_address| {
            Box::new(ChangeEmployeeAddressTx::new(id, new_address, db.clone()))
        },
        change_employee_hourly: &|id, hourly_rate| {
            Box::new(ChangeHourlyTx::new(id, hourly_rate, db.clone()))
        },
        change_employee_salaried: &|id, salary| {
            Box::new(ChangeSalariedTx::new(id, salary, db.clone()))
        },
        change_employee_commissioned: &|id, salary, commission_rate| {
            Box::new(ChangeCommissionedTx::new(
                id,
                salary,
                commission_rate,
                db.clone(),
            ))
        },
        change_employee_hold: &|id| Box::new(ChangeHoldTx::new(id, db.clone())),
        change_employee_direct: &|id, bank, account| {
            Box::new(ChangeDirectTx::new(id, bank, account, db.clone()))
        },
        change_employee_mail: &|id, address| Box::new(ChangeMailTx::new(id, address, db.clone())),
        change_employee_member: &|emp_id, member_id, dues| {
            Box::new(ChangeMemberTx::new(member_id, emp_id, dues, db.clone()))
        },
        change_employee_no_member: &|id| Box::new(ChangeNoMemberTx::new(id, db.clone())),
        payday: &|date| Box::new(PaydayTx::new(date, db.clone())),
    };
    let tx_app = TxApp::new(tx_source, tx_factory);

    info!("TxApp starting");
    tx_app.run()?;
    info!("TxApp finished");

    println!("{:#?}", db);
    info!("TxApp finished");

    Ok(())
}
