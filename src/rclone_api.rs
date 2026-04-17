use crate::{
    entities::{ConfigCreateRequest, ListRemotesResponse},
    error::CloudeError,
};
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::collections::HashMap;
type Result<T> = std::result::Result<T, CloudeError>;

pub trait RcloneApi {
    fn list_profiles(&self) -> impl Future<Output = Result<Vec<String>>>;
    fn config_create(
        &self,
        profile_name: &str,
        domen: &str,
    ) -> impl Future<Output = Result<String>>;
    fn delete_profile(&self, profile_name: &str) -> impl Future<Output = Result<String>>;
    fn mount(
        &self,
        profile_name: &str,
        domen: &str,
        file_path: &str,
    ) -> impl Future<Output = Result<String>>;
    fn link(&self, profile_name: &str, path: &str) -> impl Future<Output = Result<String>>;
}

pub struct RcClone {
    pub client: Client,
    pub url: String,
}

impl RcloneApi for RcClone {
    async fn list_profiles(&self) -> Result<Vec<String>> {
        let response = self
            .client
            .post(format!("{}config/listremotes", self.url))
            .send()
            .await
            .map_err(CloudeError::ReqwestError)?;

        let data: ListRemotesResponse = response
            .json()
            .await
            .map_err(|err| CloudeError::RcloneError((StatusCode::IM_A_TEAPOT, err.to_string())))?;

        Ok(data.remotes)
    }

    async fn config_create(&self, profile_name: &str, domen: &str) -> Result<String> {
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
            .map_err(CloudeError::ReqwestError)?;

        if response.status().is_success() {
            Ok(format!("Success: Profile {} created", profile_name))
        } else {
            Err(CloudeError::RcloneError((
                StatusCode::CONFLICT,
                "Failed to create profile".into(),
            )))
        }
    }

    async fn delete_profile(&self, profile_name: &str) -> Result<String> {
        let body = HashMap::from([("name", profile_name)]);

        self.client
            .post(format!("{}config/delete", self.url))
            .json(&body)
            .send()
            .await
            .map_err(CloudeError::ReqwestError)?;

        Ok(format!("Success: Profile {} deleted", profile_name))
    }

    async fn mount(&self, profile_name: &str, _domen: &str, file_path: &str) -> Result<String> {
        let body = json!({
            "fs": format!("{}:/", profile_name),
            "mountPoint": format!("{}", file_path),
            "vfsOpt": {
                "CacheMode": "full",
                "CacheMaxAge": "3600s",
                "CacheMaxSize": "10G",
                "CachePollInterval": "1m"
            }
        });

        self.client
            .post(format!("{}mount/mount", self.url))
            .json(&body)
            .send()
            .await
            .map_err(CloudeError::ReqwestError)?;

        Ok(format!("Mounting {} started", profile_name))
    }

    async fn link(&self, profile_name: &str, path: &str) -> Result<String> {
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
            .map_err(CloudeError::ReqwestError)?;

        // Читаем весь JSON для отладки
        let res_json: serde_json::Value = response
            .json()
            .await
            .map_err(|err| CloudeError::RcloneError((StatusCode::IM_A_TEAPOT, err.to_string())))?;

        // Печатаем в консоль Rust-приложения, что прислал rclone
        println!("Rclone link response: {:?}", res_json);

        match res_json["url"].as_str() {
            Some(url) => Ok(url.to_string()),
            None => Err(CloudeError::RcloneError((
                StatusCode::NOT_FOUND,
                "No link generated".to_string(),
            ))),
        }
    }
}
