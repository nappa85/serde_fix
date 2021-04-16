
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DerivativeSecurityXML {
	/// Must be set if SecurityXML field is specified and must immediately precede it.
	#[serde(rename = "1282")]
	/// XML Data Stream describing the Security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1283")]
	pub derivative_security_xml: Option<fix_common::EncodedText<1283>>,
	/// XML Schema used to validate the XML used to describe the Security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1284")]
	pub derivative_security_xml_schema: Option<String>,
}
