
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStubEndDateBusinessCenterGrp {
	/// NoPaymentStubEndDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42696")]
	pub payment_stub_end_date_business_centers: Option<crate::entities::RepeatingValues<PaymentStubEndDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStubEndDateBusinessCenter {
	/// Required if NoPaymentStubEndDateBusinessCenters(42696) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42697")]
	pub payment_stub_end_date_business_center: Option<String>,
}
