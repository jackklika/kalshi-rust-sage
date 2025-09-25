use crate::kalshi_error::KalshiError;
use crate::kalshi_error::RequestError;
use crate::utils::api_key_headers;
use crate::KalshiAuth;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Padding;
use openssl::sign::{RsaPssSaltlen, Signer};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Method;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::{debug, error, info, warn};

use super::Kalshi;

impl Kalshi {
    fn auth_headers(&self, path: &str, method: Method) -> HeaderMap {
        let mut headers = HeaderMap::new(); // Initialize HeaderMap here
        match &self.auth {
            KalshiAuth::ApiKey { key_id, key, .. } => {
                let pkey = PKey::private_key_from_pem(key.as_bytes()).unwrap();
                let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
                signer.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
                signer.set_rsa_mgf1_md(MessageDigest::sha256()).unwrap();
                signer
                    .set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)
                    .unwrap();
                let api_headers = api_key_headers(key_id, &mut signer, path, method).unwrap();
                for (key_str, value_string) in api_headers {
                    headers.insert(
                        HeaderName::from_static(key_str),
                        HeaderValue::from_str(&value_string).unwrap(),
                    );
                }
            }
            KalshiAuth::EmailPassword => {
                headers.insert(
                    HeaderName::from_static("Authorization"),
                    HeaderValue::from_str(
                        &self
                            .curr_token
                            .clone()
                            .expect("Token not found with EmailPassword auth"),
                    )
                    .unwrap(),
                );
            }
        }
        headers // Return the HeaderMap
    }

    pub async fn http_get<T: DeserializeOwned>(&self, url: Url) -> Result<T, KalshiError> {
        let resp = self
            .client
            .get(url.clone())
            .headers(self.auth_headers(url.path(), Method::GET))
            .send()
            .await?;

        self.process_response::<T>("GET", &url, None, resp).await
    }
    pub async fn http_post<B, T>(&self, url: Url, body: &B) -> Result<T, KalshiError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let resp = self
            .client
            .post(url.clone())
            .headers(self.auth_headers(url.path(), Method::POST))
            .json(body)
            .send()
            .await?;

        let req_body_string =
            serde_json::to_string(body).unwrap_or_else(|_| "<unserializable body>".to_string());
        self.process_response::<T>("POST", &url, Some(req_body_string), resp)
            .await
    }
    pub async fn http_delete<T: DeserializeOwned>(&self, url: Url) -> Result<T, KalshiError> {
        let resp = self
            .client
            .delete(url.clone())
            .headers(self.auth_headers(url.path(), Method::DELETE))
            .send()
            .await?;

        self.process_response::<T>("DELETE", &url, None, resp).await
    }

    // Internal: process an HTTP response with debug/info logging and JSON deserialization.
    async fn process_response<T: DeserializeOwned>(
        &self,
        method: &str,
        url: &Url,
        request_body: Option<String>,
        resp: reqwest::Response,
    ) -> Result<T, KalshiError> {
        let status = resp.status();
        let bytes = resp.bytes().await?;

        if !status.is_success() {
            match request_body {
                Some(body) => {
                    if status.is_client_error() {
                        warn!(
                            "HTTP {} non-success: url={}, status={}, request_body={}, response_body={}",
                            method,
                            url,
                            status,
                            body,
                            String::from_utf8_lossy(&bytes)
                        );
                    } else if status.is_server_error() {
                        error!(
                            "HTTP {} non-success: url={}, status={}, request_body={}, response_body={}",
                            method,
                            url,
                            status,
                            body,
                            String::from_utf8_lossy(&bytes)
                        );
                    } else {
                        info!(
                            "HTTP {} non-success: url={}, status={}, request_body={}, response_body={}",
                            method,
                            url,
                            status,
                            body,
                            String::from_utf8_lossy(&bytes)
                        );
                    }
                }
                None => {
                    if status.is_client_error() {
                        warn!(
                            "HTTP {} non-success: url={}, status={}, response_body={}",
                            method,
                            url,
                            status,
                            String::from_utf8_lossy(&bytes)
                        );
                    } else if status.is_server_error() {
                        error!(
                            "HTTP {} non-success: url={}, status={}, response_body={}",
                            method,
                            url,
                            status,
                            String::from_utf8_lossy(&bytes)
                        );
                    } else {
                        info!(
                            "HTTP {} non-success: url={}, status={}, response_body={}",
                            method,
                            url,
                            status,
                            String::from_utf8_lossy(&bytes)
                        );
                    }
                }
            }
        } else {
            debug!("{} {} -> {}", method, url, status);
            debug!("Response body: {}", String::from_utf8_lossy(&bytes));
        }

        if !status.is_success() {
            return Err(KalshiError::InternalError(format!(
                "Non-success status {}. Body: {}",
                status,
                String::from_utf8_lossy(&bytes)
            )));
        }

        serde_json::from_slice::<T>(&bytes).map_err(|e| {
            KalshiError::InternalError(format!(
                "Deserialize error: {}. Body: {}",
                e,
                String::from_utf8_lossy(&bytes)
            ))
        })
    }

    /// Helper function to build a URL with query parameters.
    /// It takes a base path (relative to self.base_url) and a vector of parameters.
    ///
    /// # Arguments
    /// * `base_path` - The path segment of the URL (e.g., "/markets/trades").
    /// * `params` - A vector of key-value pairs for query parameters.
    ///
    /// # Returns
    /// - `Ok(reqwest::Url)`: A fully constructed and parsed URL on success.
    /// - `Err(reqwest::Error)`: An error if URL parsing fails.
    pub fn build_url_with_params(
        &self,
        base_path: &str,
        params: Vec<(&str, String)>,
    ) -> Result<Url, KalshiError> {
        let base_url_str = format!("{}{}", self.base_url, base_path);
        Url::parse_with_params(&base_url_str, &params).map_err(|err| {
            // Convert url::ParseError to KalshiError::RequestError
            KalshiError::RequestError(RequestError::UrlParseError(err))
        })
    }
    pub fn build_url(&self, base_path: &str) -> Result<Url, KalshiError> {
        let base_url_str = format!("{}{}", self.base_url, base_path);
        Url::parse(&base_url_str).map_err(|err| {
            // Convert url::ParseError to KalshiError::RequestError
            KalshiError::RequestError(RequestError::UrlParseError(err))
        })
    }
}
