
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FixVersion {
    #[serde(rename = "FIX.4.0")]
    FIX40,
    #[serde(rename = "FIX.4.1")]
    FIX41,
    #[serde(rename = "FIX.4.2")]
    FIX42,
    #[serde(rename = "FIX.4.3")]
    FIX43,
    #[serde(rename = "FIX.4.4")]
    FIX44,
    #[serde(rename = "FIXT.1.1")]
    FIXT11,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApplVerID {
    /// FIX27
    #[serde(rename = "0")]
    FIX27,
    /// FIX30
    #[serde(rename = "1")]
    FIX30,
    /// FIX40
    #[serde(rename = "2")]
    FIX40,
    /// FIX41
    #[serde(rename = "3")]
    FIX41,
    /// FIX42
    #[serde(rename = "4")]
    FIX42,
    /// FIX43
    #[serde(rename = "5")]
    FIX43,
    /// FIX44
    #[serde(rename = "6")]
    FIX44,
    /// FIX50
    #[serde(rename = "7")]
    FIX50,
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1,
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2,
    /// FIXLatest
    #[serde(rename = "10")]
    FIXLatest,
}

impl ToString for ApplVerID {
    fn to_string(&self) -> String {
        serde_fix::to_string(self).expect("ApplVerID serialize failed")// should be safe
    }
}
