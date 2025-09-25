use serde::{Deserialize, Serialize};

pub mod commands;

pub mod client;

#[allow(dead_code)]
pub mod responses;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KalshiChannel {
    OrderbookDelta,
    Ticker,
    Trade,
    Fill,
    MarketLifecycle,
    MarketLifecycleV2,
    MarketPositions,
    Multivariate,
}

impl KalshiChannel {
    const fn as_str(&self) -> &'static str {
        match self {
            KalshiChannel::OrderbookDelta => "orderbook_delta",
            KalshiChannel::Ticker => "ticker",
            KalshiChannel::Trade => "trade",
            KalshiChannel::Fill => "fill",
            KalshiChannel::MarketLifecycle => "market_lifecycle",
            KalshiChannel::MarketLifecycleV2 => "market_lifecycle_v2",
            KalshiChannel::MarketPositions => "market_positions",
            KalshiChannel::Multivariate => "multivariate",
        }
    }
}

impl From<KalshiChannel> for &'static str {
    fn from(val: KalshiChannel) -> Self {
        val.as_str()
    }
}
