
use serde::{Serialize, Deserialize};

// use import_fields::import_fields;

// use crate::entities::{ApplVerID, Boolean, FixDateTime, fixt11::{header::*, trailer::Signature}};

/// MsgType = 4
// #[import_fields("src/entities/fixt11/header.rs::Header", "src/entities/fixt11/trailer.rs::Trailer")]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SequenceReset {
    #[serde(flatten)]
    pub header: crate::entities::fixt11::Header,
    /// GapFillFlag
    #[serde(rename = "123")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_fill_flag: Option<crate::entities::Boolean>,
    /// NewSeqNo
    #[serde(rename = "36")]
    pub new_seq_no: u64,
    #[serde(flatten)]
    pub trailer: crate::entities::fixt11::Trailer,
}
