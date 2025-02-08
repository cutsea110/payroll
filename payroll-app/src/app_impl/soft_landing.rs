use log::{error, trace};

use app::Application;

pub(super) struct AppSoftLanding {
    app: Box<dyn Application>,
}
impl AppSoftLanding {
    pub(super) fn new(app: Box<dyn Application>) -> Self {
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
