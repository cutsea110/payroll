use log::{debug, info, trace};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use threadpool::ThreadPool;

struct Handler;
impl Handler {
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let response = b"HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response).unwrap();
        stream.flush().unwrap();
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
            debug!("Handling connection");
            handler.handle_connection(stream);
        });
    }

    info!("Shutting down.");
    Ok(())
}
