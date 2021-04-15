
#![warn(unused_extern_crates)]
#![allow(clippy::upper_case_acronyms)]

use serde::{Deserialize, Serialize};

pub mod fix43;
pub mod fix44;
pub mod fixt11;

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "8")]
pub enum Message {
    // #[serde(rename = "FIX.4.0")]
    // FIX40,
    // #[serde(rename = "FIX.4.1")]
    // FIX41,
    // #[serde(rename = "FIX.4.2")]
    // FIX42,
    #[serde(rename = "FIX.4.3")]
    FIX43(fix43::Message),
    #[serde(rename = "FIX.4.4")]
    FIX44(fix44::Message),
    #[serde(rename = "FIXT.1.1")]
    FIXT11(fixt11::Message)
}

impl Serialize for Message {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Message::FIX43(m) => m.serialize(serializer),
            Message::FIX44(m) => m.serialize(serializer),
            Message::FIXT11(m) => m.serialize(serializer),
        }
    }
}
