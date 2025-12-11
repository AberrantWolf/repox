use crate::errors::{AppError, Result};
use anyhow::anyhow;
use tokio::process::Command;

pub fn do_do(args: &Vec<String>) -> Result<()> {
    let (cmd, args) = if let Some(cmd) = args.first() {
        (cmd, args[1..].join(" "))
    } else {
        return Err(anyhow!(AppError::NoCommand));
    };

    let _child = Command::new(cmd)
        // .stdin(Stdio::piped())
        .arg(args)
        .spawn()
        .unwrap();

    // TODO: Put the child somewhere we can wait for it before the program ends

    Ok(())
}
