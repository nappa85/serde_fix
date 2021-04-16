
use serde::{Serialize, Deserialize};

use crate::{StandardMessageHeader, StandardMessageTrailer};

/// MsgType = 2
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResendRequest {
    #[serde(flatten)]
    pub standard_message_header: StandardMessageHeader<5, '2', ' '>,
    /// BeginSeqNo
    #[serde(rename = "7")]
    #[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub begin_seq_no: u64,
    /// EndSeqNo
    #[serde(rename = "16")]
    #[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub end_seq_no: u64,
    #[serde(flatten)]
    pub standard_message_trailer: StandardMessageTrailer,
}
