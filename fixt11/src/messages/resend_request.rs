
use serde::{Serialize, Deserialize};

use fix_common::FixVersion;
use crate::{header::{Header, MsgType}, Trailer};

/// MsgType = 2
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResendRequest {
    #[serde(flatten)]
    pub header: Header,
    /// BeginSeqNo
    #[serde(rename = "7")]
    #[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub begin_seq_no: u64,
    /// EndSeqNo
    #[serde(rename = "16")]
    #[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub end_seq_no: u64,
    #[serde(flatten)]
    pub trailer: Trailer,
}

impl ResendRequest {
    pub fn new() -> Self {
        ResendRequest {
            header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::ResendRequest),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
