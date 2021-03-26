
use serde::{Serialize, Deserialize};

use crate::entities::{fixt11::{header::{Header, HasHeader, MsgType}, Trailer}, version::FixVersion};

/// MsgType = 1
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
        Self::default()
    }
}

impl HasHeader for TestRequest {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl Default for TestRequest {
    fn default() -> Self {
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
