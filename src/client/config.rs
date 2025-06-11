use std::time::Duration;
use tonic::transport::Endpoint;

pub struct ClientConfig {
    pub endpoint: String,
    pub keep_alive_interval: Duration,
    pub keep_alive_timeout: Duration,
}

impl ClientConfig {
    pub fn new(
        endpoint: String,
        keep_alive_interval: Duration,
        keep_alive_timeout: Duration,
    ) -> Self {
        Self {
            endpoint,
            keep_alive_interval,
            keep_alive_timeout,
        }
    }
}

pub fn create_channel(config: &ClientConfig) -> Result<Endpoint, tonic::transport::Error> {
    Ok(Endpoint::from_shared(config.endpoint.clone())?
        .tcp_keepalive(Some(Duration::from_secs(1)))
        .http2_keep_alive_interval(config.keep_alive_interval)
        .keep_alive_timeout(config.keep_alive_timeout)
        .keep_alive_while_idle(true))
}
