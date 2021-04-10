
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPaymentDateBusinessCenterGrp {
	/// NoUnderlyingPaymentStreamPaymentDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40969")]
	pub underlying_payment_stream_payment_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPaymentStreamPaymentDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamPaymentDateBusinessCenter {
	/// Required if NoUnderlyingPaymentStreamPaymentDateBusinessCenters(40969) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40582")]
	pub underlying_payment_stream_payment_date_business_center: Option<String>,
}
