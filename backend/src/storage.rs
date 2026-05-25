use shared::FeatureFlag;
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, RwLock};


pub struct StorageEngine {
    cache: Arc<RwLock<HashMap<String,FeatureFlag>>>,
    log_file: Arc<RwLock<File>> 
}


impl StorageEngine {
    pub fn new(path: &str) -> Self {
        use std::fs::OpenOptions;
        use std::io::{BufRead, BufReader};

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)
            .expect("Failed to open or create the log file");

        let mut raw_map =  HashMap::new();
        let reader = BufReader::new(&file);
        for line in reader.lines(){
            if let Ok(text) = line {
                if let Ok(flag) = serde_json::from_str::<FeatureFlag>(&text){
                    raw_map.insert(flag.name.clone(), flag);
                }
            }
        }

        Self {
            cache: Arc::new(RwLock::new(raw_map)),
            log_file: Arc::new(RwLock::new(file)),
        }

    }

    pub fn set_flag(&self, flag: FeatureFlag){
        let mut cache_lock = self.cache.write().unwrap();
        let mut file_lock = self.log_file.write().unwrap();

        if let Ok(json_line) = serde_json::to_string(&flag){
            use std::io::Write;
            let _ = writeln!(*file_lock, "{}", json_line);
        }
        cache_lock.insert(flag.name.clone(), flag);
    }


    pub fn get_flag(&self, name: &str) -> Option<FeatureFlag>{
        let cache_lock = self.cache.read().unwrap();

        cache_lock.get(name).cloned()
    }

    pub fn get_all_flags(&self) -> Vec<FeatureFlag>{
        let cache_lock = self.cache.read().unwrap();
        cache_lock.values().cloned().collect()

    }
}