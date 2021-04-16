
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PricingDateBusinessCenterGrp {
	/// NoPricingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41230")]
	pub pricing_date_business_centers: Option<fix_common::RepeatingValues<PricingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PricingDateBusinessCenter {
	/// Required if NoPricingDateBusinessCenters(41230) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41231")]
	pub pricing_date_business_center: Option<String>,
}
