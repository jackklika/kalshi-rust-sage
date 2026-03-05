use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Asynchronously retrieves the current status of the exchange.
    ///
    /// This function makes an HTTP GET request to the Kalshi exchange status endpoint
    /// and returns the current status of the exchange, including whether trading
    /// and the exchange itself are active.
    ///
    /// # Returns
    /// - `Ok(ExchangeStatus)`: ExchangeStatus object on successful retrieval.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// ```
    /// kalshi_instance.get_exchange_status().await.unwrap();
    /// ```
    pub async fn get_exchange_status(&self) -> Result<ExchangeStatus, KalshiError> {
        let exchange_status_url = self.build_url("/exchange/status")?;

        let result: ExchangeStatus = self.http_get(exchange_status_url).await?;

        Ok(result)
    }

    /// Asynchronously retrieves the exchange's trading schedule.
    ///
    /// Sends a GET request to the Kalshi exchange schedule endpoint to obtain
    /// detailed schedule information, including standard trading hours and
    /// maintenance windows.
    ///
    /// # Returns
    /// - `Ok(ExchangeSchedule)`: ExchangeSchedule object on success.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// ```
    /// kalshi_instance.get_exchange_schedule().await.unwrap();
    /// ```
    pub async fn get_exchange_schedule(&self) -> Result<ExchangeSchedule, KalshiError> {
        let exchange_schedule_url = self.build_url("/exchange/schedule")?;

        let result: ExchangeScheduleResponse = self.http_get(exchange_schedule_url).await?;
        Ok(result.schedule)
    }

    /// Retrieves exchange announcements.
    pub async fn get_exchange_announcements(&self) -> Result<Vec<Announcement>, KalshiError> {
        let url = self.build_url("/exchange/announcements")?;
        let resp: GetExchangeAnnouncementsResponse = self.http_get(url).await?;
        Ok(resp.announcements)
    }

    /// Retrieves the timestamp when user data was last updated.
    pub async fn get_user_data_timestamp(&self) -> Result<String, KalshiError> {
        let url = self.build_url("/exchange/user_data_timestamp")?;
        let resp: GetUserDataTimestampResponse = self.http_get(url).await?;
        Ok(resp.as_of_time)
    }

    /// Retrieves a single milestone by its ID.
    pub async fn get_milestone(&self, milestone_id: &str) -> Result<Milestone, KalshiError> {
        let path = format!("/milestones/{}", milestone_id);
        let url = self.build_url(&path)?;
        let resp: GetMilestoneResponse = self.http_get(url).await?;
        Ok(resp.milestone)
    }

    /// Retrieves milestones with optional filters.
    pub async fn get_milestones(
        &self,
        limit: Option<i32>,
        cursor: Option<String>,
        category: Option<String>,
        type_: Option<String>,
    ) -> Result<(Vec<Milestone>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "category", category);
        add_param!(params, "type", type_);

        let url = self.build_url_with_params("/milestones", params)?;
        let resp: GetMilestonesResponse = self.http_get(url).await?;
        Ok((resp.milestones, resp.cursor))
    }

    /// Retrieves live data for a specific milestone.
    pub async fn get_live_data(&self, type_: &str, milestone_id: &str) -> Result<LiveData, KalshiError> {
        let path = format!("/live_data/{}/milestone/{}", type_, milestone_id);
        let url = self.build_url(&path)?;
        let resp: GetLiveDataResponse = self.http_get(url).await?;
        Ok(resp.live_data)
    }

    /// Retrieves live data for multiple milestones.
    pub async fn get_live_datas(&self, milestone_ids: Vec<String>) -> Result<Vec<LiveData>, KalshiError> {
        let mut params = Vec::new();
        for id in milestone_ids {
            params.push(("milestone_ids", id));
        }
        let url = self.build_url_with_params("/live_data/batch", params)?;
        let resp: GetLiveDatasResponse = self.http_get(url).await?;
        Ok(resp.live_datas)
    }

    /// Retrieves incentive programs.
    pub async fn get_incentive_programs(
        &self,
        status: Option<String>,
        type_: Option<String>,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<(Vec<IncentiveProgram>, Option<String>), KalshiError> {
        let mut params = Vec::new();
        add_param!(params, "status", status);
        add_param!(params, "type", type_);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let url = self.build_url_with_params("/incentive_programs", params)?;
        let resp: GetIncentiveProgramsResponse = self.http_get(url).await?;
        Ok((resp.incentive_programs, resp.next_cursor))
    }
}

/// Represents the standard trading hours and maintenance windows of the exchange.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeSchedule {
    pub maintenance_windows: Vec<MaintenanceWindow>,
    pub standard_hours: Vec<WeeklySchedule>,
}

/// Internal struct used for deserializing the response from the exchange schedule endpoint.
#[derive(Debug, Deserialize, Serialize)]
struct ExchangeScheduleResponse {
    schedule: ExchangeSchedule,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetExchangeAnnouncementsResponse {
    pub announcements: Vec<Announcement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Announcement {
    pub r#type: String,
    pub message: String,
    pub delivery_time: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetUserDataTimestampResponse {
    pub as_of_time: String,
}

#[derive(Debug, Deserialize)]
struct GetMilestoneResponse {
    pub milestone: Milestone,
}

#[derive(Debug, Deserialize)]
struct GetMilestonesResponse {
    pub milestones: Vec<Milestone>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GetLiveDataResponse {
    pub live_data: LiveData,
}

#[derive(Debug, Deserialize)]
struct GetLiveDatasResponse {
    pub live_datas: Vec<LiveData>,
}

#[derive(Debug, Deserialize)]
struct GetIncentiveProgramsResponse {
    pub incentive_programs: Vec<IncentiveProgram>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Milestone {
    pub id: String,
    pub category: String,
    pub r#type: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub related_event_tickers: Vec<String>,
    pub title: String,
    pub notification_message: String,
    pub source_id: Option<String>,
    pub source_ids: serde_json::Value,
    pub details: serde_json::Value,
    pub primary_event_tickers: Vec<String>,
    pub last_updated_ts: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LiveData {
    pub r#type: String,
    pub details: serde_json::Value,
    pub milestone_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IncentiveProgram {
    pub id: String,
    pub market_id: String,
    pub market_ticker: String,
    pub incentive_type: String,
    pub start_date: String,
    pub end_date: String,
    pub period_reward: i64,
    pub paid_out: bool,
    pub discount_factor_bps: Option<i32>,
    pub target_size: Option<i32>,
    pub target_size_fp: Option<String>,
}

/// Represents the status of the exchange, including trading and exchange activity.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeStatus {
    pub trading_active: bool,
    pub exchange_active: bool,
    pub exchange_estimated_resume_time: Option<String>,
}

/// A maintenance window during which the exchange may be unavailable.
#[derive(Debug, Deserialize, Serialize)]
pub struct MaintenanceWindow {
    pub start_datetime: String,
    pub end_datetime: String,
}

/// A weekly schedule with trading sessions for each day.
#[derive(Debug, Deserialize, Serialize)]
pub struct WeeklySchedule {
    pub start_time: String,
    pub end_time: String,
    pub monday: Vec<DailySchedule>,
    pub tuesday: Vec<DailySchedule>,
    pub wednesday: Vec<DailySchedule>,
    pub thursday: Vec<DailySchedule>,
    pub friday: Vec<DailySchedule>,
    pub saturday: Vec<DailySchedule>,
    pub sunday: Vec<DailySchedule>,
}

/// Represents the opening and closing times of the exchange for a single day.
#[derive(Debug, Deserialize, Serialize)]
pub struct DailySchedule {
    pub open_time: String,
    pub close_time: String,
}
