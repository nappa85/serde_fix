
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingSecurityXML {
	/// Must be provided if UnderlyingSecurityXML(1875) field is specified and must immediately precede it.
	#[serde(rename = "1874")]
	/// UnderlyingSecurityXML
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1875")]
	pub underlying_security_xml: Option<fix_common::EncodedText<1875>>,
	/// UnderlyingSecurityXMLSchema
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1876")]
	pub underlying_security_xml_schema: Option<String>,
}
