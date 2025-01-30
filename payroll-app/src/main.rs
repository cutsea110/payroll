use getopts::Options;
use log::{debug, error, info, trace};
use std::{env, io::BufReader};

use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::TxApp;
use tx_impl::TxFactoryImpl;

mod reader;
use reader::{EchoReader, InteractReader};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let program = args.get(0).expect("program name");
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            error!("main: error parsing options: {}", f);
            eprintln!("{}", f.to_string());
            print_usage(&program, opts);
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        debug!("main: help flag detected");
        print_usage(&program, opts);
        return Ok(());
    }
    trace!("main: matches.free: {:?}", matches.free);

    let scripte_file = matches.free.get(0);
    debug!("main: scripte_file: {:?}", scripte_file);

    let make_tx_source = |db| {
        trace!("make_tx_source called");
        let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);
        if let Some(file) = scripte_file {
            debug!("make_tx_source: file_path is {}", file);
            let buf = std::fs::File::open(file).expect("open file");
            let reader = Box::new(BufReader::new(buf));
            return TextParserTxSource::new(tx_factory, Box::new(EchoReader::new(reader)));
        }

        debug!("make_tx_source: file_path is None, using stdin");
        TextParserTxSource::new(tx_factory, Box::new(InteractReader::new()))
    };

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
