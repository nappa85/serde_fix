
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamResetDateBusinessCenterGrp {
	/// NoUnderlyingPaymentStreamResetDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40970")]
	pub underlying_payment_stream_reset_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPaymentStreamResetDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamResetDateBusinessCenter {
	/// Required if NoUnderlyingPaymentStreamResetDateBusinessCenters(40970) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40594")]
	pub underlying_payment_stream_reset_date_business_center: Option<String>,
}
