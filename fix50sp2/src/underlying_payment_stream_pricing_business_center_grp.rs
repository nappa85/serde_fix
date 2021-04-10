
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPricingBusinessCenterGrp {
	/// NoUnderlyingPaymentStreamPricingBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41909")]
	pub underlying_payment_stream_pricing_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPaymentStreamPricingBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPricingBusinessCenter {
	/// Required if NoUnderlyingPaymentStreamPricingBusinessCenters(41909) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41910")]
	pub underlying_payment_stream_pricing_business_center: Option<String>,
}
