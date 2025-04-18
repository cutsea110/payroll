use log::trace;
use tx_app::{Response, Runner, Transaction};

// echo back the result of the transaction
pub(super) struct TxEchoBackRunner;
impl Runner for TxEchoBackRunner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error> {
        trace!("run called");
        match tx.execute() {
            Ok(v) => {
                // echo back the result of the transaction
                println!("=> {:?}", v);
                Ok(v)
            }
            Err(e) => {
                // echo back the error of the transaction
                eprintln!("=> {:?}", e);
                Err(e)
            }
        }
    }
}
