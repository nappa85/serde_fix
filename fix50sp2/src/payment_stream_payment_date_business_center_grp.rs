
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPaymentDateBusinessCenterGrp {
	/// NoPaymentStreamPaymentDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40947")]
	pub payment_stream_payment_date_business_centers: Option<crate::entities::RepeatingValues<PaymentStreamPaymentDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamPaymentDateBusinessCenter {
	/// Required if NoPaymentStreamPaymentDateBusinessCenters(40947) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40752")]
	pub payment_stream_payment_date_business_center: Option<String>,
}
