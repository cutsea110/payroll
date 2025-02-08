use log::{debug, trace};

use app::Application;

pub(super) struct AppChronograph {
    app: Box<dyn Application>,
}
impl AppChronograph {
    pub(super) fn new(app: Box<dyn Application>) -> Self {
        Self { app }
    }
}
impl Application for AppChronograph {
    fn run(&mut self) -> Result<(), anyhow::Error> {
        trace!("AppChronograph::run called");
        let start = std::time::Instant::now();
        self.app.run()?;
        let elapsed = start.elapsed();
        debug!("AppChronograph: Elapsed={:?}", elapsed);
        println!("AppChronograph: Elapsed={:?}", elapsed);
        Ok(())
    }
}
