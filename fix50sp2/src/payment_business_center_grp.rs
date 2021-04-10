
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentBusinessCenterGrp {
	/// NoPaymentBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40944")]
	pub payment_business_centers: Option<crate::entities::RepeatingValues<PaymentBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentBusinessCenter {
	/// Required if NoPaymentBusinessCenters(40944) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40221")]
	pub payment_business_center: Option<String>,
}
