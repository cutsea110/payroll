use getopts::Options;
use log::{debug, error, trace};
use std::{env, fmt, io::prelude::*, net::TcpStream, str, sync::Arc};

use app::Application;
use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::{Runner, TxApp, TxSource};
use tx_app_impl::{app_impl, reader_impl, runner_impl};
use tx_impl::TxFactoryImpl;

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
    pub fn is_quiet(&self) -> bool {
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
        let mut handler: Arc<dyn Handler + Send + Sync> = Arc::new(TcpHandler::new(db, self));
        if self.chronograph {
            debug!("Adding chronograph to handler");
            handler = with_chronograph(handler);
        };
        debug!("Handler built");

        handler
    }
}

pub trait Handler {
    fn handle_connection(&self, stream: TcpStream);
}

#[derive(Debug, Clone)]
struct TxAppBuilder {
    db: HashDB,

    quiet: bool,
    chronograph: bool,
}
impl TxAppBuilder {
    fn new(db: HashDB, app_conf: &AppConfig) -> Self {
        Self {
            db,
            quiet: app_conf.is_quiet(),
            chronograph: app_conf.chronograph(),
        }
    }

    pub fn build(&self, request_body: &str) -> Box<dyn Application> {
        trace!("build_tx_app called");

        let tx_source = self.make_tx_source(request_body);
        let tx_runner = self.make_tx_runner();

        let mut tx_app: Box<dyn Application> = Box::new(TxApp::new(tx_source, tx_runner));
        if self.chronograph {
            debug!("Adding fail-open mode");
            tx_app = app_impl::with_chronograph(tx_app);
        }

        tx_app
    }
    fn make_tx_runner(&self) -> Box<dyn Runner> {
        trace!("make_tx_runner called");

        let mut tx_runner = if self.quiet {
            debug!("Quiet mode enabled");
            runner_impl::silent_runner()
        } else {
            debug!("Echoback mode enabled");
            runner_impl::echoback_runner()
        };
        if self.chronograph {
            debug!("Chronograph mode enabled");
            tx_runner = runner_impl::with_chronograph(tx_runner);
        };

        tx_runner
    }
    fn make_tx_source(&self, body: &str) -> Box<dyn TxSource> {
        trace!("make_tx_source called");

        let tx_factory = TxFactoryImpl::new(self.db.clone(), PayrollFactoryImpl);
        let tx_source =
            TextParserTxSource::new(tx_factory, reader_impl::string_reader(body.to_string()));

        Box::new(tx_source)
    }
}

#[derive(Debug, Clone)]
struct TcpHandler {
    builder: TxAppBuilder,
}
impl TcpHandler {
    fn new(db: HashDB, app_conf: &AppConfig) -> Self {
        Self {
            builder: TxAppBuilder::new(db, app_conf),
        }
    }
}
impl Handler for TcpHandler {
    fn handle_connection(&self, mut stream: TcpStream) {
        trace!("Handling connection from {}", stream.peer_addr().unwrap());
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("read from stream");
        // TODO: check Content-Length header for larger requests
        let text = match str::from_utf8(&buffer) {
            Ok(v) => v.trim_end_matches('\0'),
            Err(e) => {
                error!("Invalid UTF-8 sequence: {}", e);
                return;
            }
        };
        let mut split = text.splitn(2, "\r\n\r\n");

        let header = split.next().unwrap_or_default();
        debug!("Received header:\n{}", header);
        let body = split.next().unwrap_or_default();
        debug!("Received body:\n{}", body);

        let mut tx_app = self.builder.build(body);
        let response = match tx_app.run() {
            Ok(_) => {
                trace!("Transaction app ran successfully");
                "HTTP/1.1 200 OK\r\n\r\n"
            }
            Err(e) => {
                error!("Error running transaction app: {}", e);
                "HTTP/1.1 500 Server Error\r\n\r\n"
            }
        };

        stream.write(response.as_bytes()).expect("write to stream");
        stream.flush().expect("flush stream");
    }
}

#[derive(Clone)]
pub struct ChronographHandler {
    handler: Arc<dyn Handler + Send + Sync>,
}
impl ChronographHandler {
    fn new(handler: Arc<dyn Handler + Send + Sync>) -> Self {
        Self { handler }
    }
}
impl Handler for ChronographHandler {
    fn handle_connection(&self, stream: TcpStream) {
        trace!("handle_connection called");
        let start = std::time::Instant::now();

        self.handler.handle_connection(stream);

        let elapsed = start.elapsed();
        debug!("handler elapsed: {:?}", elapsed);
        println!("handler elapsed: {:?}", elapsed);
    }
}
pub fn with_chronograph(handler: Arc<dyn Handler + Send + Sync>) -> Arc<dyn Handler + Send + Sync> {
    trace!("with_chronograph called");
    Arc::new(ChronographHandler::new(handler))
}
