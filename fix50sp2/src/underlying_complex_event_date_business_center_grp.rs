
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventDateBusinessCenterGrp {
	/// NoUnderlyingComplexEventDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41737")]
	pub underlying_complex_event_date_business_centers: Option<fix_common::RepeatingValues<UnderlyingComplexEventDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventDateBusinessCenter {
	/// Required if NoUnderlyingComplexEventDateBusinessCenters(41737) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41738")]
	pub underlying_complex_event_date_business_center: Option<String>,
}
