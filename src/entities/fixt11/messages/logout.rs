
use serde::{Serialize, Deserialize};

use crate::entities::{fixt11::{header::{Header, HasHeader, MsgType}, Trailer}, version::FixVersion};

/// MsgType = 5
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Logout {
    #[serde(flatten)]
    pub header: Header,
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
    pub trailer: Trailer,
}

impl Logout {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HasHeader for Logout {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl Default for Logout {
    fn default() -> Self {
        Logout {
            header: Header {
                begin_string: Some(FixVersion::FIXT11),
                msg_type: Some(MsgType::Logout),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
