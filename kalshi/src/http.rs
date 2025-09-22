use crate::KalshiAuth;
use crate::kalshi_error::RequestError;
use crate::utils::api_key_headers;
use reqwest::Method;
use crate::kalshi_error::KalshiError;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use openssl::pkey::PKey;
use openssl::sign::Signer;
use openssl::hash::MessageDigest;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::Kalshi;

impl Kalshi {

    fn auth_headers(&self, path: &str, method: Method) -> HeaderMap {
        let mut headers = HeaderMap::new(); // Initialize HeaderMap here
        match &self.auth {
            KalshiAuth::ApiKey { key_id, key, ..} => {
                let pkey = PKey::private_key_from_pem(key.as_bytes()).unwrap();
                let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
                let api_headers = api_key_headers(key_id, &mut signer, path, method).unwrap();
                for (key_str, value_string) in api_headers {
                    headers.insert(HeaderName::from_static(key_str), HeaderValue::from_str(&value_string).unwrap());
                }
            }
            KalshiAuth::EmailPassword => {
                headers.insert(HeaderName::from_static("Authorization"), HeaderValue::from_str(&self.curr_token.clone().expect("Token not found with EmailPassword auth")).unwrap());
            }
        }
        headers // Return the HeaderMap
    }

    pub async fn http_get<T: DeserializeOwned>(&self, url: Url) -> Result<T, KalshiError> {
        self
            .client
            .get(url.clone())
            .headers(self.auth_headers(url.path(), Method::GET))
            .send()
            .await?
            .json::<T>() // Deserialize directly to T
            .await
            .map_err(|e| KalshiError::RequestError(RequestError::SerializationError(e))) // Map reqwest::Error to KalshiError
    }
    pub async fn http_post<B, T>(&self, url: Url, body: &B) -> Result<T, KalshiError> where B: Serialize + ?Sized, T: DeserializeOwned {
        self
            .client
            .post(url.clone())
            .headers(self.auth_headers(url.path(), Method::POST)).json(body)
            .send()
            .await?
            .json::<T>() // Deserialize directly to T
            .await
            .map_err(|e| KalshiError::RequestError(RequestError::SerializationError(e))) // Map reqwest::Error to KalshiError
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
        let base_url_str = format!("{}{}", self.base_url.to_string(), base_path);
        Url::parse_with_params(&base_url_str, &params)
            .map_err(|err| {
                // Convert url::ParseError to KalshiError::RequestError
                KalshiError::RequestError(RequestError::UrlParseError(err))
            })
    }
    pub fn build_url(
        &self,
        base_path: &str,
    ) -> Result<Url, KalshiError> {
        let base_url_str = format!("{}{}", self.base_url.to_string(), base_path);
        Url::parse(&base_url_str)
            .map_err(|err| {
                // Convert url::ParseError to KalshiError::RequestError
                KalshiError::RequestError(RequestError::UrlParseError(err))
            })
    }
}
