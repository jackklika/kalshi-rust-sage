use super::KalshiChannel;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "snake_case")]
pub enum KalshiCommand {
    Subscribe {
        id: u32,
        params: KalshiSubscribeCommandParams,
    },
    Unsubscribe {
        id: u32,
        params: KalshiUnsubscribeCommandParams,
    },
    UpdateSubscription {
        id: u32,
        params: KalshiUpdateSubscriptionCommandParams,
    },
    ListSubscriptions {
        id: u32,
    },
    /// Internal signal to close the WebSocket connection.
    End,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct KalshiSubscribeCommandParams {
    pub channels: Vec<KalshiChannel>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_ticker_ack: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_factor: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_key: Option<u32>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct KalshiUnsubscribeCommandParams {
    pub sids: Vec<u32>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct KalshiUpdateSubscriptionCommandParams {
    pub action: KalshiUpdateSubscriptionAction,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sids: Option<[u32; 1]>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_initial_snapshot: Option<bool>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum KalshiUpdateSubscriptionAction {
    #[default]
    AddMarkets,
    DeleteMarkets,
}