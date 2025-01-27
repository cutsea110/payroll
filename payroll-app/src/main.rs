use log::{debug, info, trace};
use std::{
    env,
    io::{stdin, BufReader},
};

use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::TxApp;
use tx_impl::TxFactoryImpl;

fn main() -> Result<(), anyhow::Error> {
    let make_tx_source = |file_path| {
        trace!("make_tx_source called");
        let tx_factory = TxFactoryImpl::new(HashDB::new(), PayrollFactoryImpl);
        if let Some(file) = file_path {
            debug!("make_tx_source: file_path is {}", file);
            let buf = std::fs::File::open(file).expect("open file");
            TextParserTxSource::new(tx_factory, Box::new(BufReader::new(buf)), false)
        } else {
            debug!("make_tx_source: file_path is None, using stdin");
            TextParserTxSource::new(tx_factory, Box::new(stdin().lock()), true)
        }
    };

    info!("TxApp starting");
    env_logger::init();

    let tx_source = make_tx_source(env::args().nth(1));
    let mut tx_app = TxApp::new(tx_source);

    trace!("TxApp running");
    tx_app.run()?;
    info!("TxApp finished");

    Ok(())
}
