
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamEffectiveDateBusinessCenterGrp {
	/// NoLegStreamEffectiveDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40942")]
	pub leg_stream_effective_date_business_centers: Option<crate::entities::RepeatingValues<LegStreamEffectiveDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamEffectiveDateBusinessCenter {
	/// Required if NoLegStreamEffectiveDateBusinessCenters(40942) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40251")]
	pub leg_stream_effective_date_business_center: Option<String>,
}
