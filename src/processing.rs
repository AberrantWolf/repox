use std::process::{ExitStatus, Stdio};

use crate::{
    console::ConsoleMessage,
    errors::{AppError, Result},
};
use anyhow::anyhow;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    sync::mpsc::Sender,
    task::JoinHandle,
};

pub struct Processor {
    child: Child,
    join_handle: JoinHandle<()>,
}

impl Processor {
    fn new(mut child: Child, tx: Sender<ConsoleMessage>) -> Self {
        let stdout = child.stdout.take().unwrap();
        let mut out_reader = BufReader::new(stdout).lines();

        let join_handle = tokio::spawn(async move {
            while let Some(line) = out_reader.next_line().await.unwrap() {
                if let Err(e) = tx.send(ConsoleMessage::Stdout(line.into())).await {
                    log::error!("Unable to send STDOUT line to console: {}", e);
                }
            }
        });

        Self { child, join_handle }
    }

    pub async fn run_until_complete(&mut self) -> Result<ExitStatus> {
        match self.child.wait().await {
            Ok(status) => Ok(status),
            Err(_) => Err(anyhow!(AppError::NoExitStatus)),
        }
    }
}

pub fn do_do(args: &Vec<String>, tx: Sender<ConsoleMessage>) -> Result<Processor> {
    let (cmdstr, argstr) = if let Some(cmd) = args.first() {
        (cmd, format!("{}", args[1..].join(" ")))
    } else {
        return Err(anyhow!(AppError::NoCommand));
    };

    let child = {
        let mut cmd = Command::new(cmdstr);
        cmd.stdout(Stdio::piped());

        if argstr.len() > 0 {
            cmd.arg(argstr);
        }
        cmd.spawn().unwrap()
    };

    let processor = Processor::new(child, tx);

    Ok(processor)
}
