
use serde::{Serialize, Deserialize};

use crate::entities::{fixt11::{header::{Header, HasHeader, MsgType}, Trailer}, version::FixVersion};

/// MsgType = 4
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SequenceReset {
    #[serde(flatten)]
    pub header: Header,
    /// GapFillFlag
    #[serde(rename = "123")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_fill_flag: Option<crate::entities::Boolean>,
    /// NewSeqNo
    #[serde(rename = "36")]
    #[serde(deserialize_with = "crate::entities::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub new_seq_no: u64,
    #[serde(flatten)]
    pub trailer: Trailer,
}

impl SequenceReset {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HasHeader for SequenceReset {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl Default for SequenceReset {
    fn default() -> Self {
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
