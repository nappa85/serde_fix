
use serde::{Serialize, Deserialize};

use fix_common::FixVersion;
use crate::{header::{Header, MsgType}, Trailer};

/// MsgType = 1
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestRequest {
    #[serde(flatten)]
    pub header: Header,
    /// TestReqID
    #[serde(rename = "112")]
    pub test_req_id: String,
    #[serde(flatten)]
    pub trailer: Trailer,
}

impl TestRequest {
    pub fn new() -> Self {
        TestRequest {
            header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::TestRequest),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
