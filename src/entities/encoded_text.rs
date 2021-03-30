
use serde::{Deserialize, Serialize};

use super::data_field;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EncodedText {
    // #[serde(rename = "354")]
    len: usize,
    // #[serde(rename = "355")]
    data: String,
}

impl data_field::DataField for EncodedText {
    fn get_len(&self) -> &usize {
        &self.len
    }
    fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    fn get_data(&self) -> &str {
        &self.data
    }
    fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

impl<'de> Deserialize<'de> for EncodedText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "355")
    }
}

impl Serialize for EncodedText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "355")
    }
}
