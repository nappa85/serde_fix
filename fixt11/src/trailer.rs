
use serde::{Serialize, Deserialize};

use fix_common::EncodedText;

/// Standard Message Header
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Trailer {
    /// Required when trailer contains signature. Note: Not to be included within SecureData (91) field
    #[serde(rename = "93")]
    #[serde(alias = "89")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<EncodedText<89>>,
    /// (Always unencrypted, always last field in message)
    #[serde(rename = "10")]
    #[serde(skip_serializing)]
    pub check_sum: String,
}
