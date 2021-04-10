
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingBusinessCenterGrp {
	/// NoUnderlyingBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40962")]
	pub underlying_business_centers: Option<fix_common::RepeatingValues<UnderlyingBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingBusinessCenter {
	/// Required if NoUnderlyingBusinessCenters(40962) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40963")]
	pub underlying_business_center: Option<String>,
}
