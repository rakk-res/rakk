mod core;
mod slots;
mod config;
mod utils;

use crate::core::frame::Frame;
use crate::config::Config;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("Initializing Rakk Infrastructure v0.1.0");
    
    let config = Config::from_env().unwrap_or_else(|e| {
        warn!("Config load failed: {}, using defaults", e);
        Config { frame_id: "default".into(), log_level: "info".into() }
    });

    let _frame = Frame::boot(config);
    
    info!("System Online. Waiting for Blades...");
    
    // Keep main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
