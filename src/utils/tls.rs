use crate::error::{CertError, Result};
use std::path::Path;

use tonic::transport::{Certificate, ClientTlsConfig, Identity};
pub struct CertPaths {
    pub ca_cert: String,
    pub client_cert: String,
    pub client_key: String,
}

impl CertPaths {
    fn new(cert_dir: String) -> Self {
        Self {
            ca_cert: cert_dir.clone() + "/ca.pem",
            client_cert: cert_dir.clone() + "/client.pem",
            client_key: cert_dir.clone() + "/client-key.pem",
        }
    }
}

pub async fn load_tls_config(cert_dir: Option<String>) -> Result<ClientTlsConfig> {
    let dir = cert_dir.unwrap_or("~".into());
    let paths = CertPaths::new(dir);
    load_tls_config_with_paths(&paths).await
}

pub async fn load_tls_config_with_paths(paths: &CertPaths) -> Result<ClientTlsConfig> {
    // Check if files exist first
    check_cert_files(paths)?;

    // Load certificates
    let ca_cert = tokio::fs::read(&paths.ca_cert)
        .await
        .map_err(CertError::IoError)?;

    let client_cert = tokio::fs::read(&paths.client_cert)
        .await
        .map_err(CertError::IoError)?;

    let client_key = tokio::fs::read(&paths.client_key)
        .await
        .map_err(CertError::IoError)?;

    // Create TLS configuration
    let tls_config = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(Certificate::from_pem(&ca_cert))
        .identity(Identity::from_pem(&client_cert, &client_key));

    Ok(tls_config)
}

fn check_cert_files(paths: &CertPaths) -> Result<()> {
    let check_file = |path: &str| -> Result<()> {
        if !Path::new(path).exists() {
            return Err(CertError::MissingCert(path.to_string()));
        }
        Ok(())
    };

    check_file(&paths.ca_cert)?;
    check_file(&paths.client_cert)?;
    check_file(&paths.client_key)?;

    Ok(())
}
