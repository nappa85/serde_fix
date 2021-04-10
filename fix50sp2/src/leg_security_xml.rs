
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegSecurityXML {
	/// Must be provided if LegSecurityXML(1872) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1871")]
	pub leg_security_xml_len: Option<usize>,
	/// LegSecurityXML
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1872")]
	pub leg_security_xml: Option<String>,
	/// LegSecurityXMLSchema
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1873")]
	pub leg_security_xml_schema: Option<String>,
}
