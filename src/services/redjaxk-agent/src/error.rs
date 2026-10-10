
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{self:?}")]
    PodmanSocketConnect {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed podgate request")]
    Podgate {
        request: redjaxk_protocol::podgate::Request,
        message: String,
    },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
