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
use tokio_util::sync::CancellationToken;

pub struct Processor {
    cancellation_token: CancellationToken,
    join_handle: JoinHandle<ExitStatus>,
}

enum LogLine {
    StdOut(String),
    StdErr(String),
}

impl Processor {
    fn new(mut child: Child, tx: Sender<ConsoleMessage>) -> Self {
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        let mut out_reader = BufReader::new(stdout).lines();
        let mut err_reader = BufReader::new(stderr).lines();

        let cancellation_token = CancellationToken::new();
        let inner_cancellation_token = cancellation_token.clone();

        let join_handle = tokio::spawn(async move {
            let exit_status: ExitStatus;
            loop {
                let log_line_opt = tokio::select! {
                    Ok(Some(line)) = out_reader.next_line() => {
                        Some(LogLine::StdOut(line))
                        }
                    Ok(Some(line)) = err_reader.next_line() => {
                        Some(LogLine::StdErr(line))
                    }
                    status = child.wait() => {
                        exit_status = status.unwrap();
                        break
                    }
                    _ = inner_cancellation_token.cancelled() => {
                        match child.kill().await {
                            Ok(_) => log::info!("Child process killed successfully"),
                            Err(e) => log::error!("Problem killing child process {e}"),
                        }
                        None
                    }
                };

                if let Some(log_line) = log_line_opt {
                    let send_result = match log_line {
                        LogLine::StdOut(line) => tx.send(ConsoleMessage::Stdout(line.into())).await,
                        LogLine::StdErr(line) => tx.send(ConsoleMessage::Stderr(line.into())).await,
                    };
                    if let Err(e) = send_result {
                        log::error!("Unable to send STDOUT line to console: {}", e);
                    }
                }
            }

            // Drain stdout
            while let Some(line) = out_reader.next_line().await.unwrap() {
                if let Err(e) = tx.send(ConsoleMessage::Stdout(line.into())).await {
                    log::error!("Unable to send STDOUT line to console: {}", e);
                }
            }

            // Drain stderr
            while let Some(line) = err_reader.next_line().await.unwrap() {
                if let Err(e) = tx.send(ConsoleMessage::Stderr(line.into())).await {
                    log::error!("Unable to send STDOUT line to console: {}", e);
                }
            }

            exit_status
        });

        Self {
            cancellation_token,
            join_handle,
        }
    }

    pub async fn run_until_complete(self) -> Result<ExitStatus> {
        match self.join_handle.await {
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
        cmd.stderr(Stdio::piped());

        if argstr.len() > 0 {
            cmd.arg(argstr);
        }
        cmd.spawn().unwrap()
    };

    let processor = Processor::new(child, tx);

    Ok(processor)
}
