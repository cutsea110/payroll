use log::{debug, info, trace};

use app::Application;
use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::{TxApp, TxSource};
use tx_impl::TxFactoryImpl;

mod app_config;
mod app_impl;
mod reader_impl;
mod runner_impl;

// TODO: remove db argument
fn build_tx_app(app_conf: &app_config::AppConfig, db: HashDB) -> Box<dyn Application> {
    trace!("build_tx_app called");

    let mut tx_runner = if app_conf.should_run_quietly() {
        debug!("build_tx_app: using silent runner");
        runner_impl::silent_runner()
    } else {
        debug!("build_tx_app: using echoback runner");
        runner_impl::echoback_runner()
    };

    if app_conf.transaction_fail_safely() {
        debug!("build_tx_app: runner with fail-safe");
        tx_runner = runner_impl::with_failsafe(tx_runner);
    }

    if app_conf.should_enable_chronograph() {
        debug!("build_tx_app: runner with chronograph");
        tx_runner = runner_impl::with_chronograph(tx_runner);
    }

    let mut tx_app: Box<dyn Application> =
        Box::new(TxApp::new(make_tx_source(db, &app_conf), tx_runner));

    if app_conf.should_soft_land() {
        debug!("build_tx_app: using AppSoftLanding");
        tx_app = app_impl::with_soft_landing(tx_app);
    }
    if app_conf.should_enable_chronograph() {
        debug!("build_tx_app: using AppChronograph");
        tx_app = app_impl::with_chronograph(tx_app);
    }

    tx_app
}

fn make_tx_source(db: HashDB, conf: &app_config::AppConfig) -> Box<dyn TxSource> {
    trace!("make_tx_source called");

    let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);

    if let Some(file) = conf.script_file().clone() {
        debug!("make_tx_source: file={}", file);
        let mut reader = reader_impl::file_reader(&file);
        if !conf.should_run_quietly() {
            debug!("make_tx_source: using EchoReader");
            reader = reader_impl::with_echo(reader);
        }
        if conf.should_dive_into_repl() {
            debug!("make_tx_source: dive into REPL mode after file loaded");
            reader = reader_impl::join(reader, reader_impl::interact_reader());
            return Box::new(TextParserTxSource::new(tx_factory, reader));
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

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    info!("main starting");

    let app_conf = app_config::AppConfig::new()?;
    if app_conf.should_show_help() {
        debug!("main: help flag is set");
        print_usage(&app_conf);
        return Ok(());
    }

    let db = HashDB::new();

    trace!("TxApp running");
    let mut tx_app = build_tx_app(&app_conf, db.clone());
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
