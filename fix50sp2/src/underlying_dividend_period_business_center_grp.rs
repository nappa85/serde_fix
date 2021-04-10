
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendPeriodBusinessCenterGrp {
	/// NoUnderlyingDividendPeriodBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42882")]
	pub underlying_dividend_period_business_centers: Option<crate::entities::RepeatingValues<UnderlyingDividendPeriodBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendPeriodBusinessCenter {
	/// Required if NoUnderlyingDividendPeriodBusinessCenters(42882) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42883")]
	pub underlying_dividend_period_business_center: Option<String>,
}
