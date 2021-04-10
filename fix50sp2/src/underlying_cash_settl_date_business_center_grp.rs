
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingCashSettlDateBusinessCenterGrp {
	/// NoUnderlyingCashSettlDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42788")]
	pub underlying_cash_settl_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingCashSettlDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingCashSettlDateBusinessCenter {
	/// Required if NoUnderlyingCashSettlDateBusinessCenters(42788) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42789")]
	pub underlying_cash_settl_date_business_center: Option<String>,
}
