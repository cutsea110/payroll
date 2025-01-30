use log::debug;
use std::io::{stdin, stdout, BufRead, Read, StdinLock, Write};

pub struct StdinReader {
    stdin: StdinLock<'static>,
}
impl StdinReader {
    pub fn new() -> Self {
        Self {
            stdin: stdin().lock(),
        }
    }
}
impl Read for StdinReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stdin.read(buf)
    }
}
impl BufRead for StdinReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.stdin.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.stdin.consume(amt)
    }
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.stdin.read_until(byte, buf)
    }
    // read_line is the only method that needs to customize the behavior
    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        print!("> ");
        stdout().flush().unwrap();
        let line = self.stdin.read_line(buf);
        debug!("Read line: {:?}", buf);
        println!("<= {}", buf);
        line
    }
}
