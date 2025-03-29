use log::{error, trace};
use tx_app::{Response, Runner, Transaction};

// runner decorator that catches any error and returns Response::Void
pub(super) struct TxRunnerFailOpen {
    runner: Box<dyn Runner>,
}
impl TxRunnerFailOpen {
    pub(super) fn new(runner: Box<dyn Runner>) -> Self {
        Self { runner }
    }
}
impl Runner for TxRunnerFailOpen {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("TxRunnerFailOpen::run called");
        self.runner.run(tx).or_else(|e| {
            error!("TxRunnerFailOpen: error={} occurred", e);
            Ok(Response::Void)
        })
    }
}
