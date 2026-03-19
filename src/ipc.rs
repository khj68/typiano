use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

use crate::config;

/// Send a command to the running daemon and return the response.
pub fn send_command(cmd: &str) -> Result<String> {
    let socket_path = config::socket_path();
    let mut stream =
        UnixStream::connect(&socket_path).context("cannot connect to typiano daemon")?;
    writeln!(stream, "{cmd}")?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response.trim().to_string())
}

/// IPC command handler callback type
pub type CommandHandler = Box<dyn Fn(&str) -> String + Send>;

/// Start the IPC server, calling handler for each command received.
pub fn start_server(handler: CommandHandler) -> Result<()> {
    let socket_path = config::socket_path();

    if socket_path.exists() {
        std::fs::remove_file(&socket_path)?;
    }

    let listener = UnixListener::bind(&socket_path).context("failed to bind IPC socket")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut reader = BufReader::new(&stream);
                let mut cmd = String::new();
                if reader.read_line(&mut cmd).is_ok() {
                    let cmd = cmd.trim();
                    let response = handler(cmd);
                    let mut writer = stream;
                    let _ = write!(writer, "{response}");
                    let _ = writer.flush();
                    let _ = writer.shutdown(std::net::Shutdown::Write);

                    if cmd == "shutdown" {
                        break;
                    }
                }
            }
            Err(_) => continue,
        }
    }

    let _ = std::fs::remove_file(&socket_path);
    Ok(())
}
