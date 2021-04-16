
use serde::{Serialize, Deserialize};

use crate::{StandardMessageHeader, StandardMessageTrailer};

/// MsgType = 1
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestRequest {
    #[serde(flatten)]
    pub standard_message_header: StandardMessageHeader<5, '1', ' '>,
    /// TestReqID
    #[serde(rename = "112")]
    pub test_req_id: String,
    #[serde(flatten)]
    pub standard_message_trailer: StandardMessageTrailer,
}
