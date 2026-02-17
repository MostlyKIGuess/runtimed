#![cfg(feature = "client")]

//! Integration test for Y-sync client against a real Jupyter server.
//!
//! This test requires a running Jupyter server with jupyter-server-documents.
//! Start it with:
//!   cd /tmp/jupyter-test && uv run --with jupyter-server --with jupyter-server-documents \
//!     jupyter server --port 18888 --IdentityProvider.token=testtoken123 --no-browser
//!
//! Then get a file ID for the test notebook:
//!   curl -X POST "http://localhost:18888/api/fileid/index?token=testtoken123&path=test.ipynb"

use jupyter_ysync::{ClientConfig, YSyncClient};
use yrs::Doc;

const JUPYTER_URL: &str = "ws://localhost:18888";
const TOKEN: &str = "testtoken123";

// File ID from: curl -X POST "http://localhost:18888/api/fileid/index?token=testtoken123&path=test.ipynb"
const FILE_ID: &str = "dcd20096-cdcf-409e-8ad5-2115c531ddfe";

#[tokio::test]
#[ignore] // Only run when Jupyter server is available
async fn test_connect_to_global_awareness() {
    // The global awareness room doesn't require a file ID
    let url = format!(
        "{}/api/collaboration/room/JupyterLab:globalAwareness?token={}",
        JUPYTER_URL, TOKEN
    );

    println!("Connecting to global awareness: {}", url);

    let config = ClientConfig::new(&url);
    let result = YSyncClient::connect(config).await;

    match result {
        Ok(client) => {
            println!("Connected to global awareness successfully!");
            if let Err(e) = client.close().await {
                println!("Close error: {:?}", e);
            }
        }
        Err(e) => {
            println!("Connection error: {:?}", e);
            panic!("Failed to connect: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Only run when Jupyter server is available
async fn test_connect_and_sync_notebook() {
    // Room ID format: {format}:{type}:{file_id}
    let room_id = format!("json:file:{}", FILE_ID);
    let url = format!(
        "{}/api/collaboration/room/{}?token={}",
        JUPYTER_URL, room_id, TOKEN
    );

    println!("Connecting to notebook room: {}", url);

    let config = ClientConfig::new(&url);
    let result = YSyncClient::connect(config).await;

    match result {
        Ok(mut client) => {
            println!("Connected successfully!");

            // Create a local doc and sync
            let doc = Doc::new();
            match client.sync(&doc).await {
                Ok(()) => {
                    println!("Sync completed! is_synced: {}", client.is_synced());
                    assert!(client.is_synced());
                }
                Err(e) => {
                    println!("Sync error: {:?}", e);
                    panic!("Failed to sync: {:?}", e);
                }
            }

            // Close gracefully
            if let Err(e) = client.close().await {
                println!("Close error: {:?}", e);
            }
        }
        Err(e) => {
            println!("Connection error: {:?}", e);
            panic!("Failed to connect: {:?}", e);
        }
    }
}
