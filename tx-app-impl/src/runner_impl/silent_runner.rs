use log::trace;
use tx_app::{Response, Runner, Transaction};

// silently ignore the result of the transaction
pub(super) struct TxSilentRunner;
impl Runner for TxSilentRunner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("run called");
        tx.execute()
    }
}
