
use serde::{Serialize, Deserialize};

use crate::entities::fixt11::{header::{Header, HasHeader, MsgType}, Trailer};

/// MsgType = 2
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ResendRequest {
    #[serde(flatten)]
    pub header: Header,
    /// BeginSeqNo
    #[serde(rename = "7")]
    pub begin_seq_no: u64,
    /// EndSeqNo
    #[serde(rename = "16")]
    pub end_seq_no: u64,
    #[serde(flatten)]
    pub trailer: Trailer,
}

impl ResendRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HasHeader for ResendRequest {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl Default for ResendRequest {
    fn default() -> Self {
        ResendRequest {
            header: Header {
                msg_type: Some(MsgType::ResendRequest),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
