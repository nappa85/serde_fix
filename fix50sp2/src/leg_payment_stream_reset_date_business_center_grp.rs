
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamResetDateBusinessCenterGrp {
	/// NoLegPaymentStreamResetDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40931")]
	pub leg_payment_stream_reset_date_business_centers: Option<fix_common::RepeatingValues<LegPaymentStreamResetDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamResetDateBusinessCenter {
	/// Required if NoLegPaymentStreamResetDateBusinessCenters(40931) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40305")]
	pub leg_payment_stream_reset_date_business_center: Option<String>,
}
