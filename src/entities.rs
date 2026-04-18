use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Debug, Deserialize)]
pub struct CreateParameters {
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl CreateParameters {
    pub fn into_string_map(self) -> HashMap<String, String> {
        self.extra
            .into_iter()
            .map(|(key, value)| {
                let val = match value {
                    Value::String(s) => s,
                    _ => value.to_string(),
                };

                (key, val)
            })
            .collect()
    }
}
