use log::{debug, error, trace};

use app::Application;

pub fn with_chronograph(app: Box<dyn Application>) -> Box<dyn Application> {
    trace!("with_chronograph called");
    Box::new(AppChronograph::new(app))
}

struct AppChronograph {
    app: Box<dyn Application>,
}
impl AppChronograph {
    fn new(app: Box<dyn Application>) -> Self {
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

pub fn with_soft_landing(app: Box<dyn Application>) -> Box<dyn Application> {
    trace!("with_soft_landing called");
    Box::new(AppSoftLanding::new(app))
}

struct AppSoftLanding {
    app: Box<dyn Application>,
}
impl AppSoftLanding {
    fn new(app: Box<dyn Application>) -> Self {
        Self { app }
    }
}
impl Application for AppSoftLanding {
    fn run(&mut self) -> Result<(), anyhow::Error> {
        trace!("AppSoftLanding::run called");
        let result = self.app.run();
        if let Err(e) = result {
            error!("AppSoftLanding: Error={:?}", e);
            eprintln!("AppSoftLanding: Error={:?}", e);
            return Ok(());
        }
        result
    }
}
