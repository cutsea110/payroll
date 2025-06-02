use log::{debug, error, info, trace};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    str,
};

use app::Application;
use hs_db::HashDB;
use payroll_impl::PayrollFactoryImpl;
use payroll_web::AppConfig;
use text_parser_tx_source::TextParserTxSource;
use threadpool::ThreadPool;
use tx_app::TxApp;
use tx_app_impl::{app_impl, reader_impl, runner_impl};
use tx_impl::TxFactoryImpl;

#[derive(Debug, Clone)]
struct Handler {
    db: HashDB,

    quiet: bool,
    chronograph: bool,
}
impl Handler {
    fn new(db: HashDB, app_conf: &AppConfig) -> Self {
        Self {
            db,
            quiet: app_conf.is_quiet(),
            chronograph: app_conf.chronograph(),
        }
    }
    fn handle_connection(&self, mut stream: TcpStream) {
        trace!("Handling connection from {}", stream.peer_addr().unwrap());
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("read from stream");
        // TODO: check Content-Length header for larger requests
        let text = match str::from_utf8(&buffer) {
            Ok(v) => v.trim_end_matches('\0'),
            Err(e) => {
                error!("Invalid UTF-8 sequence: {}", e);
                return;
            }
        };
        let mut split = text.splitn(2, "\r\n\r\n");

        let header = split.next().unwrap_or_default();
        debug!("Received header:\n{}", header);
        let body = split.next().unwrap_or_default();
        debug!("Received body:\n{}", body);

        let mut tx_app = self.build_tx_app(self.db.clone(), body);
        let response = match tx_app.run() {
            Ok(_) => {
                trace!("Transaction app ran successfully");
                "HTTP/1.1 200 OK\r\n\r\n"
            }
            Err(e) => {
                error!("Error running transaction app: {}", e);
                "HTTP/1.1 500 Server Error\r\n\r\n"
            }
        };

        stream.write(response.as_bytes()).expect("write to stream");
        stream.flush().expect("flush stream");
    }

    fn build_tx_app(&self, db: HashDB, request_body: &str) -> Box<dyn Application> {
        trace!("build_tx_app called");

        let tx_factory = TxFactoryImpl::new(db, PayrollFactoryImpl);
        let tx_source = TextParserTxSource::new(
            tx_factory,
            reader_impl::string_reader(request_body.to_string()),
        );
        let mut tx_runner = if self.quiet {
            runner_impl::silent_runner()
        } else {
            runner_impl::echoback_runner()
        };
        if self.chronograph {
            trace!("Chronograph mode enabled");
            tx_runner = runner_impl::with_chronograph(tx_runner);
        };
        let mut tx_app: Box<dyn Application> = Box::new(TxApp::new(Box::new(tx_source), tx_runner));
        if self.chronograph {
            trace!("Adding fail-open mode");
            tx_app = app_impl::with_chronograph(tx_app);
        }

        tx_app
    }
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_default_env()
        .format_source_path(true)
        .format_line_number(true)
        .init();

    info!("Starting server...");

    let app_conf = payroll_web::AppConfig::new()?;
    debug!("main: app_conf={:#?}", app_conf);
    if app_conf.should_show_help() {
        debug!("main: should show help");
        println!("{}", app_conf.help_message());
        return Ok(());
    }

    let pool = ThreadPool::new(app_conf.threads());
    let handler = Handler::new(HashDB::new(), &app_conf);
    let listener = TcpListener::bind(&app_conf.sock_addr())
        .expect(&format!("Bind to {}", app_conf.sock_addr()));

    for stream in listener.incoming() {
        trace!("Incoming connection");
        let stream = stream.expect("accept connection");
        let handler = handler.clone();

        pool.execute(move || {
            handler.handle_connection(stream);
        });
    }

    info!("Shutting down.");
    Ok(())
}
