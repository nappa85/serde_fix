
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventDateBusinessCenterGrp {
	/// NoLegComplexEventDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41387")]
	pub leg_complex_event_date_business_centers: Option<fix_common::RepeatingValues<LegComplexEventDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventDateBusinessCenter {
	/// Required if NoLegComplexEventDateBusinessCenters(41387) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41388")]
	pub leg_complex_event_date_business_center: Option<String>,
}
