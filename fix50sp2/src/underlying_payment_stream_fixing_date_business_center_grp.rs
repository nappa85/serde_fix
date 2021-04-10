
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamFixingDateBusinessCenterGrp {
	/// NoUnderlyingPaymentStreamFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40972")]
	pub underlying_payment_stream_fixing_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPaymentStreamFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamFixingDateBusinessCenter {
	/// Required if NoUnderlyingPaymentStreamFixingDateBusinessCenters(40972) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40607")]
	pub underlying_payment_stream_fixing_date_business_center: Option<String>,
}
