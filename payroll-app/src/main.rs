use log::{info, trace};
use std::env;

use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::TxApp;
use tx_impl::TxFactoryImpl;

fn main() -> Result<(), anyhow::Error> {
    info!("TxApp starting");
    env_logger::init();

    for script_path in env::args().skip(1) {
        info!("running script: script_path={}", script_path);

        let db = HashDB::new();
        trace!("DB initialized: {:?}", db);
        let tx_factory = TxFactoryImpl::new(db.clone(), PayrollFactoryImpl);

        let tx_source = TextParserTxSource::new(tx_factory);

        trace!("Parsing script and Load");
        tx_source.load_from_script(script_path.clone());
        trace!("Save script as JSON");
        let json_path = script_path.replace(".scr", ".json");
        tx_source.store_to_json(json_path.clone());
        trace!("Clear txs");
        tx_source.clear_txs();
        trace!("Load from JSON");
        tx_source.load_from_json(json_path);
        let tx_app = TxApp::new(tx_source);

        trace!("TxApp running");
        tx_app.run()?;

        println!("{:#?}", db);
    }
    info!("TxApp finished");

    Ok(())
}
