pub mod error;

use std::{fs, io, os::unix::fs::PermissionsExt};

use redjaxk_protocol::podgate::{PodmanContainer, Request, Response};
use serde_json::Value;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixListener,
    process::Command,
};
use tracing::{info, warn};
use tracing_subscriber::{EnvFilter, fmt};

use crate::error::Error;

const REDJAXK_PODMAN_SOCK: &str = "/run/redjaxk/podman.sock";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let format = fmt::format().with_level(true).with_target(true).compact();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .event_format(format)
        .init();

    let listener =
        UnixListener::bind(REDJAXK_PODMAN_SOCK).map_err(|err| Error::PodmanSocketBind {
            path: REDJAXK_PODMAN_SOCK.to_owned(),
            source: err,
        })?;

    let mode = 0o600;
    fs::set_permissions(REDJAXK_PODMAN_SOCK, fs::Permissions::from_mode(mode)).map_err(|err| {
        Error::SetPodmanSocketPermissions {
            mode,
            message: err.to_string(),
        }
    })?;

    info!(listener_path = REDJAXK_PODMAN_SOCK, "successful bind");

    loop {
        let (stream, _) = listener.accept().await.map_err(Error::SocketAccept)?;

        //@NOTE: Currently operates in oneshot (connects client, handles, drops)
        tokio::spawn(async move {
            let handle_result = async {
                let (reader_s, mut writer_s) = stream.into_split();

                let mut reader = BufReader::new(reader_s);
                let mut request_line = String::new();
                reader.read_line(&mut request_line).await?;

                let request = serde_json::from_str::<Request>(&request_line)?;

                match request {
                    Request::ListContainers => {
                        let output = Command::new("podman")
                            .args(["ps", "--all", "--format", "json"])
                            .output()
                            .await?;

                        if !output.status.success() {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            let stderr = stderr.trim();

                            warn!(
                                status = %output.status,
                                stderr = %stderr,
                                "Podman CLI command failed"
                            );

                            let response = Response::Error {
                                request,
                                message: format!(
                                    "podman cli failed ({}): {}",
                                    output.status,
                                    String::from_utf8_lossy(&output.stderr).trim()
                                ),
                            };

                            let mut bytes = serde_json::to_vec(&response)?;
                            bytes.push(b'\n');
                            writer_s.write_all(&bytes).await?;

                            return Ok(());
                        }

                        let values =
                            serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout)?;

                        let containers = values
                            .iter()
                            .map(build_container)
                            .collect::<io::Result<Vec<_>>>()?;

                        let response = Response::Containers { containers };

                        let mut bytes = serde_json::to_vec(&response)?;
                        bytes.push(b'\n');
                        writer_s.write_all(&bytes).await?;
                    }
                }
                Ok::<(), Error>(())
            }
            .await;

            if let Err(err) = handle_result {
                warn!(error = %err, "Podgate error");
            }
        });
    }
}

fn build_container(value: &Value) -> io::Result<PodmanContainer> {
    fn required_string_value(value: &Value, field: &str) -> io::Result<String> {
        value
            .get(field)
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("missing or invalid field: {field}"),
                )
            })
    }

    Ok(PodmanContainer {
        id: required_string_value(value, "Id")?,
        image: required_string_value(value, "Image")?,
        redjaxk_name: required_string_value(&value["Labels"], "redjaxk.name")?,
        redjaxk_url: required_string_value(&value["Labels"], "redjaxk.url")?,
    })
}
