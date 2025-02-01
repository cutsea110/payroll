use log::{debug, trace};
use std::time::Instant;
use tx_app::{Response, Runner, Transaction};

// echo back the result of the transaction
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
pub struct TxSilentRunner;
impl Runner for TxSilentRunner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("TxRunner::run called");
        let res = tx.execute()?;
        debug!("TxRunner: tx result={:?}", res);
        Ok(res)
    }
}

// runner decorator that measures the time taken to run the transaction
pub struct TxRunnerChronograph {
    runner: Box<dyn Runner>,
}
impl TxRunnerChronograph {
    pub fn new(runner: Box<dyn Runner>) -> Self {
        Self { runner }
    }
}
impl Runner for TxRunnerChronograph {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("TxRunnerChronograph::run called");
        let start = Instant::now();
        let res = self.runner.run(tx)?;
        let elapsed = start.elapsed();
        println!("TxRunnerChronograph: elapsed={:?}", elapsed);
        Ok(res)
    }
}
