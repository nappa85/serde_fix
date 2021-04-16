
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityXML {
	/// Must be provided if SecurityXML field is specified and must immediately precede it.
	#[serde(rename = "1184")]
	/// SecurityXML
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1185")]
	pub security_xml: Option<fix_common::EncodedText<1185>>,
	/// SecurityXMLSchema
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1186")]
	pub security_xml_schema: Option<String>,
}
