
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRateValuationDateBusinessCenterGrp {
	/// NoReturnRateValuationDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42770")]
	pub return_rate_valuation_date_business_centers: Option<fix_common::RepeatingValues<ReturnRateValuationDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRateValuationDateBusinessCenter {
	/// Required if NoReturnRateValuationDateBusinessCenters(42770) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42771")]
	pub return_rate_valuation_date_business_center: Option<String>,
}
