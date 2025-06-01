use log::trace;

use app::Application;
use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::TxApp;
use tx_app_impl::{reader_impl, runner_impl};
use tx_impl::TxFactoryImpl;

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
