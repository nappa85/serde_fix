
use serde::{Serialize, Deserialize};

use crate::{Header, Trailer};

/// MsgType = 1
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestRequest {
    #[serde(flatten)]
    pub header: Header<5, '1', ' '>,
    /// TestReqID
    #[serde(rename = "112")]
    pub test_req_id: String,
    #[serde(flatten)]
    pub trailer: Trailer,
}
