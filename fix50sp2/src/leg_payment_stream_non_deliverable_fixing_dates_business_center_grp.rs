
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamNonDeliverableFixingDatesBusinessCenterGrp {
	/// NoLegPaymentStreamNonDeliverableFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40929")]
	pub leg_payment_stream_non_deliverable_fixing_date_business_centers: Option<crate::entities::RepeatingValues<LegPaymentStreamNonDeliverableFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamNonDeliverableFixingDateBusinessCenter {
	/// Required if NoLegPaymentStreamNonDeliverableFixingDatesBusinessCenters(40929) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40361")]
	pub leg_payment_stream_non_deliverable_fixing_dates_business_center: Option<String>,
}
