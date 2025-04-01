use log::{debug, info, trace};
use serde::Deserialize;
use serde_json;
use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Paycheck {
    emp_id: u32,
    gross_pay: f32,
    deductions: f32,
    net_pay: f32,
}

#[derive(Debug, Clone, PartialEq)]
enum Verify {
    GrossPay { emp_id: u32, gross_pay: f32 },
    Deductions { emp_id: u32, deductions: f32 },
    NetPay { emp_id: u32, net_pay: f32 },
}
impl Verify {
    fn parse(line: &str) -> Result<Self, ()> {
        trace!("parse called");
        debug!("parse: line={}", line);

        Ok(Verify::GrossPay {
            emp_id: 1429,
            gross_pay: 3215.88,
        })
    }
}

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
    let lines: Vec<String> = text.lines().map(Into::into).collect();

    for line in lines {
        if line.starts_with("Verify") {
            // Payday の標準出力をキャプチャ
            let mut output_json = String::new();
            reader
                .read_line(&mut output_json)
                .expect("read from payroll-app stdout");

            // JSON の検証
            let expect = Verify::parse(&line).expect("convert verify");
            let actual: Paycheck = serde_json::from_str(&output_json).expect("parse JSON");
            match expect {
                Verify::GrossPay { emp_id, gross_pay } => {
                    assert_eq!(actual.emp_id, emp_id);
                    assert_eq!(actual.gross_pay, gross_pay);
                }
                _ => todo!(),
            }
        } else {
            writeln!(stdin, "{}", line).expect("Failed to write to stdin");
            let mut buff = String::new();
            reader.read_line(&mut buff).expect("read echo back");
            stdin.flush().expect("Failed to flush stdin");
        }
    }

    // 終了処理
    let _ = child.wait().expect("Failed to wait on child process");
}
