use cln_mcp::{create_channel, load_tls_config, ClientConfig, NodeService};
use serde_json::Value;
use std::time::Duration;
use tokio;

async fn setup_test_env() -> Result<NodeService, Box<dyn std::error::Error>> {
    let tls_config = load_tls_config(Some("/Users/runner/.lightning/regtest/".to_string())).await?;

    let config = ClientConfig::new(
        "https://localhost:9736".to_string(),
        Duration::from_secs(1),
        Duration::from_secs(5),
    );

    let channel = create_channel(&config)?
        .tls_config(tls_config)?
        .connect_lazy();

    Ok(NodeService::new(channel))
}

#[tokio::test]
async fn test_server_initialization() {
    let result = setup_test_env().await;
    assert!(
        result.is_ok(),
        "Failed to initialize server: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_get_info() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.get_info().await;
    assert!(result.is_ok(), "get_info failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("lightning_dir"))
    }
}
