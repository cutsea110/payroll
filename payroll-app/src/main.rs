use log::info;
use std::fs;

use hs_db::HashDB;
use text_parser_tx_source::TextParserTxSource;
use tx_app::TxApp;
use tx_impl::TxFactoryImpl;

fn main() -> Result<(), anyhow::Error> {
    info!("TxApp starting");
    env_logger::init();

    let db = HashDB::new();
    info!("DB initialized: {:?}", db);
    let tx_factory = TxFactoryImpl::new(db.clone());

    let script_path = "script/test.scr";
    info!("Reading script: {}", script_path);
    let input = fs::read_to_string(script_path)?;
    info!("Parsing script");
    let tx_source = TextParserTxSource::new(&input, tx_factory);
    let tx_app = TxApp::new(tx_source);

    info!("TxApp running");
    tx_app.run()?;

    println!("{:#?}", db);
    info!("TxApp finished");

    Ok(())
}
