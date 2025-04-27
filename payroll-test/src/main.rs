use log::{debug, info};
use std::env;

use test_runner::TestRunner;

fn main() {
    env_logger::Builder::from_default_env()
        .format_source_path(true)
        .format_line_number(true)
        .init();

    // Expect payroll-app is in the target/debug directory and that built before running this test
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
    let profile = env::var("PROFILE").unwrap_or("debug".to_string());
    let app_path =
        env::var("APP_PATH").unwrap_or(format!("{}/{}/payroll-app", target_dir, profile));
    debug!("payroll-app path: {}", app_path);

    info!("main starting");

    // This 1 means to skip program name
    for fp in env::args().skip(1) {
        info!("test: {}", fp);
        // OPEN: eprint
        eprint!("Running test {}\t ... ", fp);

        let runner = TestRunner::new(&app_path);
        let result = runner.run(&fp);

        // CLOSE: eprintln
        eprintln!("{}", result);
        debug!("test {}: {}", result, fp);
    }

    info!("main finished");
}
