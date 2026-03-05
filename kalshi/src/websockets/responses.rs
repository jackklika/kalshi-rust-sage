use serde::Deserialize;
use super::KalshiChannel;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum KalshiWebsocketResponse {
    /// Initial snapshot of an orderbook.
    OrderbookSnapshot {
        sid: u32,
        seq: u32,
        msg: KalshiOrderbookSnapshotMessage,
    },
    /// Incremental update to an orderbook.
    OrderbookDelta {
        sid: u32,
        seq: u32,
        msg: KalshiOrderbookDeltaMessage,
    },
    /// Market ticker information.
    Ticker {
        sid: u32,
        msg: KalshiTickerMessage,
    },
    /// Public trade notification.
    Trade {
        sid: u32,
        msg: KalshiTradeMessage,
    },
    /// Private fill notification for the authenticated user.
    Fill {
        sid: u32,
        msg: KalshiFillMessage,
    },
    /// Market lifecycle event (v2).
    MarketLifecycleV2 {
        sid: u32,
        msg: KalshiMarketLifecycleV2Message,
    },
    /// Event creation notification.
    EventLifecycle {
        sid: u32,
        msg: KalshiEventLifecycleMessage,
    },
    /// Multivariate collection lookup notification.
    MultivariateLookup {
        sid: u32,
        msg: KalshiMultivariateLookupMessage,
    },
    /// Real-time position update for the authenticated user.
    MarketPosition {
        sid: u32,
        msg: KalshiMarketPositionMessage,
    },
    /// Order group lifecycle and limit updates.
    OrderGroupUpdates {
        sid: u32,
        seq: u32,
        msg: KalshiOrderGroupUpdatesMessage,
    },
    /// Real-time order update for the authenticated user.
    UserOrder {
        sid: u32,
        msg: KalshiUserOrderMessage,
    },
    /// Notification when an RFQ is created.
    RfqCreated {
        sid: u32,
        msg: KalshiRfqCreatedMessage,
    },
    /// Notification when an RFQ is deleted.
    RfqDeleted {
        sid: u32,
        msg: KalshiRfqDeletedMessage,
    },
    /// Notification when a quote is created on an RFQ.
    QuoteCreated {
        sid: u32,
        msg: KalshiQuoteCreatedMessage,
    },
    /// Notification when a quote is accepted.
    QuoteAccepted {
        sid: u32,
        msg: KalshiQuoteAcceptedMessage,
    },
    /// Notification when a quote is executed and orders are placed.
    QuoteExecuted {
        sid: u32,
        msg: KalshiQuoteExecutedMessage,
    },
    /// Confirmation that a subscription was successful.
    Subscribed {
        id: Option<u32>,
        msg: KalshiSubscribedMessage,
    },
    /// Confirmation that an unsubscription was successful.
    Unsubscribed {
        id: Option<u32>,
        sid: u32,
        seq: u32,
    },
    /// Successful update operation or command response.
    Ok {
        id: Option<u32>,
        sid: Option<u32>,
        seq: Option<u32>,
        msg: Option<KalshiOkPayload>,
    },
    /// Error response for failed operations.
    Error {
        id: Option<u32>,
        msg: KalshiErrorMessage,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiSubscribedMessage {
    pub channel: KalshiChannel,
    pub sid: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum KalshiOkPayload {
    /// For list_subscriptions response.
    Subscriptions(Vec<KalshiSubscribedMessage>),
    /// For update_subscription responses.
    MarketUpdates {
        #[serde(default)]
        market_tickers: Option<Vec<String>>,
        #[serde(default)]
        market_ids: Option<Vec<String>>,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiErrorMessage {
    pub code: u32,
    pub msg: String,
    pub market_id: Option<String>,
    pub market_ticker: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookSnapshotMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub yes: Option<Vec<(u32, u32)>>,
    pub yes_dollars: Option<Vec<(String, u32)>>,
    pub yes_dollars_fp: Option<Vec<(String, String)>>,
    pub no: Option<Vec<(u32, u32)>>,
    pub no_dollars: Option<Vec<(String, u32)>>,
    pub no_dollars_fp: Option<Vec<(String, String)>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookDeltaMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub price: u32,
    pub price_dollars: String,
    pub delta: i32,
    pub delta_fp: String,
    pub side: KalshiSide,
    pub client_order_id: Option<String>,
    pub subaccount: Option<u32>,
    pub ts: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiTickerMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub price: u32,
    pub yes_bid: u32,
    pub yes_ask: u32,
    pub price_dollars: String,
    pub yes_bid_dollars: String,
    pub yes_ask_dollars: String,
    pub volume: u32,
    pub volume_fp: String,
    pub open_interest: u32,
    pub open_interest_fp: String,
    pub dollar_volume: u32,
    pub dollar_open_interest: u32,
    pub ts: i64,
    pub time: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiTradeMessage {
    pub trade_id: String,
    pub market_ticker: String,
    pub yes_price: u32,
    pub yes_price_dollars: String,
    pub no_price: u32,
    pub no_price_dollars: String,
    pub count: u32,
    pub count_fp: String,
    pub taker_side: KalshiSide,
    pub ts: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiFillMessage {
    pub trade_id: String,
    pub order_id: String,
    pub market_ticker: String,
    pub is_taker: bool,
    pub side: KalshiSide,
    pub yes_price: u32,
    pub yes_price_dollars: String,
    pub count: u32,
    pub count_fp: String,
    pub fee_cost: String,
    pub action: KalshiAction,
    pub ts: i64,
    pub client_order_id: Option<String>,
    pub post_position: i32,
    pub post_position_fp: String,
    pub purchased_side: KalshiSide,
    pub subaccount: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketLifecycleV2Message {
    pub event_type: String,
    pub market_ticker: String,
    pub open_ts: Option<i64>,
    pub close_ts: Option<i64>,
    pub result: Option<String>,
    pub determination_ts: Option<i64>,
    pub settlement_value: Option<String>,
    pub settled_ts: Option<i64>,
    pub is_deactivated: Option<bool>,
    pub additional_metadata: Option<KalshiMarketAdditionalMetadata>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketAdditionalMetadata {
    pub name: Option<String>,
    pub title: Option<String>,
    pub yes_sub_title: Option<String>,
    pub no_sub_title: Option<String>,
    pub rules_primary: Option<String>,
    pub rules_secondary: Option<String>,
    pub can_close_early: Option<bool>,
    pub event_ticker: Option<String>,
    pub expected_expiration_ts: Option<i64>,
    pub strike_type: Option<String>,
    pub floor_strike: Option<f64>,
    pub cap_strike: Option<f64>,
    pub custom_strike: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiEventLifecycleMessage {
    pub event_ticker: String,
    pub title: String,
    pub subtitle: String,
    pub collateral_return_type: String,
    pub series_ticker: String,
    pub strike_date: Option<i64>,
    pub strike_period: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMultivariateLookupMessage {
    pub collection_ticker: String,
    pub event_ticker: String,
    pub market_ticker: String,
    pub selected_markets: Vec<KalshiSelectedMarket>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiSelectedMarket {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: KalshiSide,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketPositionMessage {
    pub user_id: String,
    pub market_ticker: String,
    pub position: i32,
    pub position_fp: String,
    pub position_cost: i64,
    pub realized_pnl: i64,
    pub fees_paid: i64,
    pub position_fee_cost: i64,
    pub volume: i32,
    pub volume_fp: String,
    pub subaccount: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderGroupUpdatesMessage {
    pub event_type: String,
    pub order_group_id: String,
    pub contracts_limit_fp: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiUserOrderMessage {
    pub order_id: String,
    pub user_id: String,
    pub ticker: String,
    pub status: String,
    pub side: KalshiSide,
    pub is_yes: bool,
    pub yes_price_dollars: String,
    pub fill_count_fp: String,
    pub remaining_count_fp: String,
    pub initial_count_fp: String,
    pub taker_fill_cost_dollars: String,
    pub maker_fill_cost_dollars: String,
    pub taker_fees_dollars: Option<String>,
    pub maker_fees_dollars: Option<String>,
    pub client_order_id: String,
    pub order_group_id: Option<String>,
    pub self_trade_prevention_type: Option<String>,
    pub created_time: String,
    pub last_update_time: Option<String>,
    pub expiration_time: Option<String>,
    pub subaccount_number: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMveSelectedLeg {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: String,
    pub yes_settlement_value_dollars: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiRfqCreatedMessage {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    pub event_ticker: Option<String>,
    pub contracts: Option<u32>,
    pub contracts_fp: Option<String>,
    pub target_cost: Option<i32>,
    pub target_cost_dollars: Option<String>,
    pub created_ts: String,
    pub mve_collection_ticker: Option<String>,
    pub mve_selected_legs: Option<Vec<KalshiMveSelectedLeg>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiRfqDeletedMessage {
    pub id: String,
    pub creator_id: String,
    pub market_ticker: String,
    pub event_ticker: Option<String>,
    pub contracts: Option<u32>,
    pub contracts_fp: Option<String>,
    pub target_cost: Option<i32>,
    pub target_cost_dollars: Option<String>,
    pub deleted_ts: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiQuoteCreatedMessage {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub market_ticker: String,
    pub event_ticker: Option<String>,
    pub yes_bid: i32,
    pub no_bid: i32,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    pub yes_contracts_offered: Option<u32>,
    pub no_contracts_offered: Option<u32>,
    pub yes_contracts_offered_fp: Option<String>,
    pub no_contracts_offered_fp: Option<String>,
    pub rfq_target_cost: Option<i32>,
    pub rfq_target_cost_dollars: Option<String>,
    pub created_ts: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiQuoteAcceptedMessage {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub market_ticker: String,
    pub event_ticker: Option<String>,
    pub yes_bid: i32,
    pub no_bid: i32,
    pub yes_bid_dollars: String,
    pub no_bid_dollars: String,
    pub accepted_side: Option<String>,
    pub contracts_accepted: Option<u32>,
    pub yes_contracts_offered: Option<u32>,
    pub no_contracts_offered: Option<u32>,
    pub contracts_accepted_fp: Option<String>,
    pub yes_contracts_offered_fp: Option<String>,
    pub no_contracts_offered_fp: Option<String>,
    pub rfq_target_cost: Option<i32>,
    pub rfq_target_cost_dollars: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiQuoteExecutedMessage {
    pub quote_id: String,
    pub rfq_id: String,
    pub quote_creator_id: String,
    pub rfq_creator_id: String,
    pub order_id: String,
    pub client_order_id: String,
    pub market_ticker: String,
    pub executed_ts: String,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KalshiSide {
    Yes,
    No,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KalshiAction {
    Buy,
    Sell,
}