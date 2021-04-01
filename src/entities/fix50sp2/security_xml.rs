
use serde::{Serialize, Deserialize};

use crate::entities::EncodedText;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityXML {
    /// Must be provided if SecurityXML field is specified and must immediately precede it.
    #[serde(rename = "1184")]
    #[serde(alias = "1185")]
    pub security_xml: Option<EncodedText<1185>>,
    /// SecurityXMLSchema
    #[serde(rename = "1186")]
    pub security_xml_schema: Option<String>,
}
