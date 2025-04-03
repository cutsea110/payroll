use log::{debug, info};
use std::env;

use test_runner::TestRunner;

fn main() {
    env_logger::Builder::from_default_env()
        .format_source_path(true)
        .format_line_number(true)
        .init();

    info!("main starting");

    // skip program name
    for script_file_path in env::args().skip(1) {
        debug!("script file path: {}", script_file_path);
        // OPEN: eprint
        eprint!("Running test {}\t ... ", script_file_path);

        // run up payroll-app
        let runner = TestRunner::new("./target/debug/payroll-app");
        let exit_code = runner.run(&script_file_path);

        // CLOSE: eprintln
        eprintln!("PASS");
        debug!("test succeeded: {} code={}", script_file_path, exit_code);
    }

    info!("main finished");
}
