use crate::{AppArgs, errors::Result, strings::Prefixes};
use log::Log;
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub enum ConsoleMessage {
    Stdout(String),
    Stderr(String),
    InfoLog(String),
    WarnLog(String),
    ErrorLog(String),
    DebugLog(String),
    TraceLog(String),
}

pub struct ConsoleWriter {
    tx: mpsc::Sender<ConsoleMessage>,
    join_handle: JoinHandle<()>,
    cancel: CancellationToken,
}

impl ConsoleWriter {
    pub fn new(args: &AppArgs) -> Self {
        let (tx, rx) = mpsc::channel::<ConsoleMessage>(100);
        let cancel = CancellationToken::new();
        let prefixes = Prefixes::new(&args);
        let join_handle = listen_loop(rx, cancel.clone(), prefixes);

        ConsoleWriter {
            tx,
            join_handle,
            cancel,
        }
    }

    pub fn get_write_channel(&self) -> mpsc::Sender<ConsoleMessage> {
        self.tx.clone()
    }

    // Consumes the ConsoleWriter
    pub async fn shutdown(self) -> Result<()> {
        self.cancel.cancel();
        self.join_handle.await.unwrap();
        Ok(())
    }
}

pub struct ConsoleLogger {
    tx: mpsc::Sender<ConsoleMessage>,
}

impl ConsoleLogger {
    pub fn new(tx: &mpsc::Sender<ConsoleMessage>) -> Self {
        Self { tx: tx.clone() }
    }

    pub fn init(self) -> Result<()> {
        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self)).unwrap();
        Ok(())
    }
}

impl Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let tx = self.tx.clone();
        let console_message = match record.level() {
            log::Level::Error => ConsoleMessage::ErrorLog(format!("{}", record.args())),
            log::Level::Warn => ConsoleMessage::WarnLog(format!("{}", record.args())),
            log::Level::Info => ConsoleMessage::InfoLog(format!("{}", record.args())),
            log::Level::Debug => ConsoleMessage::DebugLog(format!("{}", record.args())),
            log::Level::Trace => ConsoleMessage::TraceLog(format!("{}", record.args())),
        };
        tokio::spawn(async move {
            match tx.send(console_message).await {
                Ok(_) => (),
                Err(_) => println!("ERROR"),
            }
        });
    }

    fn flush(&self) {}
}

fn write_message_to_console(msg: ConsoleMessage, prefixes: &Prefixes) {
    match msg {
        ConsoleMessage::Stdout(s) => println!("{} {s}", prefixes.stdout),
        ConsoleMessage::Stderr(s) => println!("{} {s}", prefixes.stderr),
        ConsoleMessage::InfoLog(s) => println!("{} {s}", prefixes.info),
        ConsoleMessage::WarnLog(s) => println!("{} {s}", prefixes.warn),
        ConsoleMessage::ErrorLog(s) => println!("{} {s}", prefixes.error),
        ConsoleMessage::DebugLog(s) => println!("{} {s}", prefixes.debug),
        ConsoleMessage::TraceLog(s) => println!("{} {s}", prefixes.trace),
    }
}

fn listen_loop(
    mut rx: mpsc::Receiver<ConsoleMessage>,
    cancel: CancellationToken,
    prefixes: Prefixes,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        // Loop on messages until the writer signals the cancel token
        loop {
            tokio::select! {
                Some(msg) = rx.recv() => {
                    write_message_to_console(msg, &prefixes);
                }
                _ = cancel.cancelled() => {
                    println!("Ending console output");
                    break;
                }
            }
        }

        // Close the MPSC channel, and then pump any remaining messages
        rx.close();
        while let Ok(msg) = rx.try_recv() {
            write_message_to_console(msg, &prefixes);
        }
    })
}
