use log::debug;
use std::io::{stdin, stdout, BufRead, Read, StdinLock, Write};

pub struct EchoReader {
    reader: Box<dyn BufRead>,
}
impl EchoReader {
    pub fn new(reader: Box<dyn BufRead>) -> Self {
        Self { reader }
    }
}
impl Read for EchoReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}
impl BufRead for EchoReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.reader.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.reader.read_until(byte, buf)
    }
    // read_line is the only method that needs to customize the behavior
    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let line = self.reader.read_line(buf);
        debug!("read_line: buf is {}", buf);
        println!("Read line: {}", buf.trim());
        line
    }
}

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
        self.stdin.read_line(buf)
    }
}
