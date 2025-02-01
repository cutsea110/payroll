use log::trace;

use crate::tx::{Response, Transaction};
use crate::tx_source::TxSource;

pub trait Runner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error>;
}

pub struct TxApp<S>
where
    S: TxSource,
{
    tx_source: S,
    runner: Box<dyn Runner>,
}
impl<S> TxApp<S>
where
    S: TxSource,
{
    pub fn new(tx_source: S, runner: Box<dyn Runner>) -> Self {
        Self { tx_source, runner }
    }
    pub fn run(&mut self) -> Result<(), anyhow::Error> {
        trace!("TxApp::run called");
        while let Some(tx) = self.tx_source.get_tx_source() {
            self.runner.run(tx)?;
        }
        Ok(())
    }
}
