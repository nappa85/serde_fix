
use serde::{Serialize, Deserialize};

use crate::{Header, Trailer};

/// MsgType = 0
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Heartbeat {
    #[serde(flatten)]
    pub header: Header<5, '0', ' '>,
    /// Required when the heartbeat is the result of a Test Request (1) message.
    #[serde(rename = "112")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_req_id: Option<String>,
    #[serde(flatten)]
    pub trailer: Trailer,
}
