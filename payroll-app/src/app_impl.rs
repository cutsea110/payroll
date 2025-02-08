use log::trace;

use app::Application;

mod chronograph;
pub fn with_chronograph(app: Box<dyn Application>) -> Box<dyn Application> {
    trace!("with_chronograph called");
    Box::new(chronograph::AppChronograph::new(app))
}

mod soft_landing;
pub fn with_soft_landing(app: Box<dyn Application>) -> Box<dyn Application> {
    trace!("with_soft_landing called");
    Box::new(soft_landing::AppSoftLanding::new(app))
}
