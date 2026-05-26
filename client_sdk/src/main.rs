use std::time::Duration;
use tokio::time::sleep;
use client_sdk::LiveConfigClient;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{

    let client = LiveConfigClient::new("http://127.0.0.1:3000");
    println!("📡 Connecting to the Live Config Server...");

    client.establish_sync().await?;
    println!("✅ Synchronization established! Monitoring 'premium_features' state...");
    println!("💡 Tip: Send a POST request to the backend to watch this change live.\n");

    loop {
        
        if client.is_enabled("premium_features") {
            println!("[STATUS] 🚀 Premium Features are: ENABLED");
        } else {
            println!("[STATUS] 🔒 Premium Features are: DiSABLED");
        }

        sleep(Duration::from_secs(1)).await;

    }


}