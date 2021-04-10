
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendPeriodBusinessCenterGrp {
	/// NoLegDividendPeriodBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42386")]
	pub leg_dividend_period_business_centers: Option<crate::entities::RepeatingValues<LegDividendPeriodBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendPeriodBusinessCenter {
	/// Required if NoLegDividendPeriodBusinessCenters(42386) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42387")]
	pub leg_dividend_period_business_center: Option<String>,
}
