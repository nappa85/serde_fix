
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamCompoundingDatesBusinessCenterGrp {
	/// NoPaymentStreamCompoundingDatesBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42620")]
	pub payment_stream_compounding_dates_business_centers: Option<crate::entities::RepeatingValues<PaymentStreamCompoundingDatesBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamCompoundingDatesBusinessCenter {
	/// Required if NoPaymentStreamCompoundingDatesBusinessCenters(42620) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42621")]
	pub payment_stream_compounding_dates_business_center: Option<String>,
}
