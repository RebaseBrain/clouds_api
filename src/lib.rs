use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::interface;

pub mod zbus_error;
use crate::zbus_error::CloudsErrors;

#[derive(Deserialize)]
struct ListRemotesResponse {
    remotes: Vec<String>,
}

#[derive(Serialize)]
struct ConfigCreateRequest {
    name: String,
    #[serde(rename = "type")]
    r_type: String,
    parameters: HashMap<String, String>,
}

pub trait RcloneApi {
    async fn list_profiles(&self) -> zbus::fdo::Result<Vec<String>>;
    async fn config_create(&self, profile_name: &str, domen: &str) -> zbus::fdo::Result<String>;
    async fn delete_profile(&self, profile_name: &str) -> zbus::fdo::Result<String>;
    async fn mount(&self, profile_name: &str, domen: &str) -> zbus::fdo::Result<String>;
    async fn link(&self, profile_name: &str, path: &str) -> zbus::fdo::Result<String>;
}

pub struct RcClone {
    pub client: Client,
    pub url: String,
}

#[interface(name = "org.zbus.cloud_api")]
impl RcloneApi for RcClone {
    async fn list_profiles(&self) -> zbus::fdo::Result<Vec<String>> {
        let response = self
            .client
            .post(format!("{}config/listremotes", self.url))
            .send()
            .await
            .map_err(CloudsErrors::ReqwestError)?;

        let data: ListRemotesResponse =
            response.json().await.map_err(CloudsErrors::ReqwestError)?;

        Ok(data.remotes)
    }

    async fn config_create(&self, profile_name: &str, domen: &str) -> zbus::fdo::Result<String> {
        let body = ConfigCreateRequest {
            name: profile_name.to_string(),
            r_type: domen.to_string(),
            parameters: HashMap::new(),
        };

        let response = self
            .client
            .post(format!("{}config/create", self.url))
            .json(&body)
            .send()
            .await
            .map_err(CloudsErrors::ReqwestError)?;

        if response.status().is_success() {
            Ok(format!("Success: Profile {} created", profile_name))
        } else {
            Err(zbus::fdo::Error::Failed("Failed to create profile".into()))
        }
    }

    async fn delete_profile(&self, profile_name: &str) -> zbus::fdo::Result<String> {
        let body = HashMap::from([("name", profile_name)]);

        self.client
            .post(format!("{}config/delete", self.url))
            .json(&body)
            .send()
            .await
            .map_err(CloudsErrors::ReqwestError)?;

        Ok(format!("Success: Profile {} deleted", profile_name))
    }

    async fn mount(&self, profile_name: &str, _domen: &str) -> zbus::fdo::Result<String> {
        let body = HashMap::from([
            ("fs", profile_name.to_string() + ":"),
            ("mountPoint", format!("/tmp/{}", profile_name)),
        ]);

        self.client
            .post(format!("{}mount/mount", self.url))
            .json(&body)
            .send()
            .await
            .map_err(CloudsErrors::ReqwestError)?;

        Ok(format!("Mounting {} started", profile_name))
    }

    async fn link(&self, profile_name: &str, path: &str) -> zbus::fdo::Result<String> {
        let body = HashMap::from([
            ("fs", profile_name.to_string() + ":"),
            ("remote", path.to_string()),
        ]);

        let response = self
            .client
            .post(format!("{}operations/publiclink", self.url))
            .json(&body)
            .send()
            .await
            .map_err(CloudsErrors::ReqwestError)?;

        // Читаем весь JSON для отладки
        let res_json: serde_json::Value =
            response.json().await.map_err(CloudsErrors::ReqwestError)?;

        // Печатаем в консоль Rust-приложения, что прислал rclone
        println!("Rclone link response: {:?}", res_json);

        Ok(res_json["url"]
            .as_str()
            .unwrap_or("No link generated")
            .to_string())
    }
}
