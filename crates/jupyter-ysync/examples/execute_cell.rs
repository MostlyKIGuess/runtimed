//! Execute a cell in a Jupyter notebook via the kernel websocket.
//!
//! This example connects to a running kernel and executes code, showing the output.
//!
//! Usage:
//!   cargo run -p jupyter-ysync --features client --example execute_cell

use futures::{SinkExt, StreamExt};
use jupyter_protocol::{ExecuteRequest, JupyterMessageContent};
use jupyter_websocket_client::RemoteServer;

const JUPYTER_URL: &str = "http://localhost:18888";
const TOKEN: &str = "testtoken123";
const KERNEL_ID: &str = "95daa175-62e8-44a6-a6dc-0dd7e95195b7";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = RemoteServer {
        base_url: JUPYTER_URL.to_string(),
        token: TOKEN.to_string(),
    };

    println!("Connecting to kernel {}...", KERNEL_ID);

    let (kernel_socket, _response) = server.connect_to_kernel(KERNEL_ID).await?;
    let (mut writer, mut reader) = kernel_socket.split();

    println!("Connected! Executing code...");

    // Send execute request
    let execute_request = ExecuteRequest {
        code: "print(\"Hello from Rust! 🦀\")".to_string(),
        silent: false,
        store_history: true,
        user_expressions: Default::default(),
        allow_stdin: false,
        stop_on_error: true,
    };

    writer.send(execute_request.into()).await?;

    println!("Waiting for output...\n");

    // Read responses until we get execute_reply
    while let Some(response) = reader.next().await {
        let msg = response?;
        match msg.content {
            JupyterMessageContent::StreamContent(stream) => {
                print!("{}", stream.text);
            }
            JupyterMessageContent::ExecuteResult(result) => {
                println!("Result: {:?}", result.data);
            }
            JupyterMessageContent::ErrorOutput(error) => {
                println!("Error: {}", error.evalue);
                for line in &error.traceback {
                    println!("{}", line);
                }
            }
            JupyterMessageContent::ExecuteReply(reply) => {
                println!("\nExecution complete! Status: {:?}", reply.status);
                break;
            }
            JupyterMessageContent::Status(status) => {
                println!("[Kernel {:?}]", status.execution_state);
            }
            _ => {}
        }
    }

    Ok(())
}
