
use serde::{Deserialize, Serialize};

mod workarounds;
// pub mod data_field;
pub mod datetime;
pub use datetime::*;
pub mod boolean;
pub use boolean::Boolean;
pub mod separated_values;
pub use separated_values::SeparatedValues;
pub mod repeating_values;
pub use repeating_values::RepeatingValues;
pub mod version;
pub use version::ApplVerID;
pub mod encoded_text;
pub use encoded_text::EncodedText;
pub mod currency;
pub use currency::Currency;
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
