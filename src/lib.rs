use zbus::interface;
mod zbus_error;

trait RcloneApi {
    /// Получене списка всех профилей
    async fn list_profiles(&self) -> zbus::fdo::Result<Vec<String>>;
    /// Создать профиль
    /// - domen отвечает за облако. Например yandex связывает профиль с яндексом
    /// - После исполнения происходит редирект в браузер для авторизации
    /// String - success/error
    async fn config_create(&self, profile_name: &str, domen: &str) -> zbus::fdo::Result<String>;
    async fn delete_create(&self, profile_name: &str) -> zbus::fdo::Result<String>;
    async fn mount(&self, profile_name: &str, domen: &str) -> zbus::fdo::Result<String>;
    async fn link(&self, profile_name: &str, path: &str) -> zbus::fdo::Result<String>;
    async fn link(&self, profile_name: &str, path: &str) -> zbus::fdo::Result<String>;

}

pub struct RcClone {
    pub client: Client,
    pub url: String,
}
use serde::Deserialize;

use crate::zbus_error::CloudsErrors;

#[derive(Deserialize)]
struct ListRemotesResponse {
    remotes: Vec<String>,
}

#[interface(name = "org.zbus.cloud_api")]
impl RcloneApi for RcClone {
    async fn list_profiles(&self) -> String {
        // TODO: немного фиксануть надо
        let response = self
            .client
            .post(format!("{}config/listremotes", self.url))
            .send()
            .await
            .map_err(CloudsErrors::ReqwestError)?;

        Ok(response
            .json::<ListRemotesResponse>()
            .await
            .map_err(CloudsErrors::ReqwestError)?
            .remotes)
    }
}

