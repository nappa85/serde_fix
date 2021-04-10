
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamCompoundingDatesBusinessCenterGrp {
	/// NoUnderlyingPaymentStreamCompoundingDatesBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42915")]
	pub underlying_payment_stream_compounding_dates_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPaymentStreamCompoundingDatesBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamCompoundingDatesBusinessCenter {
	/// Required if NoUnderlyingPaymentStreamCompoundingDatesBusinessCenters(42915) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42916")]
	pub underlying_payment_stream_compounding_dates_business_center: Option<String>,
}
