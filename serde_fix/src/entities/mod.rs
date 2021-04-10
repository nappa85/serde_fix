
use serde::{Deserialize, Serialize};

pub mod fixt11;
pub mod fix50;
// pub mod fix50sp1;
pub mod fix50sp2;

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "8")]
pub enum Message {
    #[serde(rename = "FIXT.1.1")]
    FIXT11(fixt11::messages::Message)
}

impl Serialize for Message {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Message::FIXT11(m) => m.serialize(serializer),
        }
    }
}
