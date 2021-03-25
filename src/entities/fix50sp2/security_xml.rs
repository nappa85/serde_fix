
use serde::{Serialize, Deserialize};

use crate::entities::data_field;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecurityXML {
    /// Must be provided if SecurityXML field is specified and must immediately precede it.
    #[serde(rename = "1184")]
    #[serde(alias = "1185")]
    pub security_xml: Option<SecurityXMLData>,
    /// SecurityXMLSchema
    #[serde(rename = "1186")]
    pub security_xml_schema: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SecurityXMLData {
    // #[serde(rename = "1184")]
    len: usize,
    // #[serde(rename = "1185")]
    data: String,
}

impl data_field::DataField for SecurityXMLData {
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

impl<'de> Deserialize<'de> for SecurityXMLData {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "1185")
    }
}

impl Serialize for SecurityXMLData {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "1185")
    }
}
