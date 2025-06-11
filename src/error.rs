use thiserror::Error;

#[derive(Error, Debug)]
pub enum CertError {
    #[error("Failed to read certificate file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Missing certificate file: {0}")]
    MissingCert(String),

    #[error("TLS configuration error: {0}")]
    TlsError(#[from] tonic::transport::Error),
}

pub type Result<T> = std::result::Result<T, CertError>;
