use log::trace;
use tx_app::Runner;

mod echoback_runner;
pub fn echoback_runner() -> Box<dyn Runner> {
    trace!("echoback_runner called");
    Box::new(echoback_runner::TxEchoBackRunner)
}

mod silent_runner;
pub fn silent_runner() -> Box<dyn Runner> {
    trace!("silent_runner called");
    Box::new(silent_runner::TxSilentRunner)
}

mod runner_chronograph;
pub fn with_chronograph(runner: Box<dyn Runner>) -> Box<dyn Runner> {
    trace!("with_chronograph called");
    Box::new(runner_chronograph::TxRunnerChronograph::new(runner))
}

mod runner_failsafe;
pub fn with_failsafe(runner: Box<dyn Runner>) -> Box<dyn Runner> {
    trace!("with_fail_safe called");
    Box::new(runner_failsafe::TxRunnerFailSafe::new(runner))
}
