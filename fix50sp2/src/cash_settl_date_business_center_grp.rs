
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CashSettlDateBusinessCenterGrp {
	/// NoCashSettlDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42214")]
	pub cash_settl_date_business_centers: Option<crate::entities::RepeatingValues<CashSettlDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CashSettlDateBusinessCenter {
	/// Required if NoCashSettlDateBusinessCenters(42214) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42215")]
	pub cash_settl_date_business_center: Option<String>,
}
