use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves a single multivariate event collection by its ticker.
    ///
    /// Maps to GET /multivariate_event_collections/{collection_ticker}
    pub async fn get_multivariate_event_collection(
        &self,
        collection_ticker: &str,
    ) -> Result<MultivariateEventCollection, KalshiError> {
        let path = format!("/multivariate_event_collections/{}", collection_ticker);
        let url = self.build_url(&path)?;
        let resp: GetMultivariateEventCollectionResponse = self.http_get(url).await?;
        Ok(resp.multivariate_contract)
    }

    /// Retrieves multivariate event collections with optional filters.
    ///
    /// Maps to GET /multivariate_event_collections
    pub async fn get_multivariate_event_collections(
        &self,
        status: Option<String>,
        associated_event_ticker: Option<String>,
        series_ticker: Option<String>,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<(Vec<MultivariateEventCollection>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "status", status);
        add_param!(params, "associated_event_ticker", associated_event_ticker);
        add_param!(params, "series_ticker", series_ticker);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let url = self.build_url_with_params("/multivariate_event_collections", params)?;
        let resp: GetMultivariateEventCollectionsResponse = self.http_get(url).await?;
        Ok((resp.multivariate_contracts, resp.cursor))
    }

    /// Looks up tickers for a market in a multivariate event collection.
    ///
    /// Maps to PUT /multivariate_event_collections/{collection_ticker}/lookup
    pub async fn lookup_multivariate_market(
        &self,
        collection_ticker: &str,
        selected_markets: Vec<crate::market::MveSelectedLeg>,
    ) -> Result<MultivariateMarketLookupResponse, KalshiError> {
        let path = format!("/multivariate_event_collections/{}/lookup", collection_ticker);
        let url = self.build_url(&path)?;
        let payload = MultivariateMarketLookupRequest { selected_markets };
        self.http_put(url, &payload).await
    }

    /// Creates a market in a multivariate event collection.
    ///
    /// Maps to POST /multivariate_event_collections/{collection_ticker}
    pub async fn create_multivariate_market(
        &self,
        collection_ticker: &str,
        selected_markets: Vec<crate::market::MveSelectedLeg>,
        with_market_payload: bool,
    ) -> Result<CreateMultivariateMarketResponse, KalshiError> {
        let path = format!("/multivariate_event_collections/{}", collection_ticker);
        let url = self.build_url(&path)?;
        let payload = CreateMultivariateMarketRequest {
            selected_markets,
            with_market_payload,
        };
        self.http_post(url, &payload).await
    }

    /// Retrieves lookup history for a multivariate event collection.
    ///
    /// Maps to GET /multivariate_event_collections/{collection_ticker}/lookup
    pub async fn get_multivariate_lookup_history(
        &self,
        collection_ticker: &str,
        lookback_seconds: i32,
    ) -> Result<Vec<MultivariateLookupPoint>, KalshiError> {
        let path = format!("/multivariate_event_collections/{}/lookup", collection_ticker);
        let mut params = Vec::new();
        add_param!(params, "lookback_seconds", Some(lookback_seconds));

        let url = self.build_url_with_params(&path, params)?;
        let resp: GetMultivariateEventCollectionLookupHistoryResponse = self.http_get(url).await?;
        Ok(resp.lookup_points)
    }
}

// Internal Response Structs

#[derive(Debug, Deserialize)]
struct GetMultivariateEventCollectionResponse {
    pub multivariate_contract: MultivariateEventCollection,
}

#[derive(Debug, Deserialize)]
struct GetMultivariateEventCollectionsResponse {
    pub multivariate_contracts: Vec<MultivariateEventCollection>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GetMultivariateEventCollectionLookupHistoryResponse {
    pub lookup_points: Vec<MultivariateLookupPoint>,
}

// Public Payloads and Data Structures

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MultivariateEventCollection {
    pub collection_ticker: String,
    pub series_ticker: String,
    pub title: String,
    pub description: String,
    pub open_date: String,
    pub close_date: String,
    pub associated_events: Vec<MultivariateAssociatedEvent>,
    pub associated_event_tickers: Vec<String>,
    pub is_ordered: bool,
    pub is_single_market_per_event: bool,
    pub is_all_yes: bool,
    pub size_min: i32,
    pub size_max: i32,
    pub functional_description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MultivariateAssociatedEvent {
    pub ticker: String,
    pub is_yes_only: bool,
    pub size_max: Option<i32>,
    pub size_min: Option<i32>,
    pub active_quoters: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct MultivariateMarketLookupRequest {
    pub selected_markets: Vec<crate::market::MveSelectedLeg>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MultivariateMarketLookupResponse {
    pub event_ticker: String,
    pub market_ticker: String,
}

#[derive(Debug, Serialize)]
pub struct CreateMultivariateMarketRequest {
    pub selected_markets: Vec<crate::market::MveSelectedLeg>,
    pub with_market_payload: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateMultivariateMarketResponse {
    pub event_ticker: String,
    pub market_ticker: String,
    pub market: Option<crate::market::Market>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MultivariateLookupPoint {
    pub event_ticker: String,
    pub market_ticker: String,
    pub selected_markets: Vec<crate::market::MveSelectedLeg>,
    pub last_queried_ts: String,
}