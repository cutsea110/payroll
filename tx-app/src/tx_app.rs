use log::trace;

use crate::tx_runner::Runner;
use crate::tx_source::TxSource;
use app::Application;

pub struct TxApp {
    tx_source: Box<dyn TxSource>,
    runner: Box<dyn Runner>,
}
impl TxApp {
    pub fn new(tx_source: Box<dyn TxSource>, runner: Box<dyn Runner>) -> Self {
        Self { tx_source, runner }
    }
}
impl Application for TxApp {
    fn run(&mut self) -> Result<(), anyhow::Error> {
        trace!("TxApp::run called");
        while let Some(tx) = self.tx_source.get_tx_source() {
            trace!("TxApp: got next tx");
            self.runner.run(tx)?;
        }
        Ok(())
    }
}
