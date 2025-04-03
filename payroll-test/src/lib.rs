use log::{debug, trace};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio},
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
        trace!("app started: PID={}", child.id());

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
        debug!("test <- app: {}", buf);
    }
    fn write_line(&mut self, line: &str) {
        debug!("test -> app: {}", line);
        writeln!(self.stdin, "{}", line).expect("write line");
    }
    fn assert(&self, actual: Paycheck, expect: Verify) {
        match expect {
            Verify::GrossPay { emp_id, gross_pay } => {
                debug!("verify gross pay");
                assert_eq!(actual.emp_id, emp_id);
                assert_eq!(actual.gross_pay, gross_pay);
            }
            Verify::Deductions { emp_id, deductions } => {
                debug!("verify deductions");
                assert_eq!(actual.emp_id, emp_id);
                assert_eq!(actual.deductions, deductions);
            }
            Verify::NetPay { emp_id, net_pay } => {
                debug!("verify net pay");
                assert_eq!(actual.emp_id, emp_id);
                assert_eq!(actual.net_pay, net_pay);
            }
        }
    }
    pub fn run(&mut self, script_file_path: &str) {
        trace!("script file path: {}", script_file_path);
        let text = fs::read_to_string(&script_file_path).expect("read script file");
        trace!("script:\n{}", text);

        // execute commands
        for (i, line) in text.lines().enumerate().map(|(i, l)| (i + 1, l)) {
            debug!("execute line {}: {}", i, line);
            if Verify::is_verify(line) {
                // Payday の標準出力をキャプチャ
                let mut output_json = String::new();
                self.read_line(&mut output_json);

                // JSON の検証
                let expect: Verify = match Verify::parse(i, line) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{}", e);
                        break;
                    }
                };
                let actual: Paycheck = serde_json::from_str(&output_json).expect("parse JSON");
                debug!("expect: {:?}, actual: {:?}", expect, actual);
                self.assert(actual, expect);
            } else {
                self.write_line(line);
                let mut buff = String::new();
                self.read_line(&mut buff);
            }
        }
    }
    pub fn shutdown(mut self) -> ExitStatus {
        trace!("shutdown called");

        debug!("stdin closing ...");
        // After all commands are executed, close the standard output
        // This will cause the child process to receive EOF and exit
        // If this is not done, the child process will not exit
        drop(self.stdin);
        debug!("stdin dropped and waiting for child process stoped ...");
        let exit_code = self.child.wait().expect("wait for child process");
        trace!("child process({}) exited: {}", self.child.id(), exit_code);

        exit_code
    }
}
