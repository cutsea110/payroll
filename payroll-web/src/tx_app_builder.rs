use log::{debug, trace};

use app::Application;
use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use std::str;
use text_parser_tx_source::TextParserTxSource;
use tx_app::{Runner, TxApp, TxSource};
use tx_app_impl::{app_impl, reader_impl, runner_impl};
use tx_impl::TxFactoryImpl;

#[derive(Debug, Clone)]
pub struct TxAppBuilder {
    db: HashDB,

    quiet: bool,
    chronograph: bool,
}
impl TxAppBuilder {
    pub fn new(db: HashDB, quiet: bool, chronograph: bool) -> Self {
        Self {
            db,
            quiet,
            chronograph,
        }
    }

    pub fn build(&self, request_body: &str) -> Box<dyn Application> {
        trace!("build_tx_app called");
        let mut tx_app: Box<dyn Application> = Box::new(TxApp::new(
            self.make_tx_source(request_body),
            self.make_tx_runner(),
        ));
        if self.chronograph {
            debug!("Adding fail-open mode");
            tx_app = app_impl::with_chronograph(tx_app);
        }

        tx_app
    }

    fn make_tx_source(&self, body: &str) -> Box<dyn TxSource> {
        trace!("make_tx_source called");
        let tx_factory = TxFactoryImpl::new(self.db.clone(), PayrollFactoryImpl);

        Box::new(TextParserTxSource::new(
            tx_factory,
            reader_impl::string_reader(body.to_string()),
        ))
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
}
