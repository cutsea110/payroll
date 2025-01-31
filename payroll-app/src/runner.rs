use log::{debug, trace};
use tx_app::{Response, Runner, Transaction};

pub struct TxEchoBachRunner;
impl Runner for TxEchoBachRunner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("TxRunner::run called");
        let res = tx.execute()?;
        debug!("TxRunner: tx result={:?}", res);
        println!("=> {:?}", res);
        Ok(res)
    }
}
