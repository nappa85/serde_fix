
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStubStartDateBusinessCenterGrp {
	/// NoUnderlyingPaymentStubStartDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43000")]
	pub underlying_payment_stub_start_date_business_centers: Option<fix_common::RepeatingValues<UnderlyingPaymentStubStartDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStubStartDateBusinessCenter {
	/// Required if NoUnderlyingPaymentStubStartDateBusinessCenters(43000) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43001")]
	pub underlying_payment_stub_start_date_business_center: Option<String>,
}
