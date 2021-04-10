
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamInitialFixingDateBusinessCenterGrp {
	/// NoLegPaymentStreamInitialFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40932")]
	pub leg_payment_stream_initial_fixing_date_business_centers: Option<fix_common::RepeatingValues<LegPaymentStreamInitialFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamInitialFixingDateBusinessCenter {
	/// Required if NoLegPaymentStreamInitialFixingDateBusinessCenters(40932) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40311")]
	pub leg_payment_stream_initial_fixing_date_business_center: Option<String>,
}
