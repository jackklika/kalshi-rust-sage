use serde::Deserialize;

use super::KalshiChannel;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum KalshiWebsocketResponse {
    OrderbookSnapshot {
        sid: u32,
        seq: u32,
        msg: KalshiOrderbookSnapshotMessage,
    },
    OrderbookDelta {
        sid: u32,
        seq: u32,
        msg: KalshiOrderbookDeltaMessage,
    },
    Ticker {
        sid: u32,
        msg: KalshiTickerMessage,
    },
    Trade {
        sid: u32,
        msg: KalshiTradeMessage,
    },
    Fill {
        sid: u32,
        msg: KalshiFillMessage,
    },
    MarketLifecycle {
        sid: u32,
        msg: KalshiMarketLifecycleMessage,
    },
    Subscribed {
        id: Option<u32>,
        msg: KalshiSubscribeMessage,
    },
    Unsubscribed {
        sid: u32,
    },
    MarketLifecycleV2 {
        sid: u32,
        msg: KalshiMarketLifecycleV2Message,
    },
    EventLifecycle {
        sid: u32,
        msg: KalshiEventLifecycleMessage,
    },
    MultivariateLookup {
        sid: u32,
        msg: KalshiMultivariateLookupMessage,
    },
    MarketPosition {
        sid: u32,
        msg: KalshiMarketPositionMessage,
    },
    Error {
        id: Option<u32>,
        msg: KalshiOrderbookErrorMessage,
    },
    Ok {
        id: Option<u32>,
        sid: Option<u32>,
        seq: Option<u32>,
        market_tickers: Option<Vec<String>>,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiSubscribeMessage {
    pub channel: KalshiChannel,
    pub sid: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookErrorMessage {
    pub code: u32,
    pub msg: String,
    pub market_id: Option<String>,
    pub market_ticker: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookSnapshotMessage {
    pub market_ticker: String,
    pub yes: Option<Vec<(u32, i32)>>,
    pub no: Option<Vec<(u32, i32)>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookDeltaMessage {
    pub market_ticker: String,
    pub price: u32,
    pub delta: i32,
    pub side: KalshiSide,
    pub client_order_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiTickerMessage {
    pub market_ticker: String,
    pub price: u32,
    pub yes_bid: u32,
    pub yes_ask: u32,
    pub volume: u32,
    pub open_interest: u32,
    pub dollar_volume: u32,
    pub dollar_open_interest: u32,
    pub ts: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiTradeMessage {
    pub market_ticker: String,
    pub yes_price: u32,
    pub no_price: u32,
    pub count: u32,
    pub taker_side: KalshiSide,
    pub ts: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiFillMessage {
    pub trade_id: String,
    pub order_id: String,
    pub market_ticker: String,
    pub is_taker: bool,
    pub side: KalshiSide,
    pub yes_price: u32,
    pub no_price: u32,
    pub count: u32,
    pub action: KalshiAction,
    pub ts: u64,
    pub client_order_id: Option<String>,
    pub post_position: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketLifecycleMessage {
    pub market_ticker: String,
    pub close_ts: u32,
    pub determination_ts: Option<u32>,
    pub settled_ts: Option<u32>,
    pub result: Option<String>,
    pub is_deactivated: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketLifecycleV2Message {
    pub event_type: String,
    pub market_ticker: String,
    pub open_ts: Option<u64>,
    pub close_ts: Option<u64>,
    pub result: Option<String>,
    pub determination_ts: Option<u64>,
    pub settled_ts: Option<u64>,
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
    pub expected_expiration_ts: Option<u64>,
    pub strike_type: Option<String>,
    pub floor_strike: Option<String>,
    pub cap_strike: Option<bool>,
    pub custom_strike: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiEventLifecycleMessage {
    pub event_ticker: String,
    pub title: String,
    pub sub_title: String,
    pub collateral_return_type: String,
    pub series_ticker: String,
    pub strike_date: Option<u64>,
    pub strike_period: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiSelectedMarket {
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: KalshiSide,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMultivariateLookupMessage {
    pub collection_ticker: String,
    pub event_ticker: String,
    pub market_ticker: String,
    pub selected_markets: Vec<KalshiSelectedMarket>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketPositionMessage {
    pub user_id: String,
    pub market_ticker: String,
    pub position: i32,
    pub position_cost: i64,
    pub realized_pnl: i64,
    pub fees_paid: i64,
    pub volume: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KalshiSide {
    Yes,
    No,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KalshiAction {
    Buy,
    Sell,
}
