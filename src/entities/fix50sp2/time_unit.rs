
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TimeUnit {
    /// Hour
    #[serde(rename = "H")]
    Hour,
    /// Minute
    #[serde(rename = "Min")]
    Minute,
    /// Second
    #[serde(rename = "S")]
    Second,
    /// Day
    #[serde(rename = "D")]
    Day,
    /// Week
    #[serde(rename = "Wk")]
    Week,
    /// Month
    #[serde(rename = "Mo")]
    Month,
    /// Year
    #[serde(rename = "Yr")]
    Year,
    /// Quarter
    #[serde(rename = "Q")]
    Quarter,
}
