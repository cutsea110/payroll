use getopts::Options;
use log::{debug, error, trace};
use std::{
    env,
    io::{BufRead, BufReader},
};

use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::{Runner, TxApp, TxSource};
use tx_impl::TxFactoryImpl;

use crate::reader::{EchoReader, InteractReader};
use crate::runner::{TxEchoBachRunner, TxRunnerChronograph, TxSilentRunner};

pub struct AppConfig {
    help: bool,
    quiet: bool,
    chronograph: bool,
    program: String,
    script_file: Option<String>,
    opts: Options,
}
impl AppConfig {
    pub fn new() -> Result<Self, anyhow::Error> {
        let args: Vec<String> = env::args().collect();
        let program = args.get(0).expect("program name");
        let mut opts = Options::new();
        opts.optflag("h", "help", "Print this help menu");
        opts.optflag("q", "quiet", "Don't output unnecessary information");
        opts.optflag(
            "c",
            "chronograph",
            "Print the time taken to execute each transaction",
        );

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                error!("parse_args: error parsing options: {}", f);
                return Err(anyhow::Error::msg(f.to_string()));
            }
        };

        Ok(AppConfig {
            help: matches.opt_present("h"),
            quiet: matches.opt_present("q"),
            chronograph: matches.opt_present("c"),
            program: program.to_string(),
            script_file: matches.free.get(0).cloned(),
            opts,
        })
    }
    pub fn help(&self) -> bool {
        self.help
    }
    pub fn print_usage(&self) {
        let brief = format!("Usage: {} [options] FILE", self.program);
        print!("{}", self.opts.usage(&brief));
    }
}
impl From<AppConfig> for TxApp {
    fn from(opts: AppConfig) -> Self {
        let db = HashDB::new();

        let tx_source = make_tx_source(db, &opts);
        let mut tx_runner: Box<dyn Runner> = if opts.quiet {
            Box::new(TxSilentRunner)
        } else {
            Box::new(TxEchoBachRunner)
        };
        if opts.chronograph {
            tx_runner = Box::new(TxRunnerChronograph::new(tx_runner));
        }
        TxApp::new(tx_source, tx_runner)
    }
}

fn make_tx_source(db: HashDB, opts: &AppConfig) -> Box<dyn TxSource> {
    trace!("make_tx_source called");
    let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);
    if let Some(file) = opts.script_file.clone() {
        debug!("make_tx_source: file={}", file);
        let buf = std::fs::File::open(file).expect("open file");
        let mut reader: Box<dyn BufRead> = Box::new(BufReader::new(buf));
        if !opts.quiet {
            reader = Box::new(EchoReader::new(reader));
        }
        return Box::new(TextParserTxSource::new(tx_factory, reader));
    }

    debug!("make_tx_source: file is None, using stdin");
    Box::new(TextParserTxSource::new(
        tx_factory,
        Box::new(InteractReader::new()),
    ))
}
