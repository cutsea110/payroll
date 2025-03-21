use log::{debug, info, trace};

use app::Application;
use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::{Runner, TxApp, TxSource};
use tx_impl::TxFactoryImpl;

use payroll_app::{app_config, app_impl, reader_impl, runner_impl};

// TODO: remove db argument
fn build_tx_app(db: HashDB, conf: &app_config::AppConfig) -> Box<dyn Application> {
    trace!("build_tx_app called");
    let mut tx_app: Box<dyn Application> =
        Box::new(TxApp::new(make_tx_source(db, &conf), make_tx_runner(&conf)));

    if conf.should_soft_land() {
        debug!("build_tx_app: using AppSoftLanding");
        tx_app = app_impl::with_soft_landing(tx_app);
    }
    if conf.should_enable_chronograph() {
        debug!("build_tx_app: using AppChronograph");
        tx_app = app_impl::with_chronograph(tx_app);
    }

    tx_app
}

fn make_tx_runner(conf: &app_config::AppConfig) -> Box<dyn Runner> {
    trace!("make_tx_runner called");
    let mut tx_runner = if conf.should_run_quietly() {
        debug!("make_tx_runner: using silent runner");
        runner_impl::silent_runner()
    } else {
        debug!("make_tx_runner: using echoback runner");
        runner_impl::echoback_runner()
    };

    if conf.transaction_fail_safely() {
        debug!("build_tx_app: runner with failsafe");
        tx_runner = runner_impl::with_failsafe(tx_runner);
    }

    if conf.should_enable_chronograph() {
        debug!("build_tx_app: runner with chronograph");
        tx_runner = runner_impl::with_chronograph(tx_runner);
    }

    tx_runner
}

fn make_tx_source(db: HashDB, conf: &app_config::AppConfig) -> Box<dyn TxSource> {
    trace!("make_tx_source called");

    let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);

    if let Some(file) = conf.script_file() {
        debug!("make_tx_source: file={}", file);
        let mut reader = reader_impl::file_reader(file);
        if !conf.should_run_quietly() {
            debug!("make_tx_source: using EchoReader");
            reader = reader_impl::with_echo(reader);
        }
        if conf.should_dive_into_repl() {
            debug!("make_tx_source: dive into REPL mode after file loaded");
            reader = reader_impl::join(reader, reader_impl::interact_reader());
        }
        return Box::new(TextParserTxSource::new(tx_factory, reader));
    }

    debug!("make_tx_source: file is None, using stdin");
    Box::new(TextParserTxSource::new(
        tx_factory,
        reader_impl::interact_reader(),
    ))
}

fn print_usage(app_conf: &app_config::AppConfig) {
    trace!("print_usage called");
    println!("{}", app_conf.help_message());
}

fn print_header(_app_conf: &app_config::AppConfig) {
    trace!("print_header called");
    // this banner generated by using 'figlet -f slant payroll'
    let lines = vec![
        r#"                                  ____"#,
        r#"    ____  ____ ___  ___________  / / /"#,
        r#"   / __ \/ __ `/ / / / ___/ __ \/ / / "#,
        r#"  / /_/ / /_/ / /_/ / /  / /_/ / / /  "#,
        r#" / .___/\__,_/\__, /_/   \____/_/_/   "#,
        r#"/_/          /____/                   "#,
        r#""#,
        r#"When you quit, you press Ctrl-D."#,
    ];
    eprintln!("{}", lines.join("\n"));
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_default_env()
        .format_source_path(true)
        .format_line_number(true)
        .init();

    info!("main starting");

    let app_conf = app_config::AppConfig::new()?;
    debug!("main: app_conf={:#?}", app_conf);
    if app_conf.should_show_help() {
        debug!("main: help flag is set");
        print_usage(&app_conf);
        return Ok(());
    }
    if !app_conf.should_run_quietly() {
        debug!("main: header");
        print_header(&app_conf);
    }

    let db = HashDB::new();

    trace!("TxApp running");
    let mut tx_app = build_tx_app(db.clone(), &app_conf);
    tx_app.run()?;
    trace!("TxApp finished");

    if !app_conf.should_run_quietly() {
        debug!("main: printing db at last");
        // this is just for developer
        println!("{:#?}", db);
    }
    info!("main finished");

    Ok(())
}
