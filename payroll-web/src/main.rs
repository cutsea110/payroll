use log::{debug, error, info, trace};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    str,
    sync::Arc,
};

use hs_db::HashDB;
use threadpool::ThreadPool;

#[derive(Debug, Clone)]
struct Handler {
    db: HashDB,
}
impl Handler {
    fn new(db: HashDB) -> Self {
        Self { db }
    }
    fn handle_connection(self: Arc<Self>, mut stream: TcpStream) {
        trace!("Handling connection from {}", stream.peer_addr().unwrap());
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("read from stream");
        // TODO: check Content-Length header for larger requests
        let text = match str::from_utf8(&buffer) {
            Ok(v) => v.trim_end_matches('\0'),
            Err(e) => {
                error!("Invalid UTF-8 sequence: {}", e);
                return;
            }
        };
        let mut split = text.splitn(2, "\r\n\r\n");

        let header = split.next().unwrap_or_default();
        debug!("Received header:\n{}", header);
        let body = split.next().unwrap_or_default();
        debug!("Received body:\n{}", body);

        let mut tx_app = payroll_web::build_tx_app(self.db.clone(), body);
        let response = match tx_app.run() {
            Ok(_) => {
                trace!("Transaction app ran successfully");
                "HTTP/1.1 200 OK\r\n\r\n"
            }
            Err(e) => {
                error!("Error running transaction app: {}", e);
                "HTTP/1.1 500 Server Error\r\n\r\n"
            }
        };

        stream.write(response.as_bytes()).expect("write to stream");
        stream.flush().expect("flush stream");
    }
}

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
    let handler = Arc::new(Handler::new(HashDB::new()));
    let listener = TcpListener::bind(&app_conf.sock_addr())
        .expect(&format!("Bind to {}", app_conf.sock_addr()));

    for stream in listener.incoming() {
        trace!("Incoming connection");
        let stream = stream.expect("accept connection");
        let handler = Arc::clone(&handler);

        pool.execute(move || {
            handler.handle_connection(stream);
        });
    }

    info!("Shutting down.");
    Ok(())
}
