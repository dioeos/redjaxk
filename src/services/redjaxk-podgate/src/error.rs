#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{self:?}")]
    PodmanSocketBind {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("{self:?}")]
    SetPodmanSocketPermissions { mode: u32, message: String },

    #[error("{self:?}")]
    SocketAccept(#[source] std::io::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
