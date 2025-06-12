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

// Node Information Tests
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

        assert!(obj.contains_key("lightning_dir"));
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("alias"));
        assert!(obj.contains_key("version"));
    }
}

#[tokio::test]
async fn test_list_configs() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_configs().await;
    assert!(result.is_ok(), "list_configs failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("configs"));
    }
}

#[tokio::test]
async fn test_list_addresses() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_addresses().await;
    assert!(result.is_ok(), "list_addresses failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("addresses"));
    }
}

// Channel Information Tests
#[tokio::test]
async fn test_list_channels() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_channels().await;
    assert!(result.is_ok(), "list_channels failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("channels"));
    }
}

#[tokio::test]
async fn test_list_peer_channels() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_peer_channels().await;
    assert!(
        result.is_ok(),
        "list_peer_channels failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("channels"));
    }
}

#[tokio::test]
async fn test_list_closed_channels() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_closed_channels().await;
    assert!(
        result.is_ok(),
        "list_closed_channels failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("closedchannels"));
    }
}

#[tokio::test]
async fn test_list_htlcs() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_htlcs().await;
    assert!(result.is_ok(), "list_htlcs failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("htlcs"));
    }
}

// Payment Information Tests
#[tokio::test]
async fn test_list_pays() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_pays().await;
    assert!(result.is_ok(), "list_pays failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("pays"));
    }
}

#[tokio::test]
async fn test_list_send_pays() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_send_pays().await;
    assert!(result.is_ok(), "list_send_pays failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("payments"));
    }
}

#[tokio::test]
async fn test_list_forwards() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_forwards().await;
    assert!(result.is_ok(), "list_forwards failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("forwards"));
    }
}

#[tokio::test]
async fn test_list_invoices() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_invoices().await;
    assert!(result.is_ok(), "list_invoices failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("invoices"));
    }
}

// #[tokio::test]
// async fn test_list_invoice_requests() {
//     let service = setup_test_env()
//         .await
//         .expect("Failed to setup test environment");

//     let result = service.list_invoice_requests().await;
//     assert!(result.is_ok(), "list_invoice_requests failed: {:?}", result.err());

//     if let Ok(response) = result {
//         let content = response.content.first().expect("Empty response!");
//         let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
//         let obj = res_val.as_object().unwrap();

//         assert!(obj.contains_key("invoice_requests"));
//     }
// }

// Network Information Tests
#[tokio::test]
async fn test_list_peers() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_peers().await;
    assert!(result.is_ok(), "list_peers failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("peers"));
    }
}

#[tokio::test]
async fn test_list_nodes() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_nodes().await;
    assert!(result.is_ok(), "list_nodes failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("nodes"));
    }
}

#[tokio::test]
async fn test_list_funds() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_funds().await;
    assert!(result.is_ok(), "list_funds failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("outputs"));
        assert!(obj.contains_key("channels"));
    }
}

// #[tokio::test]
// async fn test_get_route() {
//     let service = setup_test_env()
//         .await
//         .expect("Failed to setup test environment");

//     let result = service.get_route().await;
//     assert!(result.is_ok(), "get_route failed: {:?}", result.err());

//     if let Ok(response) = result {
//         let content = response.content.first().expect("Empty response!");
//         let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
//         let obj = res_val.as_object().unwrap();

//         assert!(obj.contains_key("route"));
//     }
// }

// Offer Information Tests
#[tokio::test]
async fn test_list_offers() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_offers().await;
    assert!(result.is_ok(), "list_offers failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("offers"));
    }
}

// Database Information Tests
#[tokio::test]
async fn test_list_datastore() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.list_datastore().await;
    assert!(result.is_ok(), "list_datastore failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("datastore"));
    }
}

// Bookkeeping Information Tests
#[tokio::test]
async fn test_bkpr_channels_apy() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.bkpr_channels_apy().await;
    assert!(
        result.is_ok(),
        "bkpr_channels_apy failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("channels_apy"));
    }
}

#[tokio::test]
async fn test_bkpr_list_balances() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.bkpr_list_balances().await;
    assert!(
        result.is_ok(),
        "bkpr_list_balances failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("accounts"));
    }
}

#[tokio::test]
async fn test_bkpr_list_income() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.bkpr_list_income().await;
    assert!(
        result.is_ok(),
        "bkpr_list_income failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("income_events"));
    }
}

#[tokio::test]
async fn test_bkpr_list_account_events() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.bkpr_list_account_events().await;
    assert!(
        result.is_ok(),
        "bkpr_list_account_events failed: {:?}",
        result.err()
    );

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("events"));
    }
}

// Utility Commands Tests
// #[tokio::test]
// async fn test_decode() {
//     let service = setup_test_env()
//         .await
//         .expect("Failed to setup test environment");

//     let result = service.decode().await;
//     assert!(result.is_ok(), "decode failed: {:?}", result.err());

//     if let Ok(response) = result {
//         let content = response.content.first().expect("Empty response!");
//         let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
//         let obj = res_val.as_object().unwrap();

//         assert!(obj.contains_key("type"));
//     }
// }

// #[tokio::test]
// async fn test_decode_pay() {
//     let service = setup_test_env()
//         .await
//         .expect("Failed to setup test environment");

//     let result = service.decode_pay().await;
//     assert!(result.is_ok(), "decode_pay failed: {:?}", result.err());

//     if let Ok(response) = result {
//         let content = response.content.first().expect("Empty response!");
//         let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
//         let obj = res_val.as_object().unwrap();

//         assert!(obj.contains_key("currency"));
//         assert!(obj.contains_key("created_at"));
//         assert!(obj.contains_key("expiry"));
//         assert!(obj.contains_key("payee"));
//         assert!(obj.contains_key("amount_msat"));
//         assert!(obj.contains_key("payment_hash"));
//         assert!(obj.contains_key("signature"));
//         assert!(obj.contains_key("description"));
//     }
// }

// #[tokio::test]
// async fn test_check_message() {
//     let service = setup_test_env()
//         .await
//         .expect("Failed to setup test environment");

//     let result = service.check_message().await;
//     assert!(result.is_ok(), "check_message failed: {:?}", result.err());

//     if let Ok(response) = result {
//         let content = response.content.first().expect("Empty response!");
//         let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
//         let obj = res_val.as_object().unwrap();

//         assert!(obj.contains_key("verified"));
//     }
// }

#[tokio::test]
async fn test_feerates() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.feerates().await;
    assert!(result.is_ok(), "feerates failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("perkw"));
        assert!(obj.contains_key("perkb"));
        assert!(obj.contains_key("onchain_fee_estimates"));
    }
}

#[tokio::test]
async fn test_get_log() {
    let service = setup_test_env()
        .await
        .expect("Failed to setup test environment");

    let result = service.get_log().await;
    assert!(result.is_ok(), "get_log failed: {:?}", result.err());

    if let Ok(response) = result {
        let content = response.content.first().expect("Empty response!");
        let res_val: Value = serde_json::from_str(&content.raw.as_text().unwrap().text).unwrap();
        let obj = res_val.as_object().unwrap();

        assert!(obj.contains_key("log"));
    }
}
