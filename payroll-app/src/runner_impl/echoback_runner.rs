use log::{debug, trace};
use tx_app::{Response, Runner, Transaction};

// echo back the result of the transaction
pub(super) struct TxEchoBackRunner;
impl Runner for TxEchoBackRunner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("TxRunner::run called");
        let res = tx.execute()?;
        debug!("TxRunner: tx result={:?}", res);
        println!("=> {:?}", res);
        Ok(res)
    }
}
