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

    // Run whatever process is requested
    let args = AppArgs::parse();
    let mut process = match &args.command {
        AppCommand::Do(args) => {
            log::info!("Doing a 'do' command: {}", args.args.join(" "));
            do_do(&args.args, tx.clone()).unwrap()
        }
    };

    // Let it finish
    // TODO: Listen for ctrl-c to kill the process, actually
    match process.run_until_complete().await {
        Ok(_) => log::info!("Process ended successfully"),
        Err(_) => log::error!("Process did not end successfully"),
    }

    console_writer.shutdown().await.unwrap();

    Ok(())
}
