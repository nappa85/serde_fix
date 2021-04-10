
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStubStartDateBusinessCenterGrp {
	/// NoPaymentStubStartDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42705")]
	pub payment_stub_start_date_business_centers: Option<crate::entities::RepeatingValues<PaymentStubStartDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStubStartDateBusinessCenter {
	/// Required if NoPaymentStubStartDateBusinessCenters(42705) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42706")]
	pub payment_stub_start_date_business_center: Option<String>,
}
