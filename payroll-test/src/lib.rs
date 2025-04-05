use log::{debug, trace};
use std::{
    fmt, fs,
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

mod model;
mod parser;

use model::{Paycheck, Verify};

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
        }
    }
    fn read_line(&mut self, buf: &mut String) {
        self.reader.read_line(buf).expect("read line");
        self.stdin.flush().expect("flush");
        trace!("test <- app: {}", buf);
    }
    fn write_line(&mut self, line: &str) {
        trace!("test -> app: {}", line);
        writeln!(self.stdin, "{}", line).expect("write line");
    }
    fn assert(&self, actual: Paycheck, expect: Verify) {
        trace!("expect: {:?}, actual: {:?}", expect, actual);
        match expect {
            Verify::GrossPay { emp_id, gross_pay } => {
                assert_eq!(actual.emp_id, emp_id);
                assert_eq!(actual.gross_pay, gross_pay);
            }
            Verify::Deductions { emp_id, deductions } => {
                assert_eq!(actual.emp_id, emp_id);
                assert_eq!(actual.deductions, deductions);
            }
            Verify::NetPay { emp_id, net_pay } => {
                assert_eq!(actual.emp_id, emp_id);
                assert_eq!(actual.net_pay, net_pay);
            }
        }
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
            if Verify::is_verify(line) {
                // capture stdout of Payday
                let mut output_json = String::new();
                self.read_line(&mut output_json);

                // verify JSON
                let expect: Verify = match Verify::parse(i, line) {
                    Ok(v) => v,
                    Err(e) => {
                        result = TestResult::Fail;
                        eprintln!("{}", e);
                        break;
                    }
                };
                let actual: Paycheck = serde_json::from_str(&output_json).expect("parse JSON");
                self.assert(actual, expect);
            } else {
                // capture Transaction or comment
                self.write_line(line);
                let mut buff = String::new();
                self.read_line(&mut buff);
            }
        }
        // terminate child process
        self.shutdown();

        result
    }
}
