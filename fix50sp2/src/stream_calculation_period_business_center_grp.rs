
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCalculationPeriodBusinessCenterGrp {
	/// NoStreamCalculationPeriodBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40958")]
	pub stream_calculation_period_business_centers: Option<fix_common::RepeatingValues<StreamCalculationPeriodBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCalculationPeriodBusinessCenter {
	/// Required if NoStreamCalculationPeriodBusinessCenters(40958) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40074")]
	pub stream_calculation_period_business_center: Option<String>,
}
