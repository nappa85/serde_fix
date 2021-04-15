
use serde::{Serialize, Deserialize};

use crate::{Header, Trailer};

/// MsgType = 4
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SequenceReset {
    #[serde(flatten)]
    pub header: Header<5, '4', ' '>,
    /// GapFillFlag
    #[serde(rename = "123")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_fill_flag: Option<fix_common::Boolean>,
    /// NewSeqNo
    #[serde(rename = "36")]
    #[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub new_seq_no: u64,
    #[serde(flatten)]
    pub trailer: Trailer,
}
