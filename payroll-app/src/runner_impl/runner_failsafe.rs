use log::{error, trace};
use tx_app::{Response, Runner, Transaction};

// runner decorator that catches any error and returns Response::Void
pub(super) struct TxRunnerFailSafe {
    runner: Box<dyn Runner>,
}
impl TxRunnerFailSafe {
    pub(super) fn new(runner: Box<dyn Runner>) -> Self {
        Self { runner }
    }
}
impl Runner for TxRunnerFailSafe {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("TxRunnerFailSafe::run called");
        self.runner.run(tx).or_else(|e| {
            error!("TxRunnerFailSafe: error={}", e);
            Ok(Response::Void)
        })
    }
}
