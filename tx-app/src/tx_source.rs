use crate::tx::Transaction;

pub trait TxSource {
    fn get_tx_source(&self) -> Option<Box<dyn Transaction>>;
}
