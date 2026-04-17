use reqwest::StatusCode;
use thiserror::Error;

use crate::json_result::to_err;

#[derive(Error, Debug)]
pub enum CloudError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Rclone error")]
    RcloneError((StatusCode, String)),
}

impl From<CloudError> for String {
    fn from(value: CloudError) -> Self {
        match value {
            CloudError::ReqwestError(err) => {
                to_err(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
            }
            CloudError::RcloneError((status, err)) => to_err(status, &err),
        }
    }
}
