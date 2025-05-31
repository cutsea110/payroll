use log::trace;
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
};

pub fn string_reader(text: String) -> Box<dyn BufRead> {
    let cursor = Cursor::new(text.into_bytes());
    Box::new(BufReader::new(cursor))
}

pub fn file_reader(file: &str) -> Box<dyn BufRead> {
    trace!("file_reader called");
    let buf = File::open(file).expect("open file");
    Box::new(BufReader::new(buf))
}

pub fn interact_reader() -> Box<dyn BufRead> {
    trace!("interact_reader called");
    with_echo(stdin_reader())
}

mod stdin_reader;
pub fn stdin_reader() -> Box<dyn BufRead> {
    trace!("stdin_reader called");
    Box::new(stdin_reader::StdinReader::new())
}

mod echo_reader;
pub fn with_echo(reader: Box<dyn BufRead>) -> Box<dyn BufRead> {
    trace!("with_echo called");
    Box::new(echo_reader::EchoReader::new(reader))
}

mod reader_join;
pub fn join(reader1: Box<dyn BufRead>, reader2: Box<dyn BufRead>) -> Box<dyn BufRead> {
    trace!("join called");
    Box::new(reader_join::ReaderJoin::join(reader1, reader2))
}
