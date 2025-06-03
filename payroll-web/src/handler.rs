use log::{debug, error, trace};
use std::{io::prelude::*, net::TcpStream, str, sync::Arc};

use crate::tx_app_builder::TxAppBuilder;

pub trait Handler {
    fn handle_connection(&self, stream: TcpStream);
}

#[derive(Debug, Clone)]
pub struct TcpHandler {
    builder: TxAppBuilder,
}
impl TcpHandler {
    pub fn new(builder: TxAppBuilder) -> Self {
        Self { builder }
    }
}
impl Handler for TcpHandler {
    fn handle_connection(&self, mut stream: TcpStream) {
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

        let mut tx_app = self.builder.build(body);
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
        trace!("sent response: {}", response);

        stream.write(response.as_bytes()).expect("write to stream");
        stream.flush().expect("flush stream");
    }
}

#[derive(Clone)]
pub struct ChronographHandler {
    handler: Arc<dyn Handler + Send + Sync>,
}
impl ChronographHandler {
    fn new(handler: Arc<dyn Handler + Send + Sync>) -> Self {
        Self { handler }
    }
}
impl Handler for ChronographHandler {
    fn handle_connection(&self, stream: TcpStream) {
        trace!("handle_connection called");
        let start = std::time::Instant::now();

        self.handler.handle_connection(stream);

        let elapsed = start.elapsed();
        debug!("handler elapsed: {:?}", elapsed);
        println!("handler elapsed: {:?}", elapsed);
    }
}
pub fn with_chronograph(handler: Arc<dyn Handler + Send + Sync>) -> Arc<dyn Handler + Send + Sync> {
    trace!("with_chronograph called");
    Arc::new(ChronographHandler::new(handler))
}
