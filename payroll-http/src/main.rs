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

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let handler = Arc::new(Handler);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let handler = Arc::clone(&handler);

        pool.execute(move || {
            handler.handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
