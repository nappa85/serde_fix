
use serde::{Serialize, Deserialize};

use crate::entities::fixt11::header::{Header, HasHeader};

/// MsgType = 2
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResendRequest {
    #[serde(flatten)]
    pub header: crate::entities::fixt11::Header,
    /// BeginSeqNo
    #[serde(rename = "7")]
    pub begin_seq_no: u64,
    /// EndSeqNo
    #[serde(rename = "16")]
    pub end_seq_no: u64,
    #[serde(flatten)]
    pub trailer: crate::entities::fixt11::Trailer,
}

impl HasHeader for ResendRequest {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}
