use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Node agent request failed: {0}")]
    NodeAgent(#[from] tonic::Status),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::NodeAgent(err) => match err.code() {
                tonic::Code::Unavailable => StatusCode::SERVICE_UNAVAILABLE,

                tonic::Code::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,

                _ => StatusCode::BAD_GATEWAY,
            },
        };

        (status, "Failed to retrieve node information").into_response()
    }
}
