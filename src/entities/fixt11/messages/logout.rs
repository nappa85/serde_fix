
use serde::{Serialize, Deserialize};

// use import_fields::import_fields;

// use crate::entities::{ApplVerID, Boolean, FixDateTime, fixt11::{header::*, trailer::Signature}};

/// MsgType = 5
// #[import_fields("src/entities/fixt11/header.rs::Header", "src/entities/fixt11/trailer.rs::Trailer")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Logout {
    #[serde(flatten)]
    pub header: crate::entities::fixt11::Header,
    /// Session status at time of logout.
    #[serde(rename = "1409")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_status: Option<super::logon::SessionStatus>,
    /// Text
    #[serde(rename = "58")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Must be set if EncodedText (355) field is specified and must immediately precede it.
    #[serde(rename = "354")]
    /// Encoded (non-ASCII characters) representation of the Text (58) field in the encoded format specified via the MessageEncoding (347) field. 
    #[serde(alias = "355")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_text: Option<super::logon::EncodedText>,
    #[serde(flatten)]
    pub trailer: crate::entities::fixt11::Trailer,
}
