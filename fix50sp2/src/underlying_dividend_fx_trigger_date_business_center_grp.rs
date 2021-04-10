
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendFXTriggerDateBusinessCenterGrp {
	/// NoUnderlyingDividendFXTriggerDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42853")]
	pub underlying_dividend_fx_trigger_date_business_centers: Option<fix_common::RepeatingValues<UnderlyingDividendFXTriggerDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendFXTriggerDateBusinessCenter {
	/// Required if NoUnderlyingDividendFXTriggerDateBusinessCenters(42853) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42854")]
	pub underlying_dividend_fx_trigger_date_business_center: Option<String>,
}
