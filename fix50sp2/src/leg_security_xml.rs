
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegSecurityXML {
	/// Must be provided if LegSecurityXML(1872) field is specified and must immediately precede it.
	#[serde(rename = "1871")]
	/// LegSecurityXML
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1872")]
	pub leg_security_xml: Option<fix_common::EncodedText<1872>>,
	/// LegSecurityXMLSchema
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1873")]
	pub leg_security_xml_schema: Option<String>,
}
