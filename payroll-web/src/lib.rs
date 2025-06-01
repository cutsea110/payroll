use getopts::Options;
use log::{error, trace};
use std::{env, fmt};

use app::Application;
use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::TxApp;
use tx_app_impl::{reader_impl, runner_impl};
use tx_impl::TxFactoryImpl;

pub struct AppConfig {
    help: bool,
    host: String,
    port: u16,
    threads: usize,
    program: String,
    opts: Options,
}
impl fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AppConfig")
            .field("help", &self.help)
            .field("host", &self.host)
            .field("port", &self.port)
            .field("threads", &self.threads)
            .field("program", &self.program)
            .finish()
    }
}
impl AppConfig {
    pub fn new() -> Result<Self, anyhow::Error> {
        let args: Vec<String> = env::args().collect();
        let program = args.get(0).expect("program name");
        let mut opts = Options::new();
        opts.optflag("?", "help", "Print this help menu");
        opts.optopt("h", "host", "hostname or Ip address to connect to", "HOST");
        opts.optopt("p", "port", "port to connect to", "PORT");
        opts.optopt("t", "threads", "number of threadpool size", "THREADS");
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(e) => {
                error!("failed to parse options: {}", e);
                return Err(anyhow::Error::msg(e.to_string()));
            }
        };

        Ok(Self {
            help: matches.opt_present("?"),
            host: matches.opt_str("h").unwrap_or("127.0.0.1".to_string()),
            port: matches
                .opt_str("p")
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),
            threads: matches
                .opt_str("t")
                .and_then(|s| s.parse().ok())
                .unwrap_or(4),
            program: program.to_string(),
            opts,
        })
    }
    pub fn should_show_help(&self) -> bool {
        self.help
    }
    pub fn help_message(&self) -> String {
        trace!("help_message called");
        let brief = format!("Usage: {} [options]", self.program);
        self.opts.usage(&brief)
    }
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn threads(&self) -> usize {
        self.threads
    }
}

pub fn build_tx_app(db: HashDB, request_body: &str) -> Box<dyn Application> {
    trace!("build_tx_app called");

    let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);
    let tx_source = TextParserTxSource::new(
        tx_factory,
        reader_impl::string_reader(request_body.to_string()),
    );
    let tx_app: Box<dyn Application> = Box::new(TxApp::new(
        Box::new(tx_source),
        runner_impl::silent_runner(),
    ));

    tx_app
}
