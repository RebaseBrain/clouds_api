use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListRemotesResponse {
    pub remotes: Vec<String>,
}

#[derive(Serialize)]
pub struct ConfigCreateRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub r_type: String,
    pub parameters: HashMap<String, String>,
}
