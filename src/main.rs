mod console;
mod errors;
mod processing;
mod strings;

use crate::console::ConsoleLogger;
use crate::console::ConsoleWriter;
use crate::errors::Result;
use crate::processing::do_do;
use crate::strings::Prefixes;

use clap::{Args, Parser, Subcommand};
use log;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AppArgs {
    /// Disable colors and emoji
    #[arg(long)]
    no_fun: bool,

    /// The name of the command to try and run
    #[command(subcommand)]
    command: AppCommand,
}

#[derive(Subcommand, Debug)]
enum AppCommand {
    Do(DoArgs),
    List,
}

pub struct AppState {
    pub log_prefixes: Prefixes,
}

#[derive(Args, Debug)]
struct DoArgs {
    args: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Run whatever process is requested
    let args = AppArgs::parse();

    let console_writer = ConsoleWriter::new(&args);

    let tx = console_writer.get_write_channel();
    ConsoleLogger::new(&tx).init().unwrap();

    let opt_process = match &args.command {
        AppCommand::Do(args) => {
            log::info!("Doing a 'do' command: {}", args.args.join(" "));
            Some(do_do(&args.args, tx.clone()).unwrap())
        }
        AppCommand::List => {
            log::debug!("'list' is not implemented yet");
            None
        }
    };

    // Let any process finish if one was started
    // TODO: Listen for ctrl-c to kill the process, actually
    if let Some(process) = opt_process {
        match process.run_until_complete().await {
            Ok(_) => log::info!("Process ended successfully"),
            Err(_) => log::error!("Process did not end successfully"),
        }
    }

    console_writer.shutdown().await.unwrap();

    Ok(())
}
