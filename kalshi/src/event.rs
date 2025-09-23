use super::Kalshi;
use crate::kalshi_error::*;
use crate::SettlementSource;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves metadata for an event by its ticker.
    ///
    /// # Arguments
    /// * `event_ticker` - The ticker of the event.
    ///
    /// # Returns
    /// - `Ok(EventMetadata)`: Event metadata on success.
    /// - `Err(KalshiError)`: If the request fails or response parsing fails.
    ///
    /// # Example
    /// ```
    /// /dev/null/example.rs#L1-12
    /// # async fn example(k: &kalshi::Kalshi) -> Result<(), kalshi::KalshiError> {
    /// let meta = k.get_event_metadata(&"EVENT-123".to_string()).await?;
    /// println!("Event image: {:?}", meta.image_url);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_event_metadata(
        &self,
        event_ticker: &String,
    ) -> Result<EventMetadata, KalshiError> {
        let path = format!("/events/{}/metadata", event_ticker);
        let url = self.build_url_with_params(&path, Vec::new())?;
        let result: EventMetadata = self
            .client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }

    /// Retrieves aggregated candlestick data across all markets in an event.
    ///
    /// # Arguments
    /// * `event_ticker` - The ticker of the event.
    ///
    /// # Returns
    /// - `Ok(EventCandlesticks)`: Aggregated candlestick data by market.
    /// - `Err(KalshiError)`: If the request fails or response parsing fails.
    ///
    /// # Example
    /// ```
    /// /dev/null/example.rs#L1-12
    /// # async fn example(k: &kalshi::Kalshi) -> Result<(), kalshi::KalshiError> {
    /// let candles = k.get_event_candlesticks(&"EVENT-123".to_string()).await?;
    /// println!("Markets in event: {}", candles.market_tickers.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_event_candlesticks(
        &self,
        event_ticker: &String,
    ) -> Result<EventCandlesticks, KalshiError> {
        let path = format!("/events/{}/candlesticks", event_ticker);
        let url = self.build_url_with_params(&path, Vec::new())?;
        let result: EventCandlesticks = self
            .client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }

    /// Retrieves historical raw and formatted forecast percentiles for an event.
    ///
    /// Data is cached (slightly lagged).
    ///
    /// # Arguments
    /// * `event_ticker` - The ticker of the event.
    ///
    /// # Returns
    /// - `Ok(Vec<ForecastPercentilesSeries>)`: Forecast percentiles history.
    /// - `Err(KalshiError)`: If the request fails or response parsing fails.
    ///
    /// # Example
    /// ```
    /// /dev/null/example.rs#L1-16
    /// # async fn example(k: &kalshi::Kalshi) -> Result<(), kalshi::KalshiError> {
    /// let history = k.get_event_forecast_history(&"EVENT-123".to_string()).await?;
    /// if let Some(first) = history.first() {
    ///   println!("Interval: {} minutes, pts: {}", first.period_interval, first.percentile_points.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_event_forecast_history(
        &self,
        event_ticker: &String,
    ) -> Result<Vec<ForecastPercentilesSeries>, KalshiError> {
        let path = format!("/cached/events/{}/forecast_history", event_ticker);
        let url = self.build_url_with_params(&path, Vec::new())?;
        let result: ForecastHistoryResponse = self
            .client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(result.forecast_history)
    }

    /// Retrieves all markets for an event by ticker.
    ///
    /// This uses GET /events/{event_ticker} with with_nested_markets=false to return
    /// the markets as a top-level field, then extracts and returns them.
    pub async fn get_event_markets(
        &self,
        event_ticker: &String,
    ) -> Result<Vec<crate::Market>, KalshiError> {
        let path = format!("/events/{}", event_ticker);

        let mut params: Vec<(&str, String)> = Vec::new();
        // Ensure markets are returned top-level, not nested inside the event object
        add_param!(params, "with_nested_markets", Some(false));

        let url = self.build_url_with_params(&path, params)?;
        let result: EventWithMarketsResponse = self
            .client
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(result.markets)
    }
}

// PRIVATE RESPONSES

#[derive(Debug, Deserialize, Serialize)]
struct ForecastHistoryResponse {
    pub forecast_history: Vec<ForecastPercentilesSeries>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EventWithMarketsResponse {
    markets: Vec<crate::Market>,
}

// PUBLIC STRUCTS

/// Event metadata, including competition, images, and settlement sources.
#[derive(Debug, Deserialize, Serialize)]
pub struct EventMetadata {
    /// Competition identifier (if applicable).
    pub competition: Option<String>,
    /// Scope of competition (if applicable).
    pub competition_scope: Option<String>,
    /// Image URL for the event.
    pub image_url: Option<String>,
    /// Official settlement sources for the event.
    pub settlement_sources: Option<Vec<SettlementSource>>,
}

/// Aggregated candlestick data across all markets in an event.
#[derive(Debug, Deserialize, Serialize)]
pub struct EventCandlesticks {
    /// Adjusted end timestamp if the requested range exceeds internal limits.
    pub adjusted_end_ts: Option<i64>,
    /// For each market in `market_tickers`, a list of candlesticks for that market.
    pub market_candlesticks: Vec<Vec<MarketCandlestick>>,
    /// List of market tickers in the event, aligned with `market_candlesticks`.
    pub market_tickers: Vec<String>,
}

/// A single candlestick entry for a given market and period.
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketCandlestick {
    /// Inclusive end of the candlestick period (Unix seconds).
    pub end_period_ts: i64,
    /// Open interest at the end of the period.
    pub open_interest: i64,
    /// OHLC and related stats for traded YES prices during the period.
    pub price: PriceDistribution,
    /// Number of contracts bought during the period.
    pub volume: i64,
    /// OHLC data for YES sell offers during the period.
    pub yes_ask: BidAskDistribution,
    /// OHLC data for YES buy offers during the period.
    pub yes_bid: BidAskDistribution,
}

/// OHLC for bid/ask distributions.
#[derive(Debug, Deserialize, Serialize)]
pub struct BidAskDistribution {
    pub open: i64,
    pub high: i64,
    pub low: i64,
    pub close: i64,
}

/// OHLC and additional stats for traded YES prices during the period.
/// Values may be missing if there was no trade.
#[derive(Debug, Deserialize, Serialize)]
pub struct PriceDistribution {
    pub open: Option<i64>,
    pub high: Option<i64>,
    pub low: Option<i64>,
    pub close: Option<i64>,
    pub mean: Option<i64>,
    pub previous: Option<i64>,
}

/// A single forecast history series entry for an event.
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastPercentilesSeries {
    /// Inclusive end of the period window.
    pub end_period_ts: i64,
    /// Event ticker.
    pub event_ticker: String,
    /// Percentile points for this period.
    pub percentile_points: Vec<PercentilePoint>,
    /// Period length in minutes.
    pub period_interval: i64,
}

/// A single percentile point in the forecast distribution.
#[derive(Debug, Deserialize, Serialize)]
pub struct PercentilePoint {
    /// Human-friendly formatted forecast (e.g. "2.4%").
    pub formatted_forecast: String,
    /// Numerical forecast value.
    pub numerical_forecast: Option<serde_json::Value>,
    /// Percentile index (e.g. 5, 50, 95).
    pub percentile: i64,
    /// Raw numerical forecast value.
    pub raw_numerical_forecast: Option<serde_json::Value>,
}
