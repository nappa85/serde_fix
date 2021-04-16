
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFixingDateBusinessCenterGrp {
	/// NoLegPaymentStreamFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40933")]
	pub leg_payment_stream_fixing_date_business_centers: Option<fix_common::RepeatingValues<LegPaymentStreamFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFixingDateBusinessCenter {
	/// Required if NoLegPaymentStreamFixingDateBusinessCenters(40933) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40318")]
	pub leg_payment_stream_fixing_date_business_center: Option<String>,
}
