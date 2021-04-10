
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamResetDateBusinessCenterGrp {
	/// NoPaymentStreamResetDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40948")]
	pub payment_stream_reset_date_business_centers: Option<fix_common::RepeatingValues<PaymentStreamResetDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamResetDateBusinessCenter {
	/// Required if NoPaymentStreamResetDateBusinessCenters(40948) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40763")]
	pub payment_stream_reset_date_business_center: Option<String>,
}
