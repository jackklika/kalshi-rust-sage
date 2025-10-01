use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves a list of series filtered by category, with optional product metadata and tags.
    ///
    /// Maps to GET /series.
    ///
    /// # Arguments
    /// * `category` - Category to filter series by (required by API).
    /// * `include_product_metadata` - If true, includes internal product metadata for each series.
    /// * `tags` - Comma-separated list of tags to filter by (series containing at least one tag will be returned).
    ///
    /// # Returns
    /// - `Ok(Vec<crate::Series>)`: A vector of Series matching the filters.
    /// - `Err(KalshiError)`: If the request fails or response parsing fails.
    ///
    /// # Example
    /// ```
    /// /dev/null/example.rs#L1-13
    /// # async fn example(k: &kalshi::Kalshi) -> Result<(), kalshi::KalshiError> {
    /// let series = k
    ///     .get_series_list(
    ///         "economics".to_string(),
    ///         Some(true),
    ///         Some("employment,inflation".to_string()),
    ///     )
    ///     .await?;
    /// tracing::debug!("Found {} series", series.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_series_list(
        &self,
        category: String,
        include_product_metadata: Option<bool>,
        tags: Option<String>,
    ) -> Result<Vec<crate::Series>, KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3);
        // API requires category
        params.push(("category", category));
        add_param!(params, "include_product_metadata", include_product_metadata);
        add_param!(params, "tags", tags);

        let url = self.build_url_with_params("/series", params)?;
        let resp: SeriesListResponse = self.http_get(url).await?;
        Ok(resp.series)
    }

    /// Retrieves all upcoming series fee changes.
    ///
    /// Maps to GET /series/fee_changes.
    ///
    /// # Returns
    /// - `Ok(Vec<SeriesFeeChange>)`: A vector of fee changes.
    /// - `Err(KalshiError)`: If the request fails or response parsing fails.
    ///
    /// # Example
    /// ```
    /// /dev/null/example.rs#L1-10
    /// # async fn example(k: &kalshi::Kalshi) -> Result<(), kalshi::KalshiError> {
    /// let changes = k.get_series_fee_changes().await?;
    /// if let Some(first) = changes.first() {
    ///     tracing::debug!("Series {} fee_type {}", first.series_ticker, first.fee_type);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_series_fee_changes(&self) -> Result<Vec<SeriesFeeChange>, KalshiError> {
        let url = self.build_url("/series/fee_changes")?;
        let resp: SeriesFeeChangesResponse = self.http_get(url).await?;
        Ok(resp.series_fee_change_arr)
    }

    /// Retrieves OHLC candlestick history for a specific market scoped under a series.
    ///
    /// Maps to GET /series/{series_ticker}/markets/{ticker}/candlesticks.
    ///
    /// The `period_interval` determines the time length of each candlestick and must be one of:
    /// 1 (1 minute), 60 (1 hour), or 1440 (1 day).
    ///
    /// # Arguments
    /// * `series_ticker` - The series that contains the target market.
    /// * `market_ticker` - Market ticker (unique identifier for the market).
    /// * `start_ts` - Start timestamp (Unix seconds). Candles ending on/after this time are included.
    /// * `end_ts` - End timestamp (Unix seconds). Candles ending on/before this time are included.
    /// * `period_interval` - Length of each candlestick in minutes (1, 60, 1440).
    ///
    /// # Returns
    /// - `Ok((String, Vec<crate::MarketCandlestick>))`: Market ticker and list of candlesticks.
    /// - `Err(KalshiError)`: If the request fails or response parsing fails.
    ///
    /// # Example
    /// ```
    /// /dev/null/example.rs#L1-13
    /// # async fn example(k: &kalshi::Kalshi) -> Result<(), kalshi::KalshiError> {
    /// let (ticker, candles) = k
    ///     .get_market_candlesticks(
    ///         &"JOBS-URATE".to_string(),
    ///         &"JOBS-URATE-24NOV".to_string(),
    ///         1_700_000_000,
    ///         1_700_086_400,
    ///         60,
    ///     )
    ///     .await?;
    /// tracing::debug!("{} candles returned for {}", candles.len(), ticker);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_market_candlesticks(
        &self,
        series_ticker: &String,
        market_ticker: &String,
        start_ts: i64,
        end_ts: i64,
        period_interval: i64,
    ) -> Result<(String, Vec<crate::MarketCandlestick>), KalshiError> {
        let path = format!(
            "/series/{}/markets/{}/candlesticks",
            series_ticker, market_ticker
        );

        let mut params: Vec<(&str, String)> = Vec::with_capacity(3);
        add_param!(params, "start_ts", Some(start_ts));
        add_param!(params, "end_ts", Some(end_ts));
        add_param!(params, "period_interval", Some(period_interval));

        let url = self.build_url_with_params(&path, params)?;
        let resp: MarketCandlesticksResponse = self.http_get(url).await?;
        Ok((resp.ticker, resp.candlesticks))
    }
}

// PRIVATE RESPONSES
// -----------------------------------------------

#[derive(Debug, Deserialize, Serialize)]
struct SeriesListResponse {
    series: Vec<crate::Series>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SeriesFeeChangesResponse {
    series_fee_change_arr: Vec<SeriesFeeChange>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MarketCandlesticksResponse {
    ticker: String,
    candlesticks: Vec<crate::MarketCandlestick>,
}

// PUBLIC STRUCTS
// -----------------------------------------------

/// A scheduled fee change for a series.
#[derive(Debug, Deserialize, Serialize)]
pub struct SeriesFeeChange {
    /// ID of this scheduled fee change.
    pub id: String,
    /// Series ticker affected by the fee change.
    pub series_ticker: String,
    /// Fee structure type (e.g., "quadratic", "quadratic_with_maker_fees", "flat").
    pub fee_type: String,
    /// Multiplier applied to the fee calculations.
    pub fee_multiplier: f64,
    /// Scheduled time (RFC3339).
    pub scheduled_ts: String,
}