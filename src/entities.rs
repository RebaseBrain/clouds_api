use std::collections::BTreeMap;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ConfigCreateRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub r_type: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct RemoteConfig {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub profiles: BTreeMap<String, String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RcloneOption {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Help")]
    pub help: String,
    #[serde(rename = "Required")]
    pub required: bool,
}

#[derive(Deserialize, Debug)]
pub struct RcloneProvider {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Options")]
    pub options: Vec<RcloneOption>,
}

#[derive(Deserialize, Debug)]
pub struct ProvidersResponse {
    pub providers: Vec<RcloneProvider>,
}
