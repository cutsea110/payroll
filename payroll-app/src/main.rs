use getopts::Options;
use log::{debug, error, info, trace};
use std::{
    env,
    io::{BufRead, BufReader},
};

use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::{Runner, TxApp};
use tx_impl::TxFactoryImpl;

mod reader;
mod runner;

use reader::{EchoReader, InteractReader};
use runner::{TxEchoBachRunner, TxSilentRunner};

struct Opts {
    help: bool,
    quiet: bool,
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
        opts.optflag("q", "quiet", "Don't output unnecessary information");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                error!("parse_args: error parsing options: {}", f);
                return Err(anyhow::Error::msg(f.to_string()));
            }
        };

        Ok(Opts {
            help: matches.opt_present("h"),
            quiet: matches.opt_present("q"),
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
            let mut reader: Box<dyn BufRead> = Box::new(BufReader::new(buf));
            if !opts.quiet {
                reader = Box::new(EchoReader::new(reader));
            }
            return TextParserTxSource::new(tx_factory, reader);
        }

        debug!("make_tx_source: file is None, using stdin");
        TextParserTxSource::new(tx_factory, Box::new(InteractReader::new()))
    };

    let db = HashDB::new();

    let tx_source = make_tx_source(db.clone());
    let tx_runner: Box<dyn Runner> = if opts.quiet {
        Box::new(TxSilentRunner)
    } else {
        Box::new(TxEchoBachRunner)
    };
    let mut tx_app = TxApp::new(tx_source, tx_runner);
    trace!("TxApp running");
    tx_app.run()?;
    info!("TxApp finished");

    println!("{:#?}", db);

    Ok(())
}
