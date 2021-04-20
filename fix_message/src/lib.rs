
#![warn(unused_extern_crates)]
#![allow(clippy::upper_case_acronyms)]

use serde::{Deserialize, Serialize};

pub mod header;
pub mod has_header;

#[cfg(feature = "fix_40")]
pub mod fix40;
#[cfg(feature = "fix_41")]
pub mod fix41;
#[cfg(feature = "fix_42")]
pub mod fix42;
#[cfg(feature = "fix_43")]
pub mod fix43;
#[cfg(feature = "fix_44")]
pub mod fix44;
#[cfg(any(feature = "fix_50", feature = "fix_50sp1", feature = "fix_50sp2"))]
pub mod fixt11;

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "8")]
pub enum Message {
    #[cfg(feature = "fix_40")]
    #[serde(rename = "FIX.4.0")]
    FIX40(fix40::Message),
    #[cfg(feature = "fix_41")]
    #[serde(rename = "FIX.4.1")]
    FIX41(fix41::Message),
    #[cfg(feature = "fix_42")]
    #[serde(rename = "FIX.4.2")]
    FIX42(fix42::Message),
    #[cfg(feature = "fix_43")]
    #[serde(rename = "FIX.4.3")]
    FIX43(fix43::Message),
    #[cfg(feature = "fix_44")]
    #[serde(rename = "FIX.4.4")]
    FIX44(fix44::Message),
    #[cfg(any(feature = "fix_50", feature = "fix_50sp1", feature = "fix_50sp2"))]
    #[serde(rename = "FIXT.1.1")]
    FIXT11(fixt11::Message)
}

impl Serialize for Message {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_40")]
            Message::FIX40(m) => m.serialize(serializer),
            #[cfg(feature = "fix_41")]
            Message::FIX41(m) => m.serialize(serializer),
            #[cfg(feature = "fix_42")]
            Message::FIX42(m) => m.serialize(serializer),
            #[cfg(feature = "fix_43")]
            Message::FIX43(m) => m.serialize(serializer),
            #[cfg(feature = "fix_44")]
            Message::FIX44(m) => m.serialize(serializer),
            #[cfg(any(feature = "fix_50", feature = "fix_50sp1", feature = "fix_50sp2"))]
            Message::FIXT11(m) => m.serialize(serializer),
        }
    }
}
