
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamNonDeliverableFixingDatesBusinessCenterGrp {
	/// NoPaymentStreamNonDeliverableFixingDatesBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40946")]
	pub payment_stream_non_deliverable_fixing_dates_business_centers: Option<fix_common::RepeatingValues<PaymentStreamNonDeliverableFixingDatesBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamNonDeliverableFixingDatesBusinessCenter {
	/// Required if NoPaymentStreamNonDeliverableFixingDatesBusinessCenters(40946) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40819")]
	pub payment_stream_non_deliverable_fixing_dates_business_center: Option<String>,
}
