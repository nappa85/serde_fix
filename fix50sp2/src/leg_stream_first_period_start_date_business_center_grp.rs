
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamFirstPeriodStartDateBusinessCenterGrp {
	/// NoLegStreamFirstPeriodStartDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40941")]
	pub leg_stream_first_period_start_date_business_centers: Option<crate::entities::RepeatingValues<LegStreamFirstPeriodStartDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamFirstPeriodStartDateBusinessCenter {
	/// Required if NoLegStreamFirstPeriodStartDateBusinessCenters(40941) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40269")]
	pub leg_stream_first_period_start_date_business_center: Option<String>,
}
