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

struct Opts {
    help: bool,
    program: String,
    script_file: Option<String>,
    opts: Options,
}
impl Opts {
    fn parse_args() -> Result<Opts, anyhow::Error> {
        let args: Vec<String> = env::args().collect();
        let program = args.get(0).expect("program name");
        let mut opts = Options::new();
        opts.optflag("h", "help", "Print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                error!("parse_args: error parsing options: {}", f);
                return Err(anyhow::Error::msg(f.to_string()));
            }
        };

        Ok(Opts {
            help: matches.opt_present("h"),
            program: program.to_string(),
            script_file: matches.free.get(0).cloned(),
            opts,
        })
    }
}

fn print_usage(opts: Opts) {
    let brief = format!("Usage: {} [options] FILE", opts.program);
    print!("{}", opts.opts.usage(&brief));
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    info!("TxApp starting");

    let opts = Opts::parse_args()?;
    if opts.help {
        debug!("main: help flag is set");
        print_usage(opts);
        return Ok(());
    }

    let make_tx_source = |db| {
        trace!("make_tx_source called");
        let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);
        if let Some(file) = opts.script_file {
            debug!("make_tx_source: file={}", file);
            let buf = std::fs::File::open(file).expect("open file");
            let reader = Box::new(BufReader::new(buf));
            return TextParserTxSource::new(tx_factory, Box::new(EchoReader::new(reader)));
        }

        debug!("make_tx_source: file is None, using stdin");
        TextParserTxSource::new(tx_factory, Box::new(InteractReader::new()))
    };

    let db = HashDB::new();

    let tx_source = make_tx_source(db.clone());
    let mut tx_app = TxApp::new(tx_source);
    trace!("TxApp running");
    tx_app.run()?;
    info!("TxApp finished");

    println!("{:#?}", db);

    Ok(())
}
