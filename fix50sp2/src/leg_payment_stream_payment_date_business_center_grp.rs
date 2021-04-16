
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamPaymentDateBusinessCenterGrp {
	/// NoLegPaymentStreamPaymentDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40930")]
	pub leg_payment_stream_payment_date_business_centers: Option<fix_common::RepeatingValues<LegPaymentStreamPaymentDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamPaymentDateBusinessCenter {
	/// Requirend if NoLegPaymentStreamPaymentDateBusinessCenters(40930) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40293")]
	pub leg_payment_stream_payment_date_business_center: Option<String>,
}
