use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub name: String,
    pub enabled: bool,
    pub value: String,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Header {
    Broadcast,
    Greeting,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMessage {
    pub header: Header,
    pub payload: Option<FeatureFlag>,
}