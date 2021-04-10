
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamFirstPeriodStartDateBusinessCenterGrp {
	/// NoUnderlyingStreamFirstPeriodStartDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40974")]
	pub underlying_stream_first_period_start_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingStreamFirstPeriodStartDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamFirstPeriodStartDateBusinessCenter {
	/// Required if NoUnderlyginstreamFirstPeriodStartDateBusinessCenters(40974) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40560")]
	pub underlying_stream_first_period_start_date_business_center: Option<String>,
}
