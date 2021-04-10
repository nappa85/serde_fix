
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegBusinessCenterGrp {
	/// NoLegBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40923")]
	pub leg_business_centers: Option<crate::entities::RepeatingValues<LegBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegBusinessCenter {
	/// Required if NoLegBusinessCenters(40923) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40924")]
	pub leg_business_center: Option<String>,
}
