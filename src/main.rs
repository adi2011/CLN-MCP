mod client;
mod error;
mod utils;

use crate::client::config::{create_channel, ClientConfig};
use crate::client::node::NodeService;
use crate::utils::tls::load_tls_config;
use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};
use std::env;
use std::net::ToSocketAddrs;
use std::time::Duration;
use tracing::{debug, error, info};
use tracing_subscriber::{self, EnvFilter};

fn print_usage() {
    println!("Usage: cln-mcp [OPTIONS]");
    println!("Options:");
    println!("  --certs-dir <path>    Path to certificates directory");
    println!("  --node-address <url>  Node address (default: https://localhost:9736)");
    println!("  --help                Shows this help message");
}

fn parse_args() -> (Option<String>, String) {
    let args: Vec<String> = env::args().collect();
    let mut certs_dir = None;
    let mut node_address = "https://localhost:9736".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--certs-dir" => {
                if i + 1 < args.len() {
                    certs_dir = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    error!("Error: --certs-dir requires a path");
                    std::process::exit(1);
                }
            }
            "--node-adddress" => {
                if i + 1 < args.len() {
                    node_address = args[i + 1].clone();
                    if node_address.to_socket_addrs().is_err() {
                        error!("Error: Invalid node address!");
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    error!("Error: --node-address requires a URL");
                    std::process::exit(1);
                }
            }
            "--help" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                error!("Unknown argument: {}", args[i]);
                print_usage();
                std::process::exit(1);
            }
        }
    }

    (certs_dir, node_address)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();

    info!("CLN MCP server initiated!");

    let (certs_dir, node_addr) = parse_args();

    // Load TLS certificate
    let tls_config = load_tls_config(certs_dir).await?;

    debug!("TLS certificate loaded!");

    // Create channel with default config
    let config = ClientConfig::new(node_addr, Duration::from_secs(1), Duration::from_secs(5));

    let channel = create_channel(&config)?
        .tls_config(tls_config)?
        .connect_lazy();

    info!("--------------------Server Started Running!--------------------------");

    // Create and run the server with STDIO transport
    let service = NodeService::new(channel)
        .serve(stdio())
        .await
        .inspect_err(|e| {
            println!("Error starting server: {e}");
        })?;

    service.waiting().await?;
    Ok(())
}
