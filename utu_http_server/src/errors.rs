use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Transaction not found")]
    TransactionNotFound,
    #[error("Error fetching from Bitcoin Core: {0}")]
    BitcoinCoreError(#[from] bitcoincore_rpc::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::TransactionNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BitcoinCoreError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(serde_json::json!({ "error": error_message }))).into_response()
    }
}
