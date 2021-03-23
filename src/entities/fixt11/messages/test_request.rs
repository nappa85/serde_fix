
use serde::{Serialize, Deserialize};

// use import_fields::import_fields;

// use crate::entities::{ApplVerID, Boolean, FixDateTime, fixt11::{header::*, trailer::Signature}};

/// MsgType = 1
// #[import_fields("src/entities/fixt11/header.rs::Header", "src/entities/fixt11/trailer.rs::Trailer")]
#[derive(Serialize, Deserialize, Debug)]
pub struct TestRequest {
    #[serde(flatten)]
    pub header: crate::entities::fixt11::Header,
    /// TestReqID
    #[serde(rename = "112")]
    pub test_req_id: String,
    #[serde(flatten)]
    pub trailer: crate::entities::fixt11::Trailer,
}
