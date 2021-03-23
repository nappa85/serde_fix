
use serde::{Serialize, Deserialize};

// use import_fields::import_fields;

// use crate::entities::{ApplVerID, Boolean, FixDateTime, fixt11::{header::*, trailer::Signature}};

/// MsgType = 0
// #[import_fields("src/entities/fixt11/header.rs::Header", "src/entities/fixt11/trailer.rs::Trailer")]
#[derive(Serialize, Deserialize, Debug)]
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
