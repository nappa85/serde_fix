
use serde::{Serialize, Deserialize};

use fix_common::FixVersion;
use crate::{header::{Header, MsgType}, Trailer};

/// MsgType = 4
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SequenceReset {
    #[serde(flatten)]
    pub header: Header,
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

impl SequenceReset {
    pub fn new() -> Self {
        SequenceReset {
            header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::SequenceReset),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
