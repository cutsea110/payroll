use getopts::Options;
use log::{debug, error, trace};
use std::{env, fmt};

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
    // for each transaction
    transaction_failopen: bool,
    // for the whole application
    soft_landing: bool,
    chronograph: bool,
    repl: bool,
    program: String,
    script_file: Option<String>,
    opts: Options,
}
impl fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AppConfig")
            .field("help", &self.help)
            .field("quiet", &self.quiet)
            .field("transaction_failopen", &self.transaction_failopen)
            .field("soft_landing", &self.soft_landing)
            .field("chronograph", &self.chronograph)
            .field("repl", &self.repl)
            .field("program", &self.program)
            .field("script_file", &self.script_file)
            .finish()
    }
}
impl AppConfig {
    pub fn new() -> Result<Self, anyhow::Error> {
        let args: Vec<String> = env::args().collect();
        let program = args.get(0).expect("program name");
        let mut opts = Options::new();
        opts.optflag("?", "help", "Print this help menu");
        opts.optflag("q", "quiet", "Don't output unnecessary information");
        opts.optflag("f", "failopen-tx", "Transaction failopen");
        opts.optflag("s", "soft-landing", "Soft landing application");
        opts.optflag(
            "c",
            "chronograph",
            "Print the time taken to execute each transaction",
        );
        opts.optflag("r", "repl", "Run into REPL mode");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(e) => {
                error!("failed to parse options: {}", e);
                return Err(anyhow::Error::msg(e.to_string()));
            }
        };

        Ok(AppConfig {
            help: matches.opt_present("?"),
            quiet: matches.opt_present("q"),
            transaction_failopen: matches.opt_present("f"),
            soft_landing: matches.opt_present("s"),
            chronograph: matches.opt_present("c"),
            repl: matches.opt_present("r"),
            program: program.to_string(),
            script_file: matches.free.get(0).cloned(),
            opts,
        })
    }
    pub fn should_show_help(&self) -> bool {
        trace!("showld_show_help called: {}", self.help);
        self.help
    }
    pub fn should_run_quietly(&self) -> bool {
        trace!("should_run_quietly called: {}", self.quiet);
        self.quiet
    }
    pub fn transaction_failopen(&self) -> bool {
        trace!("transaction_failopen called: {}", self.transaction_failopen);
        self.transaction_failopen
    }
    pub fn should_soft_land(&self) -> bool {
        trace!("should_soft_land called: {}", self.soft_landing);
        self.soft_landing
    }
    pub fn should_enable_chronograph(&self) -> bool {
        trace!("should_enable_chronograph called: {}", self.chronograph);
        self.chronograph
    }
    pub fn should_dive_into_repl(&self) -> bool {
        trace!("should_dive_into_repl called: {}", self.repl);
        self.repl
    }
    pub fn script_file(&self) -> Option<&str> {
        trace!("script_file called: {:?}", self.script_file);
        self.script_file.as_deref()
    }
    pub fn help_message(&self) -> String {
        trace!("help_message called");
        let brief = format!("Usage: {} [options] FILE", self.program);
        self.opts.usage(&brief)
    }

    // db is expected to setup or initialized specially for the application or the test case.
    pub fn build_tx_app(&self, db: HashDB) -> Box<dyn Application> {
        trace!("build_tx_app called");
        let mut tx_app: Box<dyn Application> =
            Box::new(TxApp::new(self.make_tx_source(db), self.make_tx_runner()));

        if self.should_soft_land() {
            debug!("build_tx_app: should soft landing, using with_soft_landing");
            tx_app = app_impl::with_soft_landing(tx_app);
        }
        if self.should_enable_chronograph() {
            debug!("build_tx_app: should enable chronograph, using with_chronograph");
            tx_app = app_impl::with_chronograph(tx_app);
        }

        tx_app
    }

    fn make_tx_source(&self, db: HashDB) -> Box<dyn TxSource> {
        trace!("make_tx_source called");
        let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);

        if let Some(file) = self.script_file() {
            debug!("make_tx_source: with file={}, using file_reader", file);
            let mut reader = reader_impl::file_reader(file);
            if !self.should_run_quietly() {
                debug!("make_tx_source: shouldn't run quietly, using echoback_reader");
                reader = reader_impl::with_echo(reader);
            }
            if self.should_dive_into_repl() {
                debug!("make_tx_source: should dive into REPL, using interact_reader");
                reader = reader_impl::join(reader, reader_impl::interact_reader());
            }
            return Box::new(TextParserTxSource::new(tx_factory, reader));
        }

        debug!("make_tx_source: file is None, using stdin");
        Box::new(TextParserTxSource::new(
            tx_factory,
            reader_impl::interact_reader(),
        ))
    }

    fn make_tx_runner(&self) -> Box<dyn Runner> {
        trace!("make_tx_runner called");
        let mut tx_runner = if self.should_run_quietly() {
            debug!("make_tx_runner: should run quietly, using silent_runner");
            runner_impl::silent_runner()
        } else {
            debug!("make_tx_runner: shouldn't run quietly, using echoback_runner");
            runner_impl::echoback_runner()
        };

        if self.transaction_failopen() {
            debug!("make_tx_runner: transaction failopen, using with_failopen");
            tx_runner = runner_impl::with_failopen(tx_runner);
        }

        if self.should_enable_chronograph() {
            debug!("make_tx_runner: should enable chronograph, using with_chronograph");
            tx_runner = runner_impl::with_chronograph(tx_runner);
        }

        tx_runner
    }
}
