use log::{debug, trace};
use tx_app::{Response, Runner, Transaction};

// echo back the result of the transaction
#[derive(Debug, Clone)]
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

// silently ignore the result of the transaction
#[derive(Debug, Clone)]
pub struct TxSilentRunner;
impl Runner for TxSilentRunner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("TxRunner::run called");
        let res = tx.execute()?;
        debug!("TxRunner: tx result={:?}", res);
        Ok(res)
    }
}
