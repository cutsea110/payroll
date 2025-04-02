use log::info;
use serde_json;
use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use test_runner::{Paycheck, Verify};

fn main() {
    env_logger::init();

    info!("main starting");

    let mut child = Command::new("./target/debug/payroll-app")
        .arg("-q")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start payroll-app");

    let stdin = child.stdin.as_mut().expect("open payroll-app stdin");
    let stdout = child.stdout.take().expect("open payroll-app stdout");
    let mut reader = BufReader::new(stdout);

    let script_file_path = env::args().nth(1).expect("script file path is required");
    let text = fs::read_to_string(script_file_path).expect("Failed to read script file");

    for line in text.lines() {
        if Verify::is_verify(line) {
            // Payday の標準出力をキャプチャ
            let mut output_json = String::new();
            reader
                .read_line(&mut output_json)
                .expect("read from payroll-app stdout");

            // JSON の検証
            let expect = match Verify::parse(&line) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            };
            let actual: Paycheck = serde_json::from_str(&output_json).expect("parse JSON");
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
        } else {
            writeln!(stdin, "{}", line).expect("write to stdin");
            let mut buff = String::new();
            reader.read_line(&mut buff).expect("read echo back");
            stdin.flush().expect("flush stdin");
        }
    }
    // 終了処理
    let _ = child.wait().expect("wait on child process");
}
