fn main() -> Result<(), anyhow::Error> {
    use log::info;
    use std::fs;

    use hs_db::HashDB;
    use text_parser_tx_source::TextParserTxSource;
    use tx_app::TxApp;
    use tx_factory::TxFactoryImpl;
    use tx_impl::{
        AddCommissionedEmpTx, AddHourlyEmpTx, AddSalariedEmpTx, AddSalesReceiptTx,
        AddServiceChargeTx, AddTimeCardTx, ChgCommissionedTx, ChgDirectTx, ChgEmpAddressTx,
        ChgEmpNameTx, ChgHoldTx, ChgHourlyTx, ChgMailTx, ChgMemberTx, ChgSalariedTx,
        ChgUnaffiliatedTx, DelEmpTx, PaydayTx,
    };

    info!("TxApp starting");
    env_logger::init();

    let db = HashDB::new();
    info!("DB initialized: {:?}", db);

    // テストスクリプトを読み込んでシナリオを実行
    let input = fs::read_to_string("script/test.scr")?;
    let tx_source = TextParserTxSource::new(&input);
    let tx_factory = TxFactoryImpl {
        add_salaried_emp: &|id, name, address, salary| {
            Box::new(AddSalariedEmpTx::new(id, name, address, salary, db.clone()))
        },
        add_hourly_emp: &|id, name, address, hourly_rate| {
            Box::new(AddHourlyEmpTx::new(
                id,
                name,
                address,
                hourly_rate,
                db.clone(),
            ))
        },
        add_commissioned_emp: &|id, name, address, salary, commission_rate| {
            Box::new(AddCommissionedEmpTx::new(
                id,
                name,
                address,
                salary,
                commission_rate,
                db.clone(),
            ))
        },
        del_emp: &|id| Box::new(DelEmpTx::new(id, db.clone())),
        timecard: &|id, date, hours| Box::new(AddTimeCardTx::new(id, date, hours, db.clone())),
        sales_receipt: &|id, date, amount| {
            Box::new(AddSalesReceiptTx::new(id, date, amount, db.clone()))
        },
        chg_emp_name: &|id, new_name| Box::new(ChgEmpNameTx::new(id, new_name, db.clone())),
        chg_emp_address: &|id, new_address| {
            Box::new(ChgEmpAddressTx::new(id, new_address, db.clone()))
        },
        chg_salaried: &|id, salary| Box::new(ChgSalariedTx::new(id, salary, db.clone())),
        chg_hourly: &|id, hourly_rate| Box::new(ChgHourlyTx::new(id, hourly_rate, db.clone())),
        chg_commissioned: &|id, salary, commission_rate| {
            Box::new(ChgCommissionedTx::new(
                id,
                salary,
                commission_rate,
                db.clone(),
            ))
        },
        chg_hold_method: &|id| Box::new(ChgHoldTx::new(id, db.clone())),
        chg_direct_method: &|id, bank, account| {
            Box::new(ChgDirectTx::new(id, bank, account, db.clone()))
        },
        chg_mail_method: &|id, address| Box::new(ChgMailTx::new(id, address, db.clone())),
        add_union_member: &|emp_id, member_id, dues| {
            Box::new(ChgMemberTx::new(member_id, emp_id, dues, db.clone()))
        },
        del_union_member: &|id| Box::new(ChgUnaffiliatedTx::new(id, db.clone())),
        service_charge: &|member_id, date, charge| {
            Box::new(AddServiceChargeTx::new(member_id, date, charge, db.clone()))
        },
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
