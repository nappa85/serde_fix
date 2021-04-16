
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateValuationDateBusinessCenterGrp {
	/// NoUnderlyingReturnRateValuationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43069")]
	pub underlying_return_rate_valuation_date_business_centers: Option<fix_common::RepeatingValues<UnderlyingReturnRateValuationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateValuationDateBusinessCenter {
	/// Required if NoUnderlyingReturnRateValuationDateBusinessCenters(43069) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43070")]
	pub underlying_return_rate_valuation_date_business_center: Option<String>,
}
