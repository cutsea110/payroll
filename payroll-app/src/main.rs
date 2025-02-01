use log::{debug, info, trace};
use std::io::{BufRead, BufReader};

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
use app_impl::AppChronograph;
use reader::{EchoReader, InteractReader};
use runner::{TxEchoBachRunner, TxRunnerChronograph, TxSilentRunner};

// TODO: remove db argument
fn build_tx_app(app_conf: &AppConfig, db: HashDB) -> Box<dyn Application> {
    trace!("build_tx_app called");
    let tx_source = make_tx_source(db, &app_conf);
    let mut tx_runner: Box<dyn Runner> = if app_conf.should_run_quietly() {
        debug!("build_tx_app: using TxSilentRunner");
        Box::new(TxSilentRunner)
    } else {
        debug!("build_tx_app: using TxEchoBachRunner");
        Box::new(TxEchoBachRunner)
    };
    if app_conf.should_enable_chronograph() {
        debug!("build_tx_app: using TxRunnerChronograph");
        tx_runner = Box::new(TxRunnerChronograph::new(tx_runner));
    }

    Box::new(TxApp::new(tx_source, tx_runner))
}

fn make_tx_source(db: HashDB, opts: &AppConfig) -> Box<dyn TxSource> {
    trace!("make_tx_source called");
    let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);
    if let Some(file) = opts.script_file().clone() {
        debug!("make_tx_source: file={}", file);
        let buf = std::fs::File::open(file).expect("open file");
        let mut reader: Box<dyn BufRead> = Box::new(BufReader::new(buf));
        if !opts.should_run_quietly() {
            debug!("make_tx_source: using EchoReader");
            reader = Box::new(EchoReader::new(reader));
        }
        return Box::new(TextParserTxSource::new(tx_factory, reader));
    }

    debug!("make_tx_source: file is None, using stdin");
    Box::new(TextParserTxSource::new(
        tx_factory,
        Box::new(InteractReader::new()),
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
        tx_app = Box::new(AppChronograph::new(tx_app));
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
