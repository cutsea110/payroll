use getopts::Options;
use log::{debug, error, trace};
use std::{env, fmt, sync::Arc};

use hs_db::HashDB;

mod handler;
mod tx_app_builder;

use handler::{with_chronograph, Handler, TcpHandler};

pub struct AppConfig {
    help: bool,
    quiet: bool,
    host: String,
    port: u16,
    threads: usize,
    chronograph: bool,
    program: String,
    opts: Options,
}
impl fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AppConfig")
            .field("help", &self.help)
            .field("quiet", &self.quiet)
            .field("host", &self.host)
            .field("port", &self.port)
            .field("threads", &self.threads)
            .field("chronograph", &self.chronograph)
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
        opts.optflag("q", "quiet", "run in quiet mode, non verbose");
        opts.optopt("p", "port", "port to connect to", "PORT");
        opts.optopt("t", "threads", "number of threadpool size", "THREADS");
        opts.optflag("c", "chronograph", "enable chronograph mode");
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
            quiet: matches.opt_present("q"),
            port: matches
                .opt_str("p")
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),
            threads: matches
                .opt_str("t")
                .and_then(|s| s.parse().ok())
                .unwrap_or(4),
            chronograph: matches.opt_present("c"),
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
    pub fn should_run_quietly(&self) -> bool {
        self.quiet
    }
    pub fn sock_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    pub fn threads(&self) -> usize {
        self.threads
    }
    pub fn chronograph(&self) -> bool {
        self.chronograph
    }
    pub fn build_handler(&self, db: HashDB) -> Arc<dyn Handler + Send + Sync> {
        trace!("build_handler called");
        let builder = tx_app_builder::TxAppBuilder::new(db.clone(), self.quiet, self.chronograph);

        let mut handler: Arc<dyn Handler + Send + Sync> = Arc::new(TcpHandler::new(builder));
        if self.chronograph {
            debug!("adding chronograph to handler");
            handler = with_chronograph(handler);
        };

        handler
    }
}
