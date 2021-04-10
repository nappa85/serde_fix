
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStubEndDateBusinessCenterGrp {
	/// NoUnderlyingPaymentStubEndDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42991")]
	pub underlying_payment_stub_end_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPaymentStubEndDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStubEndDateBusinessCenter {
	/// Required if NoUnderlyingPaymentStubEndDateBusinessCenters(42991) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42992")]
	pub underlying_payment_stub_end_date_business_center: Option<String>,
}
