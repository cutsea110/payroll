use log::{debug, trace};
use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Read, StdinLock, Write},
};

pub fn file_reader(file: &str) -> Box<dyn BufRead> {
    trace!("file_reader called");
    let buf = File::open(file).expect("open file");
    Box::new(BufReader::new(buf))
}

pub fn interact_reader() -> Box<dyn BufRead> {
    trace!("interact_reader called");
    with_echo(stdin_reader())
}

pub fn stdin_reader() -> Box<dyn BufRead> {
    trace!("stdin_reader called");
    Box::new(StdinReader::new())
}

struct StdinReader {
    stdin: StdinLock<'static>,
}
impl StdinReader {
    fn new() -> Self {
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
        trace!("StdinReader::read_line called");
        print!("> ");
        stdout().flush().unwrap();
        self.stdin.read_line(buf)
    }
}

pub fn with_echo(reader: Box<dyn BufRead>) -> Box<dyn BufRead> {
    trace!("with_echo called");
    Box::new(EchoReader::new(reader))
}

struct EchoReader {
    reader: Box<dyn BufRead>,
}
impl EchoReader {
    fn new(reader: Box<dyn BufRead>) -> Self {
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
        trace!("EchoReader::read_line called");
        let line = self.reader.read_line(buf);
        debug!("read_line: buf is {}", buf);
        println!("Read line: {}", buf.trim());
        line
    }
}

pub fn join(reader1: Box<dyn BufRead>, reader2: Box<dyn BufRead>) -> Box<dyn BufRead> {
    trace!("join called");
    Box::new(ReaderJoin::join(reader1, reader2))
}

struct ReaderJoin {
    hd: Box<dyn BufRead>,
    tl: Vec<Box<dyn BufRead>>,
}
impl ReaderJoin {
    fn new(reader: Box<dyn BufRead>) -> Self {
        Self {
            hd: reader,
            tl: vec![],
        }
    }
    fn add_reader(&mut self, reader: Box<dyn BufRead>) {
        self.tl.push(reader);
    }
    fn join(prelude: Box<dyn BufRead>, reader: Box<dyn BufRead>) -> Self {
        let mut joined_reader = Self::new(prelude);
        joined_reader.add_reader(reader);
        joined_reader
    }
}
impl Read for ReaderJoin {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.hd.read(buf)
    }
}
impl BufRead for ReaderJoin {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.hd.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.hd.consume(amt)
    }
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.hd.read_until(byte, buf)
    }
    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        trace!("ReaderJoin::read_line called");
        match self.hd.read_line(buf) {
            Ok(0) => {
                debug!("ReaderJoin::read_line: current read_line return empty");
                if self.tl.is_empty() {
                    debug!("ReaderJoin::read_line: no more reader to read");
                    Ok(0)
                } else {
                    debug!("ReaderJoin::read_line: switch to next reader");
                    self.hd = self.tl.remove(0);
                    self.read_line(buf)
                }
            }
            Ok(n) => Ok(n),
            Err(e) => Err(e),
        }
    }
}
