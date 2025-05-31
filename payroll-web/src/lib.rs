use log::trace;

use app::Application;
use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::TxApp;
use tx_app_impl::{reader_impl, runner_impl};
use tx_impl::TxFactoryImpl;

pub struct AppConfig {
    request_body: String,
}
impl AppConfig {
    pub fn new(req: &str) -> Result<Self, anyhow::Error> {
        Ok(AppConfig {
            request_body: req.to_string(),
        })
    }
    pub fn build_tx_app(&self, db: HashDB) -> Box<dyn Application> {
        trace!("build_tx_app called");

        let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);
        let tx_source = TextParserTxSource::new(
            tx_factory,
            reader_impl::string_reader(self.request_body.clone()),
        );
        let tx_app: Box<dyn Application> = Box::new(TxApp::new(
            Box::new(tx_source),
            runner_impl::silent_runner(),
        ));

        tx_app
    }
}
