
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDRjctGrp {
	/// NoAltMDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "816")]
	pub alt_md_source: Option<crate::entities::RepeatingValues<AltMDSourc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AltMDSourc {
	/// Alternative Market Data Source
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "817")]
	pub alt_md_source_id: Option<String>,
}
