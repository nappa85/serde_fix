
use serde::{Serialize, Deserialize};

use crate::entities::fixt11::header::{Header, HasHeader};

/// MsgType = 1
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestRequest {
    #[serde(flatten)]
    pub header: crate::entities::fixt11::Header,
    /// TestReqID
    #[serde(rename = "112")]
    pub test_req_id: String,
    #[serde(flatten)]
    pub trailer: crate::entities::fixt11::Trailer,
}

impl HasHeader for TestRequest {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}
