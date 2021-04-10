
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityXML {
	/// Must be provided if SecurityXML field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1184")]
	pub security_xml_len: Option<usize>,
	/// SecurityXML
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1185")]
	pub security_xml: Option<String>,
	/// SecurityXMLSchema
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1186")]
	pub security_xml_schema: Option<String>,
}
