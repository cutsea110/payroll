use log::{debug, error, info, trace};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    str,
    sync::Arc,
};

use threadpool::ThreadPool;

struct Handler;
impl Handler {
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

        let header = split.next().unwrap_or("");
        debug!("Received header:\n{}", header);
        let body = split.next().unwrap_or("");
        debug!("Received body:\n{}", body);

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
    let handler = Arc::new(Handler);

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
