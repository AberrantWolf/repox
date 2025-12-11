mod console;
mod errors;

use console::ConsoleLogger;
use console::ConsoleWriter;
use errors::Result;
use log;

use crate::console::ConsoleMessage;

#[tokio::main]
async fn main() -> Result<()> {
    let console_writer = ConsoleWriter::new();
    let tx = console_writer.get_write_channel();
    ConsoleLogger::new(&tx).init().unwrap();

    log::info!("Test");
    log::error!("Test2");

    tx.send(ConsoleMessage::InfoLog("Started!".into()))
        .await
        .unwrap();

    // Live and do stuff, I guess

    console_writer.shutdown().await.unwrap();

    Ok(())
}
