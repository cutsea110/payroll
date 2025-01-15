use log::trace;

pub struct TxApp<S>
where
    S: TxSource,
{
    tx_source: S,
}
impl<S> TxApp<S>
where
    S: TxSource,
{
    pub fn new(tx_source: S) -> Self {
        Self { tx_source }
    }
    pub fn run(&self) -> Result<(), anyhow::Error> {
        trace!("TxApp::run called");
        while let Some(tx) = self.tx_source.get_tx_source() {
            tx.execute()?;
        }
        Ok(())
    }
}

mod tx {
    use anyhow;

    use payroll_domain::EmployeeId;

    // トランザクションのインターフェース
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Response {
        Void,
        EmployeeId(EmployeeId),
    }
    pub trait Transaction {
        fn execute(&self) -> Result<Response, anyhow::Error>;
    }
}
pub use tx::*;

mod tx_source {
    use super::Transaction;

    pub trait TxSource {
        fn get_tx_source(&self) -> Option<Box<dyn Transaction>>;
    }
}
pub use tx_source::*;
