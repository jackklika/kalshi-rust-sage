use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves information about a single event by its ticker.
    pub async fn get_single_event(&self, event_ticker: &str) -> Result<Event, KalshiError> {
        let path = format!("/events/{}", event_ticker);
        let url = self.build_url(&path)?;
        let resp: SingleEventResponse = self.http_get(url).await?;
        Ok(resp.event)
    }

    /// Retrieves information about a single market by its ticker.
    pub async fn get_single_market(&self, market_ticker: &str) -> Result<Market, KalshiError> {
        let path = format!("/markets/{}", market_ticker);
        let url = self.build_url(&path)?;
        let resp: SingleMarketResponse = self.http_get(url).await?;
        Ok(resp.market)
    }

    /// Retrieves multiple markets with various filters.
    pub async fn get_multiple_markets(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        status: Option<String>,
        series_ticker: Option<String>,
        event_ticker: Option<String>,
        max_close_ts: Option<i64>,
        min_close_ts: Option<i64>,
        tickers: Option<String>,
    ) -> Result<(Vec<Market>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "status", status);
        add_param!(params, "series_ticker", series_ticker);
        add_param!(params, "event_ticker", event_ticker);
        add_param!(params, "max_close_ts", max_close_ts);
        add_param!(params, "min_close_ts", min_close_ts);
        add_param!(params, "tickers", tickers);

        let url = self.build_url_with_params("/markets", params)?;
        let resp: PublicMarketsResponse = self.http_get(url).await?;
        Ok((resp.markets, resp.cursor))
    }

    /// Retrieves multiple events with various filters.
    pub async fn get_multiple_events(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        status: Option<String>,
        series_ticker: Option<String>,
    ) -> Result<(Vec<Event>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "status", status);
        add_param!(params, "series_ticker", series_ticker);

        let url = self.build_url_with_params("/events", params)?;
        let resp: PublicEventsResponse = self.http_get(url).await?;
        Ok((resp.events, resp.cursor))
    }

    /// Retrieves series information by ticker.
    pub async fn get_series(&self, series_ticker: &str) -> Result<Series, KalshiError> {
        let path = format!("/series/{}", series_ticker);
        let url = self.build_url(&path)?;
        let resp: SeriesResponse = self.http_get(url).await?;
        Ok(resp.series)
    }

    /// Retrieves the orderbook for a specific market.
    pub async fn get_market_orderbook(&self, market_ticker: &str, depth: Option<i32>) -> Result<Orderbook, KalshiError> {
        let path = format!("/markets/{}/orderbook", market_ticker);
        let mut params = Vec::new();
        add_param!(params, "depth", depth);
        let url = self.build_url_with_params(&path, params)?;
        let resp: OrderBookResponse = self.http_get(url).await?;
        Ok(resp.orderbook)
    }

    /// Retrieves market price history (candlesticks).
    pub async fn get_market_history(
        &self,
        market_ticker: &str,
        start_ts: Option<i64>,
        end_ts: Option<i64>,
        limit: Option<i64>,
    ) -> Result<(String, Vec<Snapshot>), KalshiError> {
        let path = format!("/markets/{}/history", market_ticker);
        let mut params = Vec::new();
        add_param!(params, "start_ts", start_ts);
        add_param!(params, "end_ts", end_ts);
        add_param!(params, "limit", limit);

        let url = self.build_url_with_params(&path, params)?;
        let resp: MarketHistoryResponse = self.http_get(url).await?;
        Ok((resp.ticker, resp.history))
    }

    /// Retrieves public trades for one or more markets.
    pub async fn get_trades(
        &self,
        tickers: Option<String>,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Vec<Trade>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "tickers", tickers);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let url = self.build_url_with_params("/markets/trades", params)?;
        let resp: PublicTradesResponse = self.http_get(url).await?;
        Ok((resp.trades, resp.cursor))
    }
}

// Structs for API responses

#[derive(Debug, Deserialize)]
struct SingleEventResponse {
    pub event: Event,
}

#[derive(Debug, Deserialize)]
struct SingleMarketResponse {
    pub market: Market,
}

#[derive(Debug, Deserialize)]
struct PublicMarketsResponse {
    pub cursor: Option<String>,
    pub markets: Vec<Market>,
}

#[derive(Debug, Deserialize)]
struct PublicEventsResponse {
    pub cursor: Option<String>,
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct SeriesResponse {
    pub series: Series,
}

#[derive(Debug, Deserialize)]
struct OrderBookResponse {
    pub orderbook: Orderbook,
}

#[derive(Debug, Deserialize)]
struct MarketHistoryResponse {
    pub cursor: Option<String>,
    pub ticker: String,
    pub history: Vec<Snapshot>,
}

#[derive(Debug, Deserialize)]
struct PublicTradesResponse {
    pub cursor: Option<String>,
    pub trades: Vec<Trade>,
}

// Data structures

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Market {
    pub ticker: String,
    pub event_ticker: String,
    pub market_type: String,
    #[deprecated]
    pub title: String,
    #[deprecated]
    pub subtitle: String,
    pub yes_sub_title: String,
    pub no_sub_title: String,
    pub created_time: String,
    pub updated_time: String,
    pub open_time: String,
    pub close_time: String,
    pub expected_expiration_time: Option<String>,
    #[deprecated]
    pub expiration_time: Option<String>,
    pub latest_expiration_time: String,
    pub settlement_timer_seconds: i64,
    pub status: String,
    #[deprecated]
    pub response_price_units: String,
    #[deprecated]
    pub yes_bid: f64,
    pub yes_bid_dollars: Option<String>,
    pub yes_bid_size_fp: Option<String>,
    #[deprecated]
    pub yes_ask: f64,
    pub yes_ask_dollars: Option<String>,
    pub yes_ask_size_fp: Option<String>,
    #[deprecated]
    pub no_bid: f64,
    pub no_bid_dollars: Option<String>,
    #[deprecated]
    pub no_ask: f64,
    pub no_ask_dollars: Option<String>,
    #[deprecated]
    pub last_price: f64,
    pub last_price_dollars: Option<String>,
    pub volume: i64,
    pub volume_fp: Option<String>,
    pub volume_24h: i64,
    pub volume_24h_fp: Option<String>,
    pub result: String,
    pub can_close_early: bool,
    pub fractional_trading_enabled: bool,
    pub open_interest: i64,
    pub open_interest_fp: Option<String>,
    #[deprecated]
    pub notional_value: i64,
    pub notional_value_dollars: Option<String>,
    #[deprecated]
    pub previous_yes_bid: i64,
    pub previous_yes_bid_dollars: Option<String>,
    #[deprecated]
    pub previous_yes_ask: i64,
    pub previous_yes_ask_dollars: Option<String>,
    #[deprecated]
    pub previous_price: i64,
    pub previous_price_dollars: Option<String>,
    #[deprecated]
    pub liquidity: i64,
    #[deprecated]
    pub liquidity_dollars: Option<String>,
    pub settlement_value: Option<i64>,
    pub settlement_value_dollars: Option<String>,
    pub settlement_ts: Option<String>,
    pub expiration_value: String,
    pub fee_waiver_expiration_time: Option<String>,
    pub early_close_condition: Option<String>,
    #[deprecated]
    pub tick_size: i64,
    pub strike_type: Option<String>,
    pub floor_strike: Option<f64>,
    pub cap_strike: Option<f64>,
    pub functional_strike: Option<String>,
    pub custom_strike: Option<serde_json::Value>,
    pub rules_primary: String,
    pub rules_secondary: String,
    pub mve_collection_ticker: Option<String>,
    pub mve_selected_legs: Option<Vec<MveSelectedLeg>>,
    pub primary_participant_key: Option<String>,
    pub price_level_structure: Option<String>,
    pub price_ranges: Option<Vec<PriceRange>>,
    pub is_provisional: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PriceRange {
    pub start: String,
    pub end: String,
    pub step: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MveSelectedLeg {
    pub event_ticker: Option<String>,
    pub market_ticker: Option<String>,
    pub side: Option<String>,
    pub yes_settlement_value_dollars: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub event_ticker: String,
    pub series_ticker: String,
    pub sub_title: String,
    pub title: String,
    pub mutually_exclusive: bool,
    pub category: String,
    pub markets: Option<Vec<Market>>,
    pub strike_date: Option<String>,
    pub strike_period: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Series {
    pub additional_prohibitions: Vec<String>,
    pub category: String,
    pub contract_terms_url: String,
    pub contract_url: String,
    pub fee_multiplier: f64,
    pub fee_type: String,
    pub frequency: String,
    pub product_metadata: Option<serde_json::Value>,
    pub settlement_sources: Vec<SettlementSource>,
    pub tags: Vec<String>,
    pub ticker: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SettlementSource {
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Orderbook {
    pub yes: Option<Vec<(u32, i32)>>,
    pub no: Option<Vec<(u32, i32)>>,
    pub yes_dollars: Option<Vec<(String, i32)>>,
    pub no_dollars: Option<Vec<(String, i32)>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Snapshot {
    pub yes_price: u32,
    pub yes_bid: u32,
    pub yes_ask: u32,
    pub no_bid: u32,
    pub no_ask: u32,
    pub volume: u32,
    pub open_interest: u32,
    pub ts: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Trade {
    pub trade_id: String,
    pub taker_side: String,
    pub ticker: String,
    pub count: u32,
    pub yes_price: u32,
    pub no_price: u32,
    pub created_time: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MarketStatus {
    Open,
    Closed,
    Settled,
}