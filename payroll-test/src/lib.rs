use log::{debug, trace};
use std::{
    collections::HashMap,
    fmt, fs,
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    str,
};

mod model;
mod parser;

use model::{Paycheck, Verify};
use parser::TxType;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TestResult {
    Pass,
    Fail,
}
impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestResult::Pass => write!(f, "PASS"),
            TestResult::Fail => write!(f, "FAIL"),
        }
    }
}

pub struct TestRunner {
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<ChildStdout>,
    // hold latest Payday's Paychecks
    output: HashMap<u32, Paycheck>,
}
impl TestRunner {
    pub fn new(app_path: &str) -> Self {
        let mut child = Command::new(app_path)
            .arg("-qfs")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("start child process");
        trace!("app started: pid={}", child.id());

        // pipe of in/out to payroll-app
        let stdin = child.stdin.take().expect("open stdin");
        let stdout = child.stdout.take().expect("open stdout");
        let reader = BufReader::new(stdout);

        TestRunner {
            child,
            stdin,
            reader,
            output: HashMap::new(),
        }
    }
    fn ready(&mut self) -> bool {
        let buffer = match self.reader.fill_buf() {
            Ok(b) => b,
            Err(e) => {
                debug!("error reading from child process: {}", e);
                return false;
            }
        };
        trace!("peeked: {:?}", str::from_utf8(buffer));
        buffer.contains(&b'\n')
    }
    fn send(&mut self, line: &str) {
        trace!("test -> app: {:?}", line);
        writeln!(self.stdin, "{}", line).expect("write line");
        // consume echo back
        let _ = self.recv();
    }
    fn recv(&mut self) -> String {
        let mut buf = String::new();
        self.reader.read_line(&mut buf).expect("read line");
        self.stdin.flush().expect("flush");
        trace!("test <- app: {:?}", buf);

        buf
    }
    fn try_consume(&mut self) {
        while self.ready() {
            let _ = self.recv();
        }
    }
    fn clear(&mut self) {
        // clear output
        if !self.output.is_empty() {
            debug!("clear output");
            self.output.clear();
        }
    }
    fn try_collect_paychecks(&mut self) {
        while self.ready() {
            let o = self.recv();
            let paycheck: Paycheck = match serde_json::from_str(&o) {
                Ok(p) => p,
                Err(e) => {
                    debug!("not a valid Paycheck JSON: {}", e);
                    continue;
                }
            };
            let emp_id = paycheck.emp_id;
            debug!("insert emp_id: {}, paycheck: {:?}", emp_id, paycheck);
            self.output.insert(emp_id, paycheck);
        }
        trace!("got {} Paychecks", self.output.len());
    }
    fn shutdown(mut self) {
        trace!("shutdown called");
        // After all commands are executed, close the standard output
        // This will cause the child process to receive EOF and exit
        // If this is not done, the child process will not exit
        drop(self.stdin);
        // wait for the child process to exit
        self.child.wait().expect("wait for child process");
    }
    pub fn run(mut self, fp: &str) -> TestResult {
        trace!("script file path: {}", fp);
        if !fs::exists(fp).unwrap_or(false) {
            debug!("script file not found: {}", fp);
            eprintln!("script file not found: {}", fp);
            // terminate child process
            self.shutdown();
            return TestResult::Fail;
        }

        let text = fs::read_to_string(&fp).expect("read script file");

        let mut result = TestResult::Pass;

        // execute commands
        for (i, line) in text.lines().enumerate().map(|(i, l)| (i + 1, l)) {
            trace!("execute line {}: {}", i, line);
            self.try_consume();

            match parser::tx_type(line) {
                TxType::Payday => {
                    trace!("Payday command");
                    self.send(line);
                    //  In case of Payday, collect outputs of Paycheck JSON data.
                    self.try_collect_paychecks();
                }
                TxType::Verify => {
                    trace!("Verify command");
                    let expect: Verify = match Verify::parse(i, line) {
                        Ok(v) => v,
                        Err(e) => {
                            result = TestResult::Fail;
                            eprintln!("{}", e);
                            break;
                        }
                    };
                    expect.verify(&self.output);
                }
                TxType::Other => {
                    trace!("Other command");
                    self.send(line);
                    // In case of other commands, we need to consume the output
                    self.try_consume();
                    // clear output
                    self.clear();
                }
            }
        }
        // terminate child process
        self.shutdown();

        result
    }
}
