
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DerivativeSecurityXML {
	/// Must be set if SecurityXML field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1282")]
	pub derivative_security_xml_len: Option<usize>,
	/// XML Data Stream describing the Security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1283")]
	pub derivative_security_xml: Option<String>,
	/// XML Schema used to validate the XML used to describe the Security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1284")]
	pub derivative_security_xml_schema: Option<String>,
}
