use log::{debug, trace};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

mod model;
mod parser;

use model::{Paycheck, Verify};

pub struct TestRunner {
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<ChildStdout>,
}
impl TestRunner {
    pub fn new(app_path: &str) -> Self {
        trace!("spawn_target called");
        let mut child = Command::new(app_path)
            .arg("-qfs")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("start child process");
        trace!("app started: pid={}", child.id());

        // pipe of in/out to payroll-app
        let stdin = child.stdin.take().expect("open stdin");
        trace!("stdin opened");
        let stdout = child.stdout.take().expect("open stdout");
        trace!("stdout opened");
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
    pub fn run(mut self, script_file_path: &str) -> bool {
        trace!("script file path: {}", script_file_path);
        match fs::exists(script_file_path) {
            Ok(true) => debug!("script file exists"),
            Ok(false) => {
                eprintln!("script file not found: {}", script_file_path);
                return false;
            }
            Err(e) => {
                eprintln!("script file error: {}", e);
                return false;
            }
        }

        let text = fs::read_to_string(&script_file_path).expect("read script file");

        let mut result = true;

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
                        result = false;
                        eprintln!("{}", e);
                        break;
                    }
                };
                let actual: Paycheck = serde_json::from_str(&output_json).expect("parse JSON");
                self.assert(actual, expect);
            } else {
                self.write_line(line);
                let mut buff = String::new();
                self.read_line(&mut buff);
            }
        }

        trace!("stdin closing ...");
        // After all commands are executed, close the standard output
        // This will cause the child process to receive EOF and exit
        // If this is not done, the child process will not exit
        drop(self.stdin);
        trace!("stdin dropped and waiting for child process stoped ...");
        self.child.wait().expect("wait for child process");

        result
    }
}
