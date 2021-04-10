
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPricingDateBusinessCenterGrp {
	/// NoUnderlyingPricingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41947")]
	pub underlying_pricing_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPricingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPricingDateBusinessCenter {
	/// Required if NoUnderlyingPricingDateBusinessCenters(41947) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41948")]
	pub underlying_pricing_date_business_center: Option<String>,
}
