use log::{debug, info};
use std::env;

use test_runner::TestRunner;

fn main() {
    env_logger::Builder::from_default_env()
        .format_source_path(true)
        .format_line_number(true)
        .init();
    let target_path = env::var("TARGET_PATH").unwrap_or("./target/debug/payroll-app".to_string());

    info!("main starting");

    // skip program name
    for fp in env::args().skip(1) {
        info!("test: {}", fp);
        // OPEN: eprint
        eprint!("Running test {}\t ... ", fp);

        let runner = TestRunner::new(&target_path);
        let result = runner.run(&fp);

        // CLOSE: eprintln
        eprintln!("{}", result);
        debug!("test {}: {}", result, fp);
    }

    info!("main finished");
}
