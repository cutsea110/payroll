use log::{debug, info, trace};
use serde_json;
use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use test_runner::{Paycheck, Verify};

const APP_PATH: &str = "./target/debug/payroll-app";

fn main() {
    env_logger::Builder::from_default_env()
        .format_source_path(true)
        .format_line_number(true)
        .init();

    info!("main starting");

    // open test script
    for script_file_path in env::args().skip(1) {
        debug!("script file path: {}", script_file_path);

        eprint!("Running test: {}", script_file_path);

        // run up payroll-app
        let mut child = Command::new(APP_PATH)
            .arg("-qfs")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("start app");
        trace!("app started: PID={}", child.id());

        // pipe of in/out to payroll-app
        let stdin = child.stdin.as_mut().expect("open stdin");
        trace!("stdin opened");
        let stdout = child.stdout.take().expect("open stdout");
        trace!("stdout opened");
        let mut reader = BufReader::new(stdout);

        trace!("script file path: {}", script_file_path);
        let text = fs::read_to_string(&script_file_path).expect("read script");
        trace!("script:\n{}", text);

        // do commands
        for (i, line) in text.lines().enumerate().map(|(i, l)| (i + 1, l)) {
            if Verify::is_verify(line) {
                // Payday の標準出力をキャプチャ
                let mut output_json = String::new();
                reader
                    .read_line(&mut output_json)
                    .expect("read from stdout");
                debug!("test <- app: {}", output_json);

                // JSON の検証
                let expect = match Verify::parse(i, &line) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{}", e);
                        break;
                    }
                };
                let actual: Paycheck = serde_json::from_str(&output_json).expect("parse JSON");
                debug!("expect: {:?}, actual: {:?}", expect, actual);
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
            } else {
                debug!("test -> app: {}", line);
                writeln!(stdin, "{}", line).expect("write to stdin");
                let mut buff = String::new();
                reader.read_line(&mut buff).expect("read echo back");
                debug!("test <- app: {}", buff);
                stdin.flush().expect("flush stdin");
            }
        }

        eprintln!(" ... PASS");
        debug!("test succeeded:{}", script_file_path);
        // 終了処理
        let _ = child.wait().expect("wait on child process");
        debug!("child process exited: {}", child.id());
    }
}
