use log::{debug, info, trace};

use tx_app::TxApp;

mod app_config;
mod reader;
mod runner;

use app_config::AppConfig;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    info!("TxApp starting");

    let app_conf = AppConfig::new()?;
    if app_conf.help() {
        debug!("main: help flag is set");
        app_conf.print_usage();
        return Ok(());
    }

    trace!("TxApp running");
    let mut tx_app: TxApp = app_conf.into();
    tx_app.run()?;
    info!("TxApp finished");

    Ok(())
}
