
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCalculationPeriodBusinessCenterGrp {
	/// NoUnderlyingStreamCalculationPeriodBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40973")]
	pub underlying_stream_calculation_period_business_centers: Option<fix_common::RepeatingValues<UnderlyingStreamCalculationPeriodBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCalculationPeriodBusinessCenter {
	/// Required if NoUnderlyingStreamCalculationPeriodBusinessCenters(40973) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40557")]
	pub underlying_stream_calculation_period_business_center: Option<String>,
}
