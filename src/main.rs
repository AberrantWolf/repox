mod console;
mod errors;
mod processing;

use clap::{Args, Parser, Subcommand};
use console::ConsoleLogger;
use console::ConsoleWriter;
use errors::Result;
use log;

use crate::console::ConsoleMessage;
use crate::processing::do_do;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[command(subcommand)]
    command: AppCommand,
}

#[derive(Subcommand, Debug)]
enum AppCommand {
    Do(DoArgs),
}

#[derive(Args, Debug)]
struct DoArgs {
    args: Vec<String>,
}

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
    let args = AppArgs::parse();
    match &args.command {
        AppCommand::Do(args) => {
            log::info!("Doing a 'do' command: {}", args.args.join(" "));
            do_do(&args.args).unwrap();
        }
    }

    // TODO: Wait until runing process ends before dying

    console_writer.shutdown().await.unwrap();

    Ok(())
}
