//! An HTTPS and Websocket wrapper that allows users to write trading bots for the [Kalshi events trading platform](https://kalshi.com).
//!
//! kalshi-rust is asynchronous, performant, and succinct. Dash past verbose and annoying HTTPS requests
//! and use this wrapper to quickly write blazingly fast trading bots in Rust!
//!
//! ## The Kalshi Struct
//!
//! The [Kalshi](Kalshi) struct is the central component of this crate.
//! All authentication, order routing, market requests, and position snapshots are handled through the struct and its methods.
//!
//! For more details, see [Kalshi](Kalshi).
//!
//! ## Quick Start Guide
//!
//! First, list the Kalshi struct as a dependency in your crate.
//!
//! ```toml
//! kalshi = { version = "0.9"}
//! ```
//!
//! Initialize the Kalshi Struct using your API key details from the Kalshi profile page:
//!
//! ```
//! use kalshi::{Kalshi, TradingEnvironment};
//!
//! let key_id = "your-api-key-id".to_string();
//! let private_key = "your-pem-formatted-private-key".to_string();
//!
//! let mut kalshi_instance = Kalshi::new(
//!     TradingEnvironment::DemoMode,
//!     key_id,
//!     private_key
//! );
//! ```
//!
//! After initializing, you can call any method present in the crate.
//! Here is a script that buys a 'yes' contract on a New York temperature market.
//!
//! ```
//! # use kalshi::{Kalshi, TradingEnvironment, CreateOrderPayload, Action, Side};
//! # async fn example(kalshi_instance: &Kalshi) {
//! let new_york_ticker = "HIGHNY-23NOV13-T51".to_string();
//!
//! let bought_order = kalshi_instance
//!     .create_order(CreateOrderPayload {
//!         action: Action::Buy,
//!         client_order_id: None,
//!         count: Some(1),
//!         count_fp: None,
//!         side: Side::Yes,
//!         ticker: new_york_ticker,
//!         r#type: "limit".to_string(),
//!         buy_max_cost: None,
//!         expiration_ts: None,
//!         no_price: None,
//!         yes_price: Some(5),
//!         no_price_dollars: None,
//!         yes_price_dollars: None,
//!         order_group_id: None,
//!         post_only: None,
//!         self_trade_prevention_type: None,
//!         time_in_force: None,
//!         subaccount: None,
//!     }).await.unwrap();
//! # }
//! ```

use std::sync::Arc;

#[macro_use]
mod utils;
mod api_keys;
mod communications;
mod event;
mod exchange;
mod historical;
mod http;
mod kalshi_error;
mod market;
mod multivariate;
mod portfolio;
mod series;
#[cfg(feature = "websockets")]
mod websockets;

pub use api_keys::*;
pub use communications::*;
pub use event::*;
pub use exchange::*;
pub use historical::*;
pub use kalshi_error::*;
pub use market::*;
pub use multivariate::*;
pub use portfolio::*;
pub use series::*;

#[cfg(feature = "websockets")]
pub use websockets::*;

use openssl::{
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Padding,
    sign::{RsaPssSaltlen, Signer},
};

/// The Kalshi struct is the core of the kalshi-crate. It acts as the interface
/// between the user and the market, abstracting away the meat of requests
/// by encapsulating authentication information and the client itself.
#[derive(Clone)]
pub struct Kalshi {
    /// The base URL for the API, determined by the trading environment.
    base_url: String,
    #[cfg(feature = "websockets")]
    ws_url: String,
    /// Identifier for the authenticated user.
    member_id: Option<String>,
    /// The HTTP client used for making requests.
    client: reqwest::Client,
    /// Stores the method of authentication and required keys.
    auth: KalshiAuth,
}

pub enum KalshiAuth {
    ApiKey {
        /// UUID of the key from the Kalshi profile page.
        key_id: String,
        /// PEM formatted RSA private key.
        key: String,
        /// The loaded private key.
        p_key: Arc<PKey<Private>>,
        /// The RSA signer used for authentication headers.
        signer: Signer<'static>,
    },
}

impl Clone for KalshiAuth {
    fn clone(&self) -> Self {
        match self {
            KalshiAuth::ApiKey { key_id, key, .. } => {
                KalshiAuth::build_api_key(key_id.clone(), key.clone())
            }
        }
    }
}

impl KalshiAuth {
    fn build_api_key(key_id: String, key: String) -> Self {
        let p_key = PKey::private_key_from_pem(key.as_bytes())
            .expect("Unable to load private key from PEM string provided");
        let mut signer = Signer::new(MessageDigest::sha256(), &p_key)
            .expect("Unable to create signer from private key");
        signer
            .set_rsa_padding(Padding::PKCS1_PSS)
            .expect("Unable to set RSA padding on signer");
        signer
            .set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)
            .expect("Unable to set RSA PSS salt length for signer");
        KalshiAuth::ApiKey {
            key_id,
            key,
            p_key: Arc::new(p_key),
            signer,
        }
    }
}

impl Kalshi {
    /// Creates a new instance of Kalshi with the specified trading environment and API key details.
    ///
    /// # Arguments
    ///
    /// * `trading_env` - The trading environment to be used.
    /// * `key_id` - ID of the api key from the Kalshi profile page.
    /// * `key` - PEM formatted RSA private key from the Kalshi profile page.
    pub fn new(trading_env: TradingEnvironment, key_id: String, key: String) -> Self {
        Kalshi {
            base_url: utils::build_base_url(trading_env).to_string(),
            #[cfg(feature = "websockets")]
            ws_url: utils::build_ws_url(trading_env).to_string(),
            member_id: None,
            client: reqwest::Client::new(),
            auth: KalshiAuth::build_api_key(key_id, key),
        }
    }

    /// Alias for `new`.
    pub fn new_with_api_key(trading_env: TradingEnvironment, key_id: String, key: String) -> Self {
        Self::new(trading_env, key_id, key)
    }

    /// Retrieves the currently set base url.
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}

// GENERAL ENUMS
// -----------------------------------------------

/// Defines the trading environment for the Kalshi exchange.
#[derive(Clone, Copy, Debug)]
pub enum TradingEnvironment {
    /// Demo mode represents a simulated environment where trades do not involve real money.
    DemoMode,
    /// Live market mode is the real trading environment.
    LiveMarketMode,
    /// Legacy only markets.
    LegacyLiveMarketMode,
}