
use serde::{Serialize, Deserialize};

use crate::entities::fixt11::header::{Header, HasHeader};

/// MsgType = 0
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Heartbeat {
    #[serde(flatten)]
    pub header: crate::entities::fixt11::Header,
    /// Required when the heartbeat is the result of a Test Request (1) message.
    #[serde(rename = "112")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_req_id: Option<String>,
    #[serde(flatten)]
    pub trailer: crate::entities::fixt11::Trailer,
}

impl HasHeader for Heartbeat {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}
