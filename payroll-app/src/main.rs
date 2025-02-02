use log::{debug, info, trace};

use app::Application;
use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use text_parser_tx_source::TextParserTxSource;
use tx_app::{Runner, TxApp, TxSource};
use tx_impl::TxFactoryImpl;

mod app_config;
mod app_impl;
mod reader;
mod runner;

use app_config::AppConfig;

// TODO: remove db argument
fn build_tx_app(app_conf: &AppConfig, db: HashDB) -> Box<dyn Application> {
    trace!("build_tx_app called");
    let tx_source = make_tx_source(db, &app_conf);
    let mut tx_runner: Box<dyn Runner> = if app_conf.should_run_quietly() {
        debug!("build_tx_app: using silent runner");
        runner::silent_runner()
    } else {
        debug!("build_tx_app: using echoback runner");
        runner::echoback_runner()
    };
    if app_conf.should_enable_chronograph() {
        debug!("build_tx_app: runner with chronograph");
        tx_runner = runner::with_chronograph(tx_runner);
    }

    Box::new(TxApp::new(tx_source, tx_runner))
}

fn make_tx_source(db: HashDB, conf: &AppConfig) -> Box<dyn TxSource> {
    trace!("make_tx_source called");
    let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);
    if let Some(file) = conf.script_file().clone() {
        debug!("make_tx_source: file={}", file);
        let mut reader = reader::file_reader(&file);
        if !conf.should_run_quietly() {
            debug!("make_tx_source: using EchoReader");
            reader = reader::with_echo(reader);
        }
        if conf.should_dive_into_repl() {
            debug!("make_tx_source: dive into REPL mode after file loaded");
            reader = reader::join(reader, reader::interact_reader());
            return Box::new(TextParserTxSource::new(tx_factory, reader));
        }
        return Box::new(TextParserTxSource::new(tx_factory, reader));
    }

    debug!("make_tx_source: file is None, using stdin");
    Box::new(TextParserTxSource::new(
        tx_factory,
        reader::interact_reader(),
    ))
}

fn print_usage(app_conf: &AppConfig) {
    trace!("print_usage called");
    println!("{}", app_conf.usage_string());
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    info!("main starting");

    let app_conf = AppConfig::new()?;
    if app_conf.should_show_help() {
        debug!("main: help flag is set");
        print_usage(&app_conf);
        return Ok(());
    }

    let db = HashDB::new();

    trace!("TxApp running");
    let mut tx_app: Box<dyn Application> = build_tx_app(&app_conf, db.clone());
    if app_conf.should_enable_chronograph() {
        debug!("main: using AppChronograph");
        tx_app = app_impl::with_chronograph(tx_app);
    }
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
