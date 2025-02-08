use log::{debug, trace};
use std::io::{BufRead, Read};

pub(super) struct ReaderJoin {
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
    pub(super) fn join(prelude: Box<dyn BufRead>, reader: Box<dyn BufRead>) -> Self {
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
