
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamEffectiveDateBusinessCenterGrp {
	/// NoUnderlyingStreamEffectiveDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40975")]
	pub underlying_stream_effective_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingStreamEffectiveDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamEffectiveDateBusinessCenter {
	/// Required if NoUnderlyingStreamEffectiveDateBusinessCenters(40975) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40059")]
	pub underlying_stream_effective_date_business_center: Option<String>,
}
