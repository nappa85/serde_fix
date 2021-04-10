
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamEffectiveBusinessCenterGrp {
	/// NoStreamEffectiveBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40960")]
	pub stream_effective_business_centers: Option<crate::entities::RepeatingValues<StreamEffectiveBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamEffectiveBusinessCenter {
	/// Required if NoStreamEffectiveBusinessCenters(40960) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40909")]
	pub stream_effective_date_business_center: Option<String>,
}
