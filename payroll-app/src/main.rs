use log::{debug, info, trace};
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

    let (buf_reader, interact_mode): (Box<dyn BufRead>, bool) = {
        if let Some(script_path) = env::args().nth(1) {
            trace!("script_path={}", script_path);
            let script = File::open(script_path.clone())?;
            (Box::new(BufReader::new(script)), false)
        } else {
            let stdin = stdin();
            (Box::new(stdin.lock()), true)
        }
    };
    debug!("interact={}", interact_mode);

    let tx_source = TextParserTxSource::new(tx_factory, buf_reader, interact_mode);
    let mut tx_app = TxApp::new(tx_source, interact_mode);

    trace!("TxApp running");
    tx_app.run()?;

    println!("{:#?}", db);
    info!("TxApp finished");

    Ok(())
}
