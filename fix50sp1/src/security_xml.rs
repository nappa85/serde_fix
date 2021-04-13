
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityXML {
	/// Must be set if SecurityXML field is specified and must immediately precede it.
	#[serde(rename = "1184")]
	/// XML payload or content describing the Security information.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1185")]
	pub security_xml: Option<fix_common::EncodedText<1185>>,
	/// XML Schema used to validate the XML used to describe the Security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1186")]
	pub security_xml_schema: Option<String>,
}
