
use serde::{Serialize, Deserialize};

use crate::entities::data_field;

/// Standard Message Header
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Trailer {
    /// Required when trailer contains signature. Note: Not to be included within SecureData (91) field
    #[serde(rename = "93")]
    #[serde(alias = "89")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Signature>,
    /// (Always unencrypted, always last field in message)
    #[serde(rename = "10")]
    #[serde(skip_serializing)]
    pub check_sum: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Signature {
    // #[serde(rename = "93")]
    len: usize,
    // #[serde(rename = "89")]
    data: String,
}

impl data_field::DataField for Signature {
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

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "89")
    }
}

impl Serialize for Signature {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "89")
    }
}
