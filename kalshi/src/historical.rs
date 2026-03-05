use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves the historical cutoff timestamps.
    /// These timestamps indicate how far back historical data is available for different categories.
    ///
    /// Maps to GET /historical/cutoff
    pub async fn get_historical_cutoff(&self) -> Result<HistoricalCutoff, KalshiError> {
        let url = self.build_url("/historical/cutoff")?;
        self.http_get(url).await
    }

    /// Retrieves historical candlesticks for a specific market.
    ///
    /// Maps to GET /historical/markets/{ticker}/candlesticks
    ///
    /// # Arguments
    /// * `ticker` - Market ticker.
    /// * `start_ts` - Start timestamp (Unix seconds).
    /// * `end_ts` - End timestamp (Unix seconds).
    /// * `period_interval` - Length of each candlestick in minutes (1, 60, or 1440).
    pub async fn get_market_candlesticks_historical(
        &self,
        ticker: &str,
        start_ts: i64,
        end_ts: i64,
        period_interval: i32,
    ) -> Result<Vec<MarketCandlestickHistorical>, KalshiError> {
        let path = format!("/historical/markets/{}/candlesticks", ticker);
        let mut params = Vec::new();
        add_param!(params, "start_ts", Some(start_ts));
        add_param!(params, "end_ts", Some(end_ts));
        add_param!(params, "period_interval", Some(period_interval));

        let url = self.build_url_with_params(&path, params)?;
        let resp: GetMarketCandlesticksHistoricalResponse = self.http_get(url).await?;
        Ok(resp.candlesticks)
    }

    /// Retrieves historical fills for the authenticated user.
    ///
    /// Maps to GET /historical/fills
    ///
    /// # Arguments
    /// * `ticker` - Optional market ticker filter.
    /// * `order_id` - Optional order ID filter.
    /// * `min_ts` - Optional filter items after this Unix timestamp.
    /// * `max_ts` - Optional filter items before this Unix timestamp.
    /// * `limit` - Optional number of results per page.
    /// * `cursor` - Optional pagination cursor.
    pub async fn get_fills_historical(
        &self,
        ticker: Option<String>,
        order_id: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Vec<crate::portfolio::Fill>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "ticker", ticker);
        add_param!(params, "order_id", order_id);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let url = self.build_url_with_params("/historical/fills", params)?;
        let resp: HistoricalFillsResponse = self.http_get(url).await?;
        Ok((resp.fills, resp.cursor))
    }

    /// Retrieves historical orders for the authenticated user.
    ///
    /// Maps to GET /historical/orders
    ///
    /// # Arguments
    /// * `ticker` - Optional market ticker filter.
    /// * `order_id` - Optional order ID filter.
    /// * `min_ts` - Optional filter items after this Unix timestamp.
    /// * `max_ts` - Optional filter items before this Unix timestamp.
    /// * `limit` - Optional number of results per page.
    /// * `cursor` - Optional pagination cursor.
    pub async fn get_historical_orders(
        &self,
        ticker: Option<String>,
        order_id: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Vec<crate::portfolio::Order>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "ticker", ticker);
        add_param!(params, "order_id", order_id);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let url = self.build_url_with_params("/historical/orders", params)?;
        let resp: HistoricalOrdersResponse = self.http_get(url).await?;
        Ok((resp.orders, resp.cursor))
    }

    /// Retrieves historical markets.
    ///
    /// Maps to GET /historical/markets
    ///
    /// # Arguments
    /// * `limit` - Optional number of results per page.
    /// * `cursor` - Optional pagination cursor.
    /// * `event_ticker` - Optional event ticker filter.
    /// * `series_ticker` - Optional series ticker filter.
    /// * `max_close_ts` - Optional filter for markets closing before this Unix timestamp.
    pub async fn get_historical_markets(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        event_ticker: Option<String>,
        series_ticker: Option<String>,
        max_close_ts: Option<i64>,
    ) -> Result<(Vec<crate::market::Market>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "event_ticker", event_ticker);
        add_param!(params, "series_ticker", series_ticker);
        add_param!(params, "max_close_ts", max_close_ts);

        let url = self.build_url_with_params("/historical/markets", params)?;
        let resp: HistoricalMarketsResponse = self.http_get(url).await?;
        Ok((resp.markets, resp.cursor))
    }

    /// Retrieves a single historical market by ticker.
    ///
    /// Maps to GET /historical/markets/{ticker}
    pub async fn get_historical_market(&self, ticker: &str) -> Result<crate::market::Market, KalshiError> {
        let path = format!("/historical/markets/{}", ticker);
        let url = self.build_url(&path)?;
        let resp: HistoricalMarketResponse = self.http_get(url).await?;
        Ok(resp.market)
    }
}

// Internal Response Structs

#[derive(Debug, Deserialize)]
struct GetMarketCandlesticksHistoricalResponse {
    pub ticker: String,
    pub candlesticks: Vec<MarketCandlestickHistorical>,
}

#[derive(Debug, Deserialize)]
struct HistoricalFillsResponse {
    pub fills: Vec<crate::portfolio::Fill>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HistoricalOrdersResponse {
    pub orders: Vec<crate::portfolio::Order>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HistoricalMarketsResponse {
    pub markets: Vec<crate::market::Market>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HistoricalMarketResponse {
    pub market: crate::market::Market,
}

// Public Data Structures

/// Historical cutoff timestamps indicating data availability.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HistoricalCutoff {
    pub market_settled_ts: String,
    pub trades_created_ts: String,
    pub orders_updated_ts: String,
}

/// A historical candlestick data point.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketCandlestickHistorical {
    /// Unix timestamp for the inclusive end of the candlestick period.
    pub end_period_ts: i64,
    /// OHLC data for YES buy offers.
    pub yes_bid: BidAskDistributionHistorical,
    /// OHLC data for YES sell offers.
    pub yes_ask: BidAskDistributionHistorical,
    /// OHLC and trade price stats for YES contracts.
    pub price: PriceDistributionHistorical,
    /// String representation of contracts traded during the period.
    pub volume: String,
    /// String representation of open contracts at the end of the period.
    pub open_interest: String,
}

/// OHLC distribution for bid/ask data in historical candlesticks.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BidAskDistributionHistorical {
    /// Price at the start of the period in dollars.
    pub open: String,
    /// Lowest price during the period in dollars.
    pub low: String,
    /// Highest price during the period in dollars.
    pub high: String,
    /// Price at the end of the period in dollars.
    pub close: String,
}

/// OHLC distribution for trade prices in historical candlesticks.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PriceDistributionHistorical {
    /// Price of the first trade in dollars.
    pub open: Option<String>,
    /// Lowest trade price in dollars.
    pub low: Option<String>,
    /// Highest trade price in dollars.
    pub high: Option<String>,
    /// Price of the last trade in dollars.
    pub close: Option<String>,
    /// Volume-weighted average price in dollars.
    pub mean: Option<String>,
    /// Close price from the previous period in dollars.
    pub previous: Option<String>,
}