//! Live editing example - connects to a running Jupyter server and modifies a notebook.
//!
//! Usage:
//!   cargo run -p jupyter-ysync --features client --example live_edit

use jupyter_ysync::{ClientConfig, YSyncClient};
use yrs::{Array, GetString, Map, ReadTxn, Text, Transact};

const JUPYTER_URL: &str = "ws://localhost:18888";
const TOKEN: &str = "testtoken123";
const FILE_ID: &str = "9539cb85-a726-4e60-a3fc-e6825563ead4";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JupyterLab uses "notebook" type, not "file"
    let room_id = format!("json:notebook:{}", FILE_ID);
    let url = format!(
        "{}/api/collaboration/room/{}?token={}",
        JUPYTER_URL, room_id, TOKEN
    );

    println!("Connecting to: {}", url);

    let config = ClientConfig::new(&url);
    let mut client = YSyncClient::connect(config).await?;

    println!("Connected! Syncing document...");

    // Create a local Y.Doc and sync with server
    let doc = yrs::Doc::new();
    client.sync(&doc).await?;

    println!("Synced! is_synced: {}", client.is_synced());

    // First, read the current state to get the cell source ref
    let cells_ref = {
        let txn = doc.transact();
        txn.get_array("cells")
    };

    let Some(cells) = cells_ref else {
        println!("No cells array found!");
        return Ok(());
    };

    // Now modify the first cell's source
    {
        let mut txn = doc.transact_mut();

        println!("\nCells array length: {}", cells.len(&txn));

        if let Some(cell_value) = cells.get(&txn, 0) {
            println!("Got first cell");
            if let yrs::Out::YMap(cell_map) = cell_value {
                // Get the source Y.Text
                if let Some(yrs::Out::YText(source)) = cell_map.get(&txn, "source") {
                    let old_content = source.get_string(&txn);
                    println!("Current cell content: {}", old_content);

                    // Clear and set new content
                    let len = source.len(&txn);
                    source.remove_range(&mut txn, 0, len);
                    source.insert(&mut txn, 0, "print(\"Hello from Rust! 🦀\")");

                    println!("Modified cell to: print(\"Hello from Rust! 🦀\")");
                } else {
                    println!("source not found in cell map");
                }
            } else {
                println!("cell is not a YMap");
            }
        } else {
            println!("No cell at index 0");
        }
    }

    // Send the update to the server
    println!("\nSending update to server...");
    client.send_update(&doc).await?;

    println!("Done! Check your JupyterLab - the cell should have changed.");

    // Keep connection open briefly so the update propagates
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    client.close().await?;
    Ok(())
}
