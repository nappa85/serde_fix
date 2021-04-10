
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamNonDeliverableFixingDatesBusinessCenterGrp {
	/// NoUnderlyingPaymentStreamNonDeliverableFixingDatesBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40968")]
	pub underlying_payment_stream_non_deliverable_fixing_dates_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPaymentStreamNonDeliverableFixingDatesBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamNonDeliverableFixingDatesBusinessCenter {
	/// Required if NoUnderlyingPaymentStreamNonDeliverableFixingDatesBusinessCenters(40968) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40650")]
	pub underlying_payment_stream_non_deliverable_fixing_dates_business_center: Option<String>,
}
