use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};
use std::fmt;

impl Kalshi {
    /// Retrieves the balance and portfolio value for the authenticated user.
    pub async fn get_balance(&self) -> Result<BalanceResponse, KalshiError> {
        let url = self.build_url("/portfolio/balance")?;
        self.http_get(url).await
    }

    /// Retrieves multiple orders for the authenticated user with optional filters.
    pub async fn get_multiple_orders(
        &self,
        ticker: Option<String>,
        event_ticker: Option<String>,
        status: Option<String>,
        limit: Option<i64>,
        cursor: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
    ) -> Result<(Vec<Order>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "ticker", ticker);
        add_param!(params, "event_ticker", event_ticker);
        add_param!(params, "status", status);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);

        let url = self.build_url_with_params("/portfolio/orders", params)?;
        let resp: MultipleOrderResponse = self.http_get(url).await?;
        Ok((resp.orders, resp.cursor))
    }

    /// Retrieves a single order by its ID.
    pub async fn get_single_order(&self, order_id: &str) -> Result<Order, KalshiError> {
        let path = format!("/portfolio/orders/{}", order_id);
        let url = self.build_url(&path)?;
        let resp: SingleOrderResponse = self.http_get(url).await?;
        Ok(resp.order)
    }

    /// Cancels a specific order.
    pub async fn cancel_order(&self, order_id: &str) -> Result<DeleteOrderResponse, KalshiError> {
        let path = format!("/portfolio/orders/{}", order_id);
        let url = self.build_url(&path)?;
        self.http_delete(url).await
    }

    /// Decreases the size of an existing order.
    pub async fn decrease_order(
        &self,
        order_id: &str,
        reduce_by: Option<i32>,
        reduce_to: Option<i32>,
    ) -> Result<DecreaseOrderResponse, KalshiError> {
        let path = format!("/portfolio/orders/{}/decrease", order_id);
        let url = self.build_url(&path)?;
        let payload = DecreaseOrderPayload { reduce_by, reduce_to };
        self.http_post(url, &payload).await
    }

    /// Retrieves multiple fills for the authenticated user.
    pub async fn get_multiple_fills(
        &self,
        ticker: Option<String>,
        order_id: Option<String>,
        limit: Option<i64>,
        cursor: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
    ) -> Result<(Vec<Fill>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "ticker", ticker);
        add_param!(params, "order_id", order_id);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);

        let url = self.build_url_with_params("/portfolio/fills", params)?;
        let resp: MultipleFillsResponse = self.http_get(url).await?;
        Ok((resp.fills, resp.cursor))
    }

    /// Retrieves portfolio settlements.
    pub async fn get_portfolio_settlements(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Vec<Settlement>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let url = self.build_url_with_params("/portfolio/settlements", params)?;
        let resp: PortfolioSettlementResponse = self.http_get(url).await?;
        Ok((resp.settlements, resp.cursor))
    }

    /// Retrieves user positions across markets and events.
    pub async fn get_user_positions(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        ticker: Option<String>,
        event_ticker: Option<String>,
    ) -> Result<GetPositionsResponse, KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "ticker", ticker);
        add_param!(params, "event_ticker", event_ticker);

        let url = self.build_url_with_params("/portfolio/positions", params)?;
        self.http_get(url).await
    }

    /// Creates a new order.
    pub async fn create_order(&self, payload: CreateOrderPayload) -> Result<Order, KalshiError> {
        let url = self.build_url("/portfolio/orders")?;
        let resp: SingleOrderResponse = self.http_post(url, &payload).await?;
        Ok(resp.order)
    }

    /// Batch cancels multiple orders.
    pub async fn batch_cancel_order(&self, order_ids: Vec<String>) -> Result<Vec<DeleteOrderResponse>, KalshiError> {
        let url = self.build_url("/portfolio/orders/batched")?;
        #[derive(Serialize)]
        struct BatchCancelRequest {
            orders: Vec<BatchCancelItem>,
        }
        #[derive(Serialize)]
        struct BatchCancelItem {
            order_id: String,
        }
        let payload = BatchCancelRequest {
            orders: order_ids.into_iter().map(|id| BatchCancelItem { order_id: id }).collect(),
        };
        let resp: BatchCancelOrdersResponse = self.http_delete_with_body(url, &payload).await?;
        Ok(resp.orders.into_iter().map(|o| DeleteOrderResponse {
            order: o.order,
            reduced_by: o.reduced_by,
        }).collect())
    }

    // Order Group Management

    /// Retrieves all order groups for the user.
    pub async fn get_order_groups(&self) -> Result<Vec<OrderGroup>, KalshiError> {
        let url = self.build_url("/portfolio/order_groups")?;
        let resp: GetOrderGroupsResponse = self.http_get(url).await?;
        Ok(resp.order_groups)
    }

    /// Creates a new order group.
    pub async fn create_order_group(&self, payload: CreateOrderGroupRequest) -> Result<String, KalshiError> {
        let url = self.build_url("/portfolio/order_groups/create")?;
        let resp: CreateOrderGroupResponse = self.http_post(url, &payload).await?;
        Ok(resp.order_group_id)
    }

    /// Retrieves a single order group by its ID.
    pub async fn get_order_group(&self, order_group_id: &str) -> Result<GetOrderGroupResponse, KalshiError> {
        let path = format!("/portfolio/order_groups/{}", order_group_id);
        let url = self.build_url(&path)?;
        self.http_get(url).await
    }

    /// Deletes an order group.
    pub async fn delete_order_group(&self, order_group_id: &str) -> Result<(), KalshiError> {
        let path = format!("/portfolio/order_groups/{}", order_group_id);
        let url = self.build_url(&path)?;
        self.http_delete(url).await
    }

    /// Resets an order group.
    pub async fn reset_order_group(&self, order_group_id: &str) -> Result<(), KalshiError> {
        let path = format!("/portfolio/order_groups/{}/reset", order_group_id);
        let url = self.build_url(&path)?;
        self.http_put(url, &serde_json::json!({})).await
    }

    /// Triggers an order group.
    pub async fn trigger_order_group(&self, order_group_id: &str) -> Result<(), KalshiError> {
        let path = format!("/portfolio/order_groups/{}/trigger", order_group_id);
        let url = self.build_url(&path)?;
        self.http_put(url, &serde_json::json!({})).await
    }

    /// Updates the limit for an order group.
    pub async fn update_order_group_limit(&self, order_group_id: &str, payload: UpdateOrderGroupLimitRequest) -> Result<(), KalshiError> {
        let path = format!("/portfolio/order_groups/{}/limit", order_group_id);
        let url = self.build_url(&path)?;
        self.http_put(url, &payload).await
    }

    // Subaccount Management

    /// Creates a new subaccount.
    pub async fn create_subaccount(&self) -> Result<u32, KalshiError> {
        let url = self.build_url("/portfolio/subaccounts")?;
        let resp: CreateSubaccountResponse = self.http_post(url, &serde_json::json!({})).await?;
        Ok(resp.subaccount_number)
    }

    /// Transfers funds between subaccounts.
    pub async fn transfer_between_subaccounts(&self, payload: ApplySubaccountTransferRequest) -> Result<(), KalshiError> {
        let url = self.build_url("/portfolio/subaccounts/transfer")?;
        self.http_post(url, &payload).await
    }

    /// Retrieves balances for all subaccounts.
    pub async fn get_subaccount_balances(&self) -> Result<Vec<SubaccountBalance>, KalshiError> {
        let url = self.build_url("/portfolio/subaccounts/balances")?;
        let resp: GetSubaccountBalancesResponse = self.http_get(url).await?;
        Ok(resp.subaccount_balances)
    }

    /// Retrieves transfers between subaccounts.
    pub async fn get_subaccount_transfers(&self, limit: Option<i64>, cursor: Option<String>) -> Result<(Vec<SubaccountTransfer>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        let url = self.build_url_with_params("/portfolio/subaccounts/transfers", params)?;
        let resp: GetSubaccountTransfersResponse = self.http_get(url).await?;
        Ok((resp.transfers, resp.cursor))
    }

    /// Updates netting settings for a subaccount.
    pub async fn update_subaccount_netting(&self, payload: UpdateSubaccountNettingRequest) -> Result<(), KalshiError> {
        let url = self.build_url("/portfolio/subaccounts/netting")?;
        self.http_put(url, &payload).await
    }

    /// Retrieves netting settings for all subaccounts.
    pub async fn get_subaccount_netting(&self) -> Result<Vec<SubaccountNettingConfig>, KalshiError> {
        let url = self.build_url("/portfolio/subaccounts/netting")?;
        let resp: GetSubaccountNettingResponse = self.http_get(url).await?;
        Ok(resp.netting_configs)
    }

    /// Retrieves the total value of resting orders in cents.
    pub async fn get_total_resting_order_value(&self) -> Result<i64, KalshiError> {
        let url = self.build_url("/portfolio/summary/total_resting_order_value")?;
        let resp: GetPortfolioRestingOrderTotalValueResponse = self.http_get(url).await?;
        Ok(resp.total_resting_order_value)
    }
}

// Responses and Payloads

#[derive(Debug, Deserialize)]
pub struct BalanceResponse {
    pub balance: i64,
    pub portfolio_value: i64,
    pub updated_ts: i64,
}

#[derive(Debug, Deserialize)]
struct SingleOrderResponse {
    pub order: Order,
}

#[derive(Debug, Deserialize)]
struct MultipleOrderResponse {
    pub orders: Vec<Order>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteOrderResponse {
    pub order: Option<Order>,
    pub reduced_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct DecreaseOrderResponse {
    pub order: Order,
}

#[derive(Debug, Serialize)]
struct DecreaseOrderPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_by: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_to: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct MultipleFillsResponse {
    pub fills: Vec<Fill>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PortfolioSettlementResponse {
    pub cursor: Option<String>,
    pub settlements: Vec<Settlement>,
}

#[derive(Debug, Deserialize)]
pub struct GetPositionsResponse {
    pub cursor: Option<String>,
    pub event_positions: Vec<EventPosition>,
    pub market_positions: Vec<MarketPosition>,
}

#[derive(Debug, Serialize)]
pub struct CreateOrderPayload {
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_fp: Option<String>,
    pub side: Side,
    pub ticker: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_max_cost: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yes_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_price_dollars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yes_price_dollars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct BatchCancelOrdersResponse {
    pub orders: Vec<BatchCancelOrdersIndividualResponse>,
}

#[derive(Debug, Deserialize)]
struct BatchCancelOrdersIndividualResponse {
    pub order_id: String,
    pub order: Option<Order>,
    pub reduced_by: i32,
}

// Core Data Structures

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Order {
    pub order_id: String,
    pub user_id: Option<String>,
    pub client_order_id: String,
    pub ticker: String,
    pub side: Side,
    pub action: Action,
    pub status: OrderStatus,
    pub yes_price: i64,
    pub no_price: i64,
    pub yes_price_dollars: Option<String>,
    pub no_price_dollars: Option<String>,
    pub fill_count: i32,
    pub fill_count_fp: Option<String>,
    pub remaining_count: i32,
    pub remaining_count_fp: Option<String>,
    pub initial_count: i32,
    pub initial_count_fp: Option<String>,
    pub taker_fees: i64,
    pub taker_fees_dollars: Option<String>,
    pub maker_fees: i64,
    pub maker_fees_dollars: Option<String>,
    pub taker_fill_cost: i64,
    pub taker_fill_cost_dollars: Option<String>,
    pub maker_fill_cost: i64,
    pub maker_fill_cost_dollars: Option<String>,
    pub queue_position: Option<i32>,
    pub expiration_time: Option<String>,
    pub created_time: Option<String>,
    pub last_update_time: Option<String>,
    pub r#type: String,
    pub order_group_id: Option<String>,
    pub self_trade_prevention_type: Option<String>,
    pub subaccount_number: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Fill {
    pub fill_id: String,
    #[deprecated]
    pub trade_id: String,
    pub order_id: String,
    pub client_order_id: Option<String>,
    pub ticker: String,
    pub side: Side,
    pub action: Action,
    pub count: i32,
    pub count_fp: Option<String>,
    pub yes_price: i64,
    pub no_price: i64,
    pub yes_price_fixed: Option<String>,
    pub no_price_fixed: Option<String>,
    pub is_taker: bool,
    pub created_time: String,
    pub fee_cost: Option<String>,
    pub subaccount_number: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settlement {
    pub ticker: String,
    pub event_ticker: String,
    pub market_result: String,
    pub yes_count: i64,
    pub yes_count_fp: Option<String>,
    pub yes_total_cost: i64,
    pub no_count: i64,
    pub no_count_fp: Option<String>,
    pub no_total_cost: i64,
    pub revenue: i64,
    pub settled_time: String,
    pub fee_cost: Option<String>,
    pub value: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventPosition {
    pub event_ticker: String,
    pub total_cost: i64,
    pub total_cost_dollars: Option<String>,
    pub total_cost_shares: i64,
    pub total_cost_shares_fp: Option<String>,
    pub event_exposure: i64,
    pub event_exposure_dollars: Option<String>,
    pub realized_pnl: i64,
    pub realized_pnl_dollars: Option<String>,
    pub fees_paid: i64,
    pub fees_paid_dollars: Option<String>,
    pub resting_order_count: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketPosition {
    pub ticker: String,
    pub total_traded: i64,
    pub total_traded_dollars: Option<String>,
    pub position: i32,
    pub position_fp: Option<String>,
    pub market_exposure: i64,
    pub market_exposure_dollars: Option<String>,
    pub realized_pnl: i64,
    pub realized_pnl_dollars: Option<String>,
    pub resting_orders_count: i32,
    pub fees_paid: i64,
    pub fees_paid_dollars: Option<String>,
    pub last_updated_ts: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Yes,
    No,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Buy,
    Sell,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Buy => write!(f, "buy"),
            Action::Sell => write!(f, "sell"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Resting,
    Canceled,
    Executed,
    Pending,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderStatus::Resting => "resting",
            OrderStatus::Canceled => "canceled",
            OrderStatus::Executed => "executed",
            OrderStatus::Pending => "pending",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrderGroup {
    pub id: String,
    pub contracts_limit: i64,
    pub contracts_limit_fp: String,
    pub is_auto_cancel_enabled: bool,
}

#[derive(Debug, Deserialize)]
struct GetOrderGroupsResponse {
    pub order_groups: Vec<OrderGroup>,
}

#[derive(Debug, Deserialize)]
pub struct GetOrderGroupResponse {
    pub is_auto_cancel_enabled: bool,
    pub contracts_limit: i64,
    pub contracts_limit_fp: String,
    pub orders: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateOrderGroupRequest {
    pub contracts_limit: Option<i64>,
    pub contracts_limit_fp: Option<String>,
    pub subaccount: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateOrderGroupLimitRequest {
    pub contracts_limit: Option<i64>,
    pub contracts_limit_fp: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateOrderGroupResponse {
    pub order_group_id: String,
}

#[derive(Debug, Deserialize)]
struct CreateSubaccountResponse {
    pub subaccount_number: u32,
}

#[derive(Debug, Serialize)]
pub struct ApplySubaccountTransferRequest {
    pub client_transfer_id: String,
    pub from_subaccount: u32,
    pub to_subaccount: u32,
    pub amount_cents: i64,
}

#[derive(Debug, Deserialize)]
struct GetSubaccountBalancesResponse {
    pub subaccount_balances: Vec<SubaccountBalance>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubaccountBalance {
    pub subaccount_number: u32,
    pub balance: String,
    pub updated_ts: i64,
}

#[derive(Debug, Deserialize)]
struct GetSubaccountTransfersResponse {
    pub transfers: Vec<SubaccountTransfer>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubaccountTransfer {
    pub transfer_id: String,
    pub from_subaccount: u32,
    pub to_subaccount: u32,
    pub amount_cents: i64,
    pub created_ts: i64,
}

#[derive(Debug, Serialize)]
pub struct UpdateSubaccountNettingRequest {
    pub subaccount_number: u32,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
struct GetSubaccountNettingResponse {
    pub netting_configs: Vec<SubaccountNettingConfig>,
}

#[derive(Debug, Deserialize)]
struct GetPortfolioRestingOrderTotalValueResponse {
    pub total_resting_order_value: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubaccountNettingConfig {
    pub subaccount_number: u32,
    pub enabled: bool,
}