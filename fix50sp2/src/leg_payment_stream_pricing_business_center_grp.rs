
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamPricingBusinessCenterGrp {
	/// NoLegPaymentStreamPricingBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41561")]
	pub leg_payment_stream_pricing_business_centers: Option<fix_common::RepeatingValues<LegPaymentStreamPricingBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamPricingBusinessCenter {
	/// Required if NoLegPaymentStreamPricingBusinessCentrers(41561) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41562")]
	pub leg_payment_stream_pricing_business_center: Option<String>,
}
