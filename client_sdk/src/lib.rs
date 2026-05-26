use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use shared::FeatureFlag;


pub struct LiveConfigClient {
    pub url: String,
    pub cache: Arc<RwLock<HashMap<String, FeatureFlag>>>,
}

impl LiveConfigClient {

    pub fn new(server_url: &str) -> Self {
        Self {
            url: server_url.to_string(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn establish_sync(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        let http_url = format!("{}/api/flags", self.url);
        let response = reqwest::get(&http_url).await?.json::<Vec<FeatureFlag>>().await?;
    
        {
            let mut write_lock = self.cache.write().unwrap();
            for flag in response {
                write_lock.insert(flag.name.clone(), flag);
            }
        }

        let ws_url = format!{"{}/ws", self.url}
            .replace("http://", "ws://")
            .replace("https://", "wss://");

        let (ws_stream, _) = tokio_tungstenite::connect_async(ws_url).await?;

        use futures_util::StreamExt;
        let(_, mut read_stream) = ws_stream.split();

        let local_cache = Arc::clone(&self.cache);

        tokio::spawn(async move {
            while let Some(Ok(msg)) = read_stream.next().await {
                if let Ok(text) = msg.to_text() {
                    if let Ok(incoming_flag) = serde_json::from_str::<FeatureFlag>(text) {
                        let mut write_lock = local_cache.write().unwrap();
                        write_lock.insert(incoming_flag.name.clone(), incoming_flag);
                    }
                }
            }
        });

        Ok(())

    
    }


    pub fn is_enabled(&self, flag_name:&str) -> bool{
        let read_lock = self.cache.read().unwrap();
        read_lock
            .get(flag_name)
            .map(|flag| flag.is_enabled)
            .unwrap_or(false)
    }
}