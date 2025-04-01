use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("./target/debug/payroll-app")
        .arg("-q")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start payroll-app");

    let to_app = child.stdin.as_mut().expect("Failed to open stdin");
    let from_app = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(from_app);

    let script = vec![
        r#"AddEmp 1429 "Bob" "Home" S 3215.88"#,
        r#"Payday 2025-03-31"#,
        r#"Verify Paycheck 2025-03-31 EmpId 1429 GrossPay 3215.88"#,
    ];

    for line in script {
        if line.starts_with("Verify") {
            // Payday の標準出力をキャプチャ
            let mut output_json = String::new();
            reader
                .read_line(&mut output_json)
                .expect("Failed to read stdout");

            // JSON の検証
            let expect = r#"{"emp_id":1429,"name":"Bob","gross_pay":3215.88,"deductions":0.0,"net_pay":3215.88}"#;
            let actual = output_json.trim();
            assert_eq!(actual, expect, "Verified");
            eprintln!("Verified");
        } else {
            writeln!(to_app, "{}", line).expect("Failed to write to stdin");
            let mut buff = String::new();
            reader.read_line(&mut buff).expect("read echo back");
            to_app.flush().expect("Failed to flush stdin");
        }
    }

    // 終了処理
    let _ = child.wait().expect("Failed to wait on child process");
}
