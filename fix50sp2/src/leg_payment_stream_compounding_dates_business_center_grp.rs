
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamCompoundingDatesBusinessCenterGrp {
	/// NoLegPaymentStreamCompoundingDatesBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42419")]
	pub leg_payment_stream_compounding_dates_business_centers: Option<crate::entities::RepeatingValues<LegPaymentStreamCompoundingDatesBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamCompoundingDatesBusinessCenter {
	/// Required if NoLegPaymentStreamCompoundingDatesBusinessCenters(42419) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42420")]
	pub leg_payment_stream_compounding_dates_business_center: Option<String>,
}
