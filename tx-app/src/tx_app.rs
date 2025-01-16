use log::trace;

use crate::tx_source::TxSource;

pub struct TxApp<S>
where
    S: TxSource,
{
    tx_source: S,
}
impl<S> TxApp<S>
where
    S: TxSource,
{
    pub fn new(tx_source: S) -> Self {
        Self { tx_source }
    }
    pub fn run(&self) -> Result<(), anyhow::Error> {
        trace!("TxApp::run called");
        while let Some(tx) = self.tx_source.get_tx_source() {
            tx.execute()?;
        }
        Ok(())
    }
}
