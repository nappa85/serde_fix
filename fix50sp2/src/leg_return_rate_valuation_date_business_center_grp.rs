
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateValuationDateBusinessCenterGrp {
	/// NoLegReturnRateValuationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42569")]
	pub leg_return_rate_valuation_date_business_centers: Option<crate::entities::RepeatingValues<LegReturnRateValuationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateValuationDateBusinessCenter {
	/// Required if NoLegReturnRateValuationDateBusinessCenters(42569) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42570")]
	pub leg_return_rate_valuation_date_business_center: Option<String>,
}
