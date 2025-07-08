// filepath: /solana-qubic-bridge/solana-qubic-bridge/src/relayer/src/main.rs
use std::error::Error;
use tokio::task;
use tokio::time::{sleep, Duration};

mod qubic_monitor;
mod solana_monitor;
mod bridge_relayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let qubic_task = task::spawn(async {
        qubic_monitor::monitor().await;
    });

    let solana_task = task::spawn(async {
        solana_monitor::monitor().await;
    });

    let relayer_task = task::spawn(async {
        bridge_relayer::relay().await;
    });

    // Wait for all tasks to complete
    let _ = tokio::join!(qubic_task, solana_task, relayer_task);

    // Keep the main thread alive
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}