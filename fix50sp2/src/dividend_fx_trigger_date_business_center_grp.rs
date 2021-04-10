
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendFXTriggerDateBusinessCenterGrp {
	/// NoDividendFXTriggerDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42272")]
	pub dividend_fx_trigger_date_business_centers: Option<crate::entities::RepeatingValues<DividendFXTriggerDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendFXTriggerDateBusinessCenter {
	/// Required if NoDividendFXTriggerDateBusinessCenters(42272) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42273")]
	pub dividend_fx_trigger_date_business_center: Option<String>,
}
