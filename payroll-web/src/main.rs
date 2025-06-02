use log::{debug, info, trace};
use std::net::TcpListener;

use hs_db::HashDB;
use threadpool::ThreadPool;

fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_default_env()
        .format_source_path(true)
        .format_line_number(true)
        .init();

    info!("Starting server...");

    let app_conf = payroll_web::AppConfig::new()?;
    debug!("main: app_conf={:#?}", app_conf);
    if app_conf.should_show_help() {
        debug!("main: should show help");
        println!("{}", app_conf.help_message());
        return Ok(());
    }

    let pool = ThreadPool::new(app_conf.threads());
    let handler = app_conf.build_handler(HashDB::new());
    let listener = TcpListener::bind(&app_conf.sock_addr())
        .expect(&format!("Bind to {}", app_conf.sock_addr()));

    for stream in listener.incoming() {
        trace!("Incoming connection");
        let stream = stream.expect("accept connection");
        let handler = handler.clone();

        pool.execute(move || {
            handler.handle_connection(stream);
        });
    }

    info!("Shutting down.");
    Ok(())
}
