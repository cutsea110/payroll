use log::trace;
use std::time::Instant;
use tx_app::{Response, Runner, Transaction};

// runner decorator that measures the time taken to run the transaction
pub(super) struct TxRunnerChronograph {
    runner: Box<dyn Runner>,
}
impl TxRunnerChronograph {
    pub(super) fn new(runner: Box<dyn Runner>) -> Self {
        Self { runner }
    }
}
impl Runner for TxRunnerChronograph {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("run called");
        let start = Instant::now();
        let res = self.runner.run(tx);
        let elapsed = start.elapsed();
        println!("elapsed={:?}", elapsed);
        res
    }
}
