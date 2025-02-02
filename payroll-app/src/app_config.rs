use getopts::Options;
use log::error;
use std::env;

pub struct AppConfig {
    help: bool,
    quiet: bool,
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
        opts.optflag(
            "c",
            "chronograph",
            "Print the time taken to execute each transaction",
        );
        opts.optflag("r", "repl", "Run into REPL mode");

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
        let brief = format!("Usage: {} [options] FILE", self.program);
        self.opts.usage(&brief)
    }
}
