use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves the public communications ID for the authenticated user.
    ///
    /// Maps to GET /communications/id
    pub async fn get_communications_id(&self) -> Result<String, KalshiError> {
        let url = self.build_url("/communications/id")?;
        let resp: GetCommunicationsIDResponse = self.http_get(url).await?;
        Ok(resp.communications_id)
    }

    /// Retrieves a list of RFQs with optional filters.
    ///
    /// Maps to GET /communications/rfqs
    pub async fn get_rfqs(
        &self,
        limit: Option<i32>,
        cursor: Option<String>,
        status: Option<String>,
        creator_user_id: Option<String>,
    ) -> Result<(Vec<RFQ>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "status", status);
        add_param!(params, "creator_user_id", creator_user_id);

        let url = self.build_url_with_params("/communications/rfqs", params)?;
        let resp: GetRFQsResponse = self.http_get(url).await?;
        Ok((resp.rfqs, resp.cursor))
    }

    /// Creates a new RFQ.
    ///
    /// Maps to POST /communications/rfqs
    pub async fn create_rfq(&self, payload: CreateRFQRequest) -> Result<String, KalshiError> {
        let url = self.build_url("/communications/rfqs")?;
        let resp: CreateRFQResponse = self.http_post(url, &payload).await?;
        Ok(resp.id)
    }

    /// Retrieves a single RFQ by its ID.
    ///
    /// Maps to GET /communications/rfqs/{rfq_id}
    pub async fn get_rfq(&self, rfq_id: &str) -> Result<RFQ, KalshiError> {
        let path = format!("/communications/rfqs/{}", rfq_id);
        let url = self.build_url(&path)?;
        let resp: GetRFQResponse = self.http_get(url).await?;
        Ok(resp.rfq)
    }

    /// Deletes an existing RFQ.
    ///
    /// Maps to DELETE /communications/rfqs/{rfq_id}
    pub async fn delete_rfq(&self, rfq_id: &str) -> Result<(), KalshiError> {
        let path = format!("/communications/rfqs/{}", rfq_id);
        let url = self.build_url(&path)?;
        self.http_delete(url).await
    }

    /// Retrieves a list of quotes with optional filters.
    ///
    /// Maps to GET /communications/quotes
    pub async fn get_quotes(
        &self,
        limit: Option<i32>,
        cursor: Option<String>,
        status: Option<String>,
        rfq_id: Option<String>,
        quote_creator_user_id: Option<String>,
        rfq_creator_user_id: Option<String>,
    ) -> Result<(Vec<Quote>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "status", status);
        add_param!(params, "rfq_id", rfq_id);
        add_param!(params, "quote_creator_user_id", quote_creator_user_id);
        add_param!(params, "rfq_creator_user_id", rfq_creator_user_id);

        let url = self.build_url_with_params("/communications/quotes", params)?;
        let resp: GetQuotesResponse = self.http_get(url).await?;
        Ok((resp.quotes, resp.cursor))
    }

    /// Creates a new quote in response to an RFQ.
    ///
    /// Maps to POST /communications/quotes
    pub async fn create_quote(&self, payload: CreateQuoteRequest) -> Result<String, KalshiError> {
        let url = self.build_url("/communications/quotes")?;
        let resp: CreateQuoteResponse = self.http_post(url, &payload).await?;
        Ok(resp.id)
    }

    /// Retrieves a single quote by its ID.
    ///
    /// Maps to GET /communications/quotes/{quote_id}
    pub async fn get_quote(&self, quote_id: &str) -> Result<Quote, KalshiError> {
        let path = format!("/communications/quotes/{}", quote_id);
        let url = self.build_url(&path)?;
        let resp: GetQuoteResponse = self.http_get(url).await?;
        Ok(resp.quote)
    }

    /// Deletes an existing quote.
    ///
    /// Maps to DELETE /communications/quotes/{quote_id}
    pub async fn delete_quote(&self, quote_id: &str) -> Result<(), KalshiError> {
        let path = format!("/communications/quotes/{}", quote_id);
        let url = self.build_url(&path)?;
        self.http_delete(url).await
    }

    /// Accepts a quote.
    ///
    /// Maps to PUT /communications/quotes/{quote_id}/accept
    pub async fn accept_quote(&self, quote_id: &str, payload: AcceptQuoteRequest) -> Result<(), KalshiError> {
        let path = format!("/communications/quotes/{}/accept", quote_id);
        let url = self.build_url(&path)?;
        self.http_put(url, &payload).await
    }
}

// Internal Response Structs

#[derive(Debug, Deserialize)]
struct GetCommunicationsIDResponse {
    pub communications_id: String,
}

#[derive(Debug, Deserialize)]
struct GetRFQsResponse {
    pub rfqs: Vec<RFQ>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateRFQResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct GetRFQResponse {
    pub rfq: RFQ,
}

#[derive(Debug, Deserialize)]
struct GetQuotesResponse {
    pub quotes: Vec<Quote>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateQuoteResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct GetQuoteResponse {
    pub quote: Quote,
}

// Public Data Structures

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RFQ {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    pub contracts: i32,
    pub contracts_fp: String,
    pub target_cost_dollars: String,
    pub status: String,
    pub created_ts: String,
    pub updated_ts: String,
    pub cancelled_ts: Option<String>,
    pub cancellation_reason: Option<String>,
    pub mve_collection_ticker: Option<String>,
    pub mve_selected_legs: Option<Vec<crate::market::MveSelectedLeg>>,
    pub rest_remainder: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateRFQRequest {
    pub market_ticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<String>,
    pub rest_remainder: bool,
    pub replace_existing: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Quote {
    pub id: String,
    pub rfq_id: String,
    pub creator_id: String,
    pub rfq_creator_id: Option<String>,
    pub market_ticker: String,
    pub contracts: i32,
    pub contracts_fp: String,
    pub yes_bid: i32,
    pub no_bid: i32,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    pub created_ts: String,
    pub updated_ts: String,
    pub status: String,
    pub accepted_side: Option<String>,
    pub accepted_ts: Option<String>,
    pub confirmed_ts: Option<String>,
    pub executed_ts: Option<String>,
    pub cancelled_ts: Option<String>,
    pub cancellation_reason: Option<String>,
    pub rfq_target_cost_dollars: String,
    pub rest_remainder: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateQuoteRequest {
    pub rfq_id: String,
    pub yes_bid: String,
    pub no_bid: String,
    pub rest_remainder: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AcceptQuoteRequest {
    pub accepted_side: String,
}