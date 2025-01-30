use log::{debug, info, trace};
use std::{env, io::BufReader};

use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::TxApp;
use tx_impl::TxFactoryImpl;

mod reader;
use reader::{EchoReader, StdinReader};

fn main() -> Result<(), anyhow::Error> {
    let make_tx_source = |db| {
        trace!("make_tx_source called");
        let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);
        if let Some(file) = env::args().nth(1) {
            debug!("make_tx_source: file_path is {}", file);
            let buf = std::fs::File::open(file).expect("open file");
            let reader = Box::new(BufReader::new(buf));
            return TextParserTxSource::new(tx_factory, Box::new(EchoReader::new(reader)));
        }

        debug!("make_tx_source: file_path is None, using stdin");
        let reader = Box::new(StdinReader::new());
        TextParserTxSource::new(tx_factory, Box::new(EchoReader::new(reader)))
    };

    env_logger::init();

    info!("TxApp starting");
    let db = HashDB::new();

    let tx_source = make_tx_source(db.clone());
    let mut tx_app = TxApp::new(tx_source);
    trace!("TxApp running");
    tx_app.run()?;
    info!("TxApp finished");

    println!("{:#?}", db);

    Ok(())
}
