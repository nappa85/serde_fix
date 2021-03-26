
use serde::{Serialize, Deserialize};

use crate::entities::{fixt11::{header::{HasHeader, Header, MsgType}, Trailer}, version::FixVersion};

/// MsgType = 0
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Heartbeat {
    #[serde(flatten)]
    pub header: Header,
    /// Required when the heartbeat is the result of a Test Request (1) message.
    #[serde(rename = "112")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_req_id: Option<String>,
    #[serde(flatten)]
    pub trailer: Trailer,
}

impl Heartbeat {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HasHeader for Heartbeat {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl Default for Heartbeat {
    fn default() -> Self {
        Heartbeat {
            header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::Heartbeat),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
