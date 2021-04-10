
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BusinessCenterGrp {
	/// NoBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40278")]
	pub business_centers: Option<crate::entities::RepeatingValues<BusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BusinessCenter {
	/// Required if NoBusinessCenters(40278) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40471")]
	pub business_center: Option<String>,
}
