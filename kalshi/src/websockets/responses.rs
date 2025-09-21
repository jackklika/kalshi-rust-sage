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
    code: u32,
    msg: String,
    market_id: Option<String>,
    market_ticker: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookSnapshotMessage {
    market_ticker: String,
    yes: Option<Vec<(u32, i32)>>,
    no: Option<Vec<(u32, i32)>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiOrderbookDeltaMessage {
    market_ticker: String,
    price: u32,
    delta: i32,
    side: KalshiSide,
    client_order_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiTickerMessage {
    market_ticker: String,
    price: u32,
    yes_bid: u32,
    yes_ask: u32,
    volume: u32,
    open_interest: u32,
    dollar_volume: u32,
    dollar_open_interest: u32,
    ts: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiTradeMessage {
    market_ticker: String,
    yes_price: u32,
    no_price: u32,
    count: u32,
    taker_side: KalshiSide,
    ts: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiFillMessage {
    trade_id: String,
    order_id: String,
    market_ticker: String,
    is_taker: bool,
    side: KalshiSide,
    yes_price: u32,
    no_price: u32,
    count: u32,
    action: KalshiAction,
    ts: u64,
    client_order_id: Option<String>,
    post_position: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketLifecycleMessage {
    market_ticker: String,
    close_ts: u32,
    determination_ts: Option<u32>,
    settled_ts: Option<u32>,
    result: Option<String>,
    is_deactivated: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketLifecycleV2Message {
    event_type: String,
    market_ticker: String,
    open_ts: Option<u64>,
    close_ts: Option<u64>,
    result: Option<String>,
    determination_ts: Option<u64>,
    settled_ts: Option<u64>,
    is_deactivated: Option<bool>,
    additional_metadata: Option<KalshiMarketAdditionalMetadata>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketAdditionalMetadata {
    name: Option<String>,
    title: Option<String>,
    yes_sub_title: Option<String>,
    no_sub_title: Option<String>,
    rules_primary: Option<String>,
    rules_secondary: Option<String>,
    can_close_early: Option<bool>,
    expected_expiration_ts: Option<u64>,
    strike_type: Option<String>,
    floor_strike: Option<String>,
    cap_strike: Option<bool>,
    custom_strike: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiEventLifecycleMessage {
    event_ticker: String,
    title: String,
    sub_title: String,
    collateral_return_type: String,
    series_ticker: String,
    strike_date: Option<u64>,
    strike_period: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiSelectedMarket {
    event_ticker: String,
    market_ticker: String,
    side: KalshiSide,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMultivariateLookupMessage {
    collection_ticker: String,
    event_ticker: String,
    market_ticker: String,
    selected_markets: Vec<KalshiSelectedMarket>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KalshiMarketPositionMessage {
    user_id: String,
    market_ticker: String,
    position: i32,
    position_cost: i64,
    realized_pnl: i64,
    fees_paid: i64,
    volume: i32,
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
