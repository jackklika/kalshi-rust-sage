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
