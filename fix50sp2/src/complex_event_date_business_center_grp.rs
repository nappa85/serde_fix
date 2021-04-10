
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventDateBusinessCenterGrp {
	/// NoComplexEventDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41018")]
	pub complex_event_date_business_centers: Option<crate::entities::RepeatingValues<ComplexEventDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventDateBusinessCenter {
	/// Required if NoComplexEventDateBuisinessCenters(41018) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41019")]
	pub complex_event_date_business_center: Option<String>,
}
