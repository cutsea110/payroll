use log::{debug, info};
use std::env;

use test_runner::TestRunner;

fn main() {
    env_logger::Builder::from_default_env()
        .format_source_path(true)
        .format_line_number(true)
        .init();
    let app_path = env::var("APP_PATH").unwrap_or("./target/debug/payroll-app".to_string());

    info!("main starting");

    // skip program name
    for fp in env::args().skip(1) {
        info!("test: {}", fp);
        // OPEN: eprint
        eprint!("Running test {}\t ... ", fp);

        // run up payroll-app
        let runner = TestRunner::new(&app_path);
        let pass = runner.run(&fp);

        // CLOSE: eprintln
        if pass {
            eprintln!("PASS");
            debug!("test passed: {}", fp);
        } else {
            eprintln!("FAIL");
            debug!("test failed: {}", fp);
        }
    }

    info!("main finished");
}
