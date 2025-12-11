pub use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("No command was given when one is required to function.")]
    NoCommand,
    #[error("No exit status given for the process on completion")]
    NoExitStatus,
}
