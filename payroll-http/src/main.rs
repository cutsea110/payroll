use log::{debug, error, info, trace};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    str,
    sync::Arc,
};

use app::Application;
use hs_db::HashDB;
use payroll_app::{reader_impl, runner_impl};
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use threadpool::ThreadPool;
use tx_app::TxApp;
use tx_impl::TxFactoryImpl;

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

        let header = split.next().unwrap_or("");
        debug!("Received header:\n{}", header);
        let body = split.next().unwrap_or("");
        debug!("Received body:\n{}", body);

        let tx_factory = TxFactoryImpl::new(self.db.clone(), PayrollFactoryImpl);
        let tx_source =
            TextParserTxSource::new(tx_factory, reader_impl::string_reader(body.to_string()));
        let mut tx_app: Box<dyn Application> = Box::new(TxApp::new(
            Box::new(tx_source),
            runner_impl::silent_runner(),
        ));

        tx_app.run().unwrap_or_else(|e| {
            error!("Error running transaction app: {}", e);
        });

        let response = b"HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response).expect("write to stream");
        stream.flush().expect("flush stream");
    }
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_default_env()
        .format_source_path(true)
        .format_line_number(true)
        .init();

    info!("Starting server...");

    let listener = TcpListener::bind("127.0.0.1:7878").expect("Bind to 127.0.0.1:7878");
    let pool = ThreadPool::new(4);
    let db = HashDB::new();
    let handler = Arc::new(Handler::new(db.clone()));

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
