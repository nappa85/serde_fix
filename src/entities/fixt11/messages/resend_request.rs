
use serde::{Serialize, Deserialize};

// use import_fields::import_fields;

// use crate::entities::{ApplVerID, Boolean, FixDateTime, fixt11::{header::*, trailer::Signature}};

/// MsgType = 2
// #[import_fields("src/entities/fixt11/header.rs::Header", "src/entities/fixt11/trailer.rs::Trailer")]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResendRequest {
    #[serde(flatten)]
    pub header: crate::entities::fixt11::Header,
    /// BeginSeqNo
    #[serde(rename = "7")]
    pub begin_seq_no: u64,
    /// EndSeqNo
    #[serde(rename = "16")]
    pub end_seq_no: u64,
    #[serde(flatten)]
    pub trailer: crate::entities::fixt11::Trailer,
}
