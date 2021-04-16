
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegCashSettlDateBusinessCenterGrp {
	/// NoLegCashSettlDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42306")]
	pub leg_cash_settl_date_business_centers: Option<fix_common::RepeatingValues<LegCashSettlDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegCashSettlDateBusinessCenter {
	/// Required if NoLegCashSettlDateBusinessCenters(42306) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42307")]
	pub leg_cash_settl_date_business_center: Option<String>,
}
