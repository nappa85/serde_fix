
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStubStartDateBusinessCenterGrp {
	/// NoLegPaymentStubStartDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42504")]
	pub leg_payment_stub_start_date_business_centers: Option<fix_common::RepeatingValues<LegPaymentStubStartDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStubStartDateBusinessCenter {
	/// Required if NoLegPaymentStubStartDateBusinessCenters(42504) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42505")]
	pub leg_payment_stub_start_date_business_center: Option<String>,
}
