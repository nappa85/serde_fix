
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPricingBusinessCenterGrp {
	/// NoPaymentStreamPricingBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41192")]
	pub payment_stream_pricing_business_centers: Option<fix_common::RepeatingValues<PaymentStreamPricingBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPricingBusinessCenter {
	/// Required if NoPaymentStreamPricingBusinessCenters(41192) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41193")]
	pub payment_stream_pricing_business_center: Option<String>,
}
