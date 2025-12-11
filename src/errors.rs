pub use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("No command was given when one is required to function.")]
    NoCommand,
}
