
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSecurityXML {
	/// Must be provided if UnderlyingSecurityXML(1875) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1874")]
	pub underlying_security_xml_len: Option<usize>,
	/// UnderlyingSecurityXML
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1875")]
	pub underlying_security_xml: Option<String>,
	/// UnderlyingSecurityXMLSchema
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1876")]
	pub underlying_security_xml_schema: Option<String>,
}
