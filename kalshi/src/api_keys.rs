use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves a list of all API keys associated with the user.
    ///
    /// Maps to GET /api_keys
    pub async fn get_api_keys(&self) -> Result<Vec<ApiKey>, KalshiError> {
        let url = self.build_url("/api_keys")?;
        let resp: GetApiKeysResponse = self.http_get(url).await?;
        Ok(resp.api_keys)
    }

    /// Creates a new API key using a provided RSA public key.
    ///
    /// Maps to POST /api_keys
    pub async fn create_api_key(&self, payload: CreateApiKeyRequest) -> Result<String, KalshiError> {
        let url = self.build_url("/api_keys")?;
        let resp: CreateApiKeyResponse = self.http_post(url, &payload).await?;
        Ok(resp.api_key_id)
    }

    /// Generates a new API key and returns both the ID and the RSA private key.
    /// The private key cannot be retrieved again after this response.
    ///
    /// Maps to POST /api_keys/generate
    pub async fn generate_api_key(&self, payload: GenerateApiKeyRequest) -> Result<GenerateApiKeyResponse, KalshiError> {
        let url = self.build_url("/api_keys/generate")?;
        self.http_post(url, &payload).await
    }

    /// Deletes an API key by its ID.
    ///
    /// Maps to DELETE /api_keys/{api_key}
    pub async fn delete_api_key(&self, api_key_id: &str) -> Result<(), KalshiError> {
        let path = format!("/api_keys/{}", api_key_id);
        let url = self.build_url(&path)?;
        self.http_delete(url).await
    }
}

// Internal Response Structs

#[derive(Debug, Deserialize)]
struct GetApiKeysResponse {
    pub api_keys: Vec<ApiKey>,
}

#[derive(Debug, Deserialize)]
struct CreateApiKeyResponse {
    pub api_key_id: String,
}

// Public Payloads and Data Structures

/// Represents an API key in the Kalshi system.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiKey {
    pub api_key_id: String,
    pub name: String,
    pub scopes: Vec<String>,
}

/// Request payload for creating an API key with an existing public key.
#[derive(Debug, Serialize, Clone)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

/// Request payload for generating a new API key.
#[derive(Debug, Serialize, Clone)]
pub struct GenerateApiKeyRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

/// Response returned when an API key is generated.
/// Contains the private key which must be saved immediately.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GenerateApiKeyResponse {
    pub api_key_id: String,
    pub private_key: String,
}