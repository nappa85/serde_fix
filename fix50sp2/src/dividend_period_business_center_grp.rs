
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendPeriodBusinessCenterGrp {
	/// NoDividendPeriodBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42294")]
	pub dividend_period_business_centers: Option<crate::entities::RepeatingValues<DividendPeriodBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendPeriodBusinessCenter {
	/// Required if NoDividendPeriodBusinessCenters(42294) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42295")]
	pub dividend_period_business_center: Option<String>,
}
