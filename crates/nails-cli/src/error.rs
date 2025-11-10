use anyhow::Error as AnyhowError;

/// All errors possible to occur during reconciliation
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Any error originating from the `kube-rs` crate
    #[error("Kubernetes reported error: {source}")]
    Kube {
        #[from]
        source: kube::Error,
    },
    /// Raised when an operator dependency has not been installed in the cluster.
    #[error("{0}")]
    DependencyMissing(&'static str),
    /// Error in user input or application resource definition, typically missing fields.
    //#[error("Invalid application CRD: {0}")]
    //UserInput(String),
    #[error("Invalid Kubernetes Yaml: {source}")]
    Yaml {
        #[from]
        source: serde_json::Error,
    },
    /// Any other error that does not map cleanly to the variants above.
    #[error("{0}")]
    Other(String),
}

impl From<AnyhowError> for Error {
    fn from(err: AnyhowError) -> Self {
        Error::Other(err.to_string())
    }
}
