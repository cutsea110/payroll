fn main() -> Result<(), anyhow::Error> {
    use log::info;
    use std::fs;

    use hs_db::HashDB;
    use text_parser_tx_source::TextParserTxSource;
    use tx_app::TxApp;
    use tx_impl::TxFactoryImpl;

    info!("TxApp starting");
    env_logger::init();

    let db = HashDB::new();
    info!("DB initialized: {:?}", db);

    // テストスクリプトを読み込んでシナリオを実行
    let input = fs::read_to_string("script/test.scr")?;
    let tx_factory = TxFactoryImpl::new(db.clone());
    let tx_source = TextParserTxSource::new(&input, tx_factory);
    let tx_app = TxApp::new(tx_source);

    info!("TxApp starting");
    tx_app.run()?;
    info!("TxApp finished");

    println!("{:#?}", db);
    info!("TxApp finished");

    Ok(())
}
