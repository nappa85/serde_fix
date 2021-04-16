
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCalculationPeriodBusinessCenterGrp {
	/// NoLegStreamCalculationPeriodBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40940")]
	pub leg_stream_calculation_period_business_centers: Option<fix_common::RepeatingValues<LegStreamCalculationPeriodBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamCalculationPeriodBusinessCenter {
	/// Required if NoLegStreamCalculationPeriodBusinessCenters(40940) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40266")]
	pub leg_stream_calculation_period_business_center: Option<String>,
}
