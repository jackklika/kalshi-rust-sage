use super::Kalshi;
use crate::kalshi_error::*;
use crate::SettlementSource;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves metadata for an event by its ticker.
    ///
    /// Maps to GET /events/{event_ticker}/metadata
    pub async fn get_event_metadata(
        &self,
        event_ticker: &str,
    ) -> Result<EventMetadata, KalshiError> {
        let path = format!("/events/{}/metadata", event_ticker);
        let url = self.build_url(&path)?;
        let result: EventMetadata = self.http_get(url).await?;
        Ok(result)
    }

    /// Retrieves aggregated candlestick data across all markets in an event.
    ///
    /// Maps to GET /events/{event_ticker}/candlesticks
    pub async fn get_event_candlesticks(
        &self,
        event_ticker: &str,
    ) -> Result<EventCandlesticks, KalshiError> {
        let path = format!("/events/{}/candlesticks", event_ticker);
        let url = self.build_url(&path)?;
        let result: EventCandlesticks = self.http_get(url).await?;
        Ok(result)
    }

    /// Retrieves historical raw and formatted forecast percentiles for an event.
    ///
    /// Maps to GET /series/{series_ticker}/events/{ticker}/forecast_percentile_history
    pub async fn get_event_forecast_history(
        &self,
        series_ticker: &str,
        event_ticker: &str,
        percentiles: Vec<i32>,
        start_ts: Option<i64>,
        end_ts: Option<i64>,
        period_interval: Option<i32>,
    ) -> Result<Vec<ForecastPercentilesSeries>, KalshiError> {
        let path = format!(
            "/series/{}/events/{}/forecast_percentile_history",
            series_ticker, event_ticker
        );
        let mut params = Vec::new();
        
        // Percentiles are sent as multiple query params with the same name 'percentiles'
        for p in percentiles {
            params.push(("percentiles", p.to_string()));
        }
        add_param!(params, "start_ts", start_ts);
        add_param!(params, "end_ts", end_ts);
        add_param!(params, "period_interval", period_interval);

        let url = self.build_url_with_params(&path, params)?;
        let result: ForecastHistoryResponse = self.http_get(url).await?;
        Ok(result.forecast_history)
    }

    /// Retrieves all markets for an event by ticker.
    ///
    /// Maps to GET /events/{event_ticker}
    pub async fn get_event_markets(
        &self,
        event_ticker: &str,
        with_nested_markets: bool,
    ) -> Result<Vec<crate::Market>, KalshiError> {
        let path = format!("/events/{}", event_ticker);
        let mut params = Vec::new();
        add_param!(params, "with_nested_markets", Some(with_nested_markets));

        let url = self.build_url_with_params(&path, params)?;
        let result: EventWithMarketsResponse = self.http_get(url).await?;
        Ok(result.markets)
    }
}

// PRIVATE RESPONSES

#[derive(Debug, Deserialize, Serialize)]
struct ForecastHistoryResponse {
    #[serde(rename = "forecast_history")]
    pub forecast_history: Vec<ForecastPercentilesSeries>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EventWithMarketsResponse {
    pub markets: Vec<crate::Market>,
}

// PUBLIC STRUCTS

/// Event metadata, including competition, images, and settlement sources.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventMetadata {
    pub image_url: String,
    pub featured_image_url: Option<String>,
    pub market_details: Option<Vec<MarketMetadata>>,
    pub settlement_sources: Vec<SettlementSource>,
    pub competition: Option<String>,
    pub competition_scope: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketMetadata {
    pub market_ticker: String,
    pub image_url: String,
    pub color_code: String,
}

/// Aggregated candlestick data across all markets in an event.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventCandlesticks {
    pub market_tickers: Vec<String>,
    pub market_candlesticks: Vec<Vec<MarketCandlestick>>,
    pub adjusted_end_ts: Option<i64>,
}

/// A single candlestick entry for a given market and period.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketCandlestick {
    pub end_period_ts: i64,
    pub yes_bid: BidAskDistribution,
    pub yes_ask: BidAskDistribution,
    pub price: PriceDistribution,
    pub volume: i64,
    pub volume_fp: String,
    pub open_interest: i64,
    pub open_interest_fp: String,
}

/// OHLC for bid/ask distributions.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BidAskDistribution {
    pub open: i64,
    pub open_dollars: String,
    pub low: i64,
    pub low_dollars: String,
    pub high: i64,
    pub high_dollars: String,
    pub close: i64,
    pub close_dollars: String,
}

/// OHLC and additional stats for traded YES prices during the period.
/// Values may be missing if there was no trade.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PriceDistribution {
    pub open: Option<i64>,
    pub open_dollars: Option<String>,
    pub low: Option<i64>,
    pub low_dollars: Option<String>,
    pub high: Option<i64>,
    pub high_dollars: Option<String>,
    pub close: Option<i64>,
    pub close_dollars: Option<String>,
    pub mean: Option<i64>,
    pub mean_dollars: Option<String>,
    pub previous: Option<i64>,
    pub previous_dollars: Option<String>,
}

/// A single forecast history series entry for an event.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ForecastPercentilesSeries {
    pub event_ticker: String,
    pub end_period_ts: i64,
    pub period_interval: i32,
    pub percentile_points: Vec<PercentilePoint>,
}

/// A single percentile point in the forecast distribution.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PercentilePoint {
    pub percentile: i32,
    pub raw_numerical_forecast: f64,
    pub numerical_forecast: f64,
    pub formatted_forecast: String,
}