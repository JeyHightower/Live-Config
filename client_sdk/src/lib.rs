use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use shared::FeatureFlag;


pub struct LivConfigClient {
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

    pub async fn establish_sync(&self) -> Result


}