use getopts::Options;
use log::{error, trace};
use std::env;

pub struct AppConfig {
    help: bool,
    quiet: bool,
    // for each transaction
    transaction_fail_safely: bool,
    // for the whole application
    soft_landing: bool,
    chronograph: bool,
    repl: bool,
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
        opts.optflag("f", "fail-safe-tx", "Transaction fail safely");
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
                error!("parse_args: error parsing options: {}", e);
                return Err(anyhow::Error::msg(e.to_string()));
            }
        };

        Ok(AppConfig {
            help: matches.opt_present("h"),
            quiet: matches.opt_present("q"),
            transaction_fail_safely: matches.opt_present("f"),
            soft_landing: matches.opt_present("s"),
            chronograph: matches.opt_present("c"),
            repl: matches.opt_present("r"),
            program: program.to_string(),
            script_file: matches.free.get(0).cloned(),
            opts,
        })
    }
    pub fn should_show_help(&self) -> bool {
        self.help
    }
    pub fn should_run_quietly(&self) -> bool {
        self.quiet
    }
    pub fn transaction_fail_safely(&self) -> bool {
        self.transaction_fail_safely
    }
    pub fn should_soft_land(&self) -> bool {
        self.soft_landing
    }
    pub fn should_enable_chronograph(&self) -> bool {
        self.chronograph
    }
    pub fn should_dive_into_repl(&self) -> bool {
        self.repl
    }
    pub fn script_file(&self) -> Option<&str> {
        self.script_file.as_deref()
    }
    pub fn help_message(&self) -> String {
        trace!("help_message called");
        let brief = format!("Usage: {} [options] FILE", self.program);
        self.opts.usage(&brief)
    }
}
