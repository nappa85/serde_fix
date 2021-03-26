
use serde::{Serialize, Deserialize};

use crate::entities::fixt11::header::{Header, HasHeader};

/// MsgType = 4
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SequenceReset {
    #[serde(flatten)]
    pub header: crate::entities::fixt11::Header,
    /// GapFillFlag
    #[serde(rename = "123")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_fill_flag: Option<crate::entities::Boolean>,
    /// NewSeqNo
    #[serde(rename = "36")]
    pub new_seq_no: u64,
    #[serde(flatten)]
    pub trailer: crate::entities::fixt11::Trailer,
}

impl HasHeader for SequenceReset {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}
