use crate::FeatureFlag;
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, RwLock};


pub struct StorageEngine {
    cache: Arc<RwLock<HashMap<String,FeatureFlag>>>,
    log_file: Arc<RwLock<File>> 
}


impl StorageEngine {
    pub fn new(path: &str) -> Self {

    }

    pub fn set_flag(&self, flag: FeatureFlag){
        
    }

    pub fn get_flag(&self, name: &str) -> Option<FeatureFlag>{
        let cache_lock = self.cache.read().unwrap();

        cache_lock.get(name).cloned();
    }
}