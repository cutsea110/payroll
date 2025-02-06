use log::{debug, error, trace};
use std::time::Instant;
use tx_app::{Response, Runner, Transaction};

pub fn echoback_runner() -> Box<dyn Runner> {
    trace!("echoback_runner called");
    Box::new(TxEchoBackRunner)
}
// echo back the result of the transaction
struct TxEchoBackRunner;
impl Runner for TxEchoBackRunner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("TxRunner::run called");
        let res = tx.execute()?;
        debug!("TxRunner: tx result={:?}", res);
        println!("=> {:?}", res);
        Ok(res)
    }
}

pub fn silent_runner() -> Box<dyn Runner> {
    trace!("silent_runner called");
    Box::new(TxSilentRunner)
}
// silently ignore the result of the transaction
struct TxSilentRunner;
impl Runner for TxSilentRunner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("TxRunner::run called");
        let res = tx.execute()?;
        debug!("TxRunner: tx result={:?}", res);
        Ok(res)
    }
}

pub fn with_chronograph(runner: Box<dyn Runner>) -> Box<dyn Runner> {
    trace!("with_chronograph called");
    Box::new(TxRunnerChronograph::new(runner))
}
// runner decorator that measures the time taken to run the transaction
struct TxRunnerChronograph {
    runner: Box<dyn Runner>,
}
impl TxRunnerChronograph {
    fn new(runner: Box<dyn Runner>) -> Self {
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

pub fn with_failsafe(runner: Box<dyn Runner>) -> Box<dyn Runner> {
    trace!("with_fail_safe called");
    Box::new(TxRunnerFailSafe::new(runner))
}
// runner decorator that catches any error and returns Response::Void
struct TxRunnerFailSafe {
    runner: Box<dyn Runner>,
}
impl TxRunnerFailSafe {
    fn new(runner: Box<dyn Runner>) -> Self {
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
