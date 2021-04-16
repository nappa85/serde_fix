
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFixingDateBusinessCenterGrp {
	/// NoPaymentStreamFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40950")]
	pub payment_stream_fixing_date_business_centers: Option<fix_common::RepeatingValues<PaymentStreamFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFixingDateBusinessCenter {
	/// Required if NoPaymentStreamFixingDateBusinessCenters(40950) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40776")]
	pub payment_stream_fixing_date_business_center: Option<String>,
}
