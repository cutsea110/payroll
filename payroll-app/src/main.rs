use log::{info, trace};
use std::{
    env,
    fs::File,
    io::{stdin, BufRead, BufReader},
};

use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::TxApp;
use tx_impl::TxFactoryImpl;

fn main() -> Result<(), anyhow::Error> {
    info!("TxApp starting");
    env_logger::init();

    let db = HashDB::new();
    trace!("DB initialized: {:?}", db);
    let tx_factory = TxFactoryImpl::new(db.clone(), PayrollFactoryImpl);

    let buf_reader: Box<dyn BufRead> = {
        if let Some(script_path) = env::args().nth(1) {
            trace!("script_path={}", script_path);
            let script = File::open(script_path.clone())?;
            Box::new(BufReader::new(script))
        } else {
            let stdin = stdin();
            Box::new(stdin.lock())
        }
    };

    let tx_source = TextParserTxSource::new(tx_factory, buf_reader);
    let mut tx_app = TxApp::new(tx_source);

    trace!("TxApp running");
    tx_app.run()?;

    println!("{:#?}", db);
    info!("TxApp finished");

    Ok(())
}
