use log::{debug, info};
use std::env;

use test_runner::TestRunner;

const APP_PATH: &str = "./target/debug/payroll-app";

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
        let runner = TestRunner::new(APP_PATH);
        let pass = runner.run(&script_file_path);

        // CLOSE: eprintln
        if pass {
            eprintln!("PASS");
            debug!("test passed: {}", script_file_path);
        } else {
            eprintln!("FAIL");
            debug!("test failed: {}", script_file_path);
        }
    }

    info!("main finished");
}
