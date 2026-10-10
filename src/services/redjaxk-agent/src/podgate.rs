use redjaxk_protocol::podgate::{PodmanContainer, Request::ListContainers};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
    sync::Mutex,
};

pub struct PodgateClient {
    stream: Mutex<BufReader<UnixStream>>,
}

const REDJAXK_PODMAN_SOCK: &str = "/run/redjaxk/podman.sock";

use crate::error::Error;

impl PodgateClient {
    pub async fn connect() -> Result<Self, Error> {
        let connection = UnixStream::connect(REDJAXK_PODMAN_SOCK)
            .await
            .map_err(|err| Error::PodmanSocketConnect {
                path: REDJAXK_PODMAN_SOCK.to_owned(),
                source: err,
            })?;

        let stream = BufReader::new(connection);
        Ok(Self {
            stream: Mutex::new(stream),
        })
    }

    pub async fn list_containers(&self) -> Result<Vec<PodmanContainer>, Error> {
        let mut stream = self.stream.lock().await;

        let mut request = serde_json::to_vec(&ListContainers)?;
        request.push(b'\n');

        stream.get_mut().write_all(&request).await?;

        let mut buf = String::new();
        if stream.read_line(&mut buf).await? == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Podgate closed the connection",
            ))?;
        }

        let response = serde_json::from_str::<redjaxk_protocol::podgate::Response>(&buf)?;

        match response {
            redjaxk_protocol::podgate::Response::Containers { containers } => Ok(containers),
            redjaxk_protocol::podgate::Response::Error { request, message } => {
                Err(Error::Podgate { request, message })
            }
        }
    }
}
