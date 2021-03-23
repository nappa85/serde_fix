
use serde::Serialize;

use crate::entities::fixt11::header::MsgType;

mod ioi;
pub use ioi::IoI;

#[derive(Serialize, Debug)]
#[serde(tag = "35")]
pub enum Message {
    IndicationOfInterest(IoI),
}

impl Message {
    /// approach due to https://github.com/serde-rs/serde/issues/1183
    pub fn from_str(t: &MsgType, s: &str) -> Result<Message, serde::de::value::Error> {
        Ok(match t {
            MsgType::IndicationOfInterest => Message::IndicationOfInterest(crate::from_str(s)?),
            _ => unimplemented!(),
        })
    }
}
