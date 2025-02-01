use anyhow::Result;

pub trait Application {
    fn run(&mut self) -> Result<(), anyhow::Error>;
}
