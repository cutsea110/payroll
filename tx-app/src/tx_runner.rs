use crate::tx::{Response, Transaction};

pub trait Runner {
    fn run(&self, tx: Box<dyn Transaction>) -> Result<Response, anyhow::Error>;
}
