
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamFirstPeriodStartDateBusinessCenterGrp {
	/// NoStreamFirstPeriodStartDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40959")]
	pub stream_first_period_start_date_business_centers: Option<fix_common::RepeatingValues<StreamFirstPeriodStartDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamFirstPeriodStartDateBusinessCenter {
	/// Required if NoStreamFirstPeriodStartDateBusinessCenters(40959) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40077")]
	pub stream_first_period_start_date_business_center: Option<String>,
}
