
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStubEndDateBusinessCenterGrp {
	/// NoLegPaymentStubEndDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42495")]
	pub leg_payment_stub_end_date_business_centers: Option<crate::entities::RepeatingValues<LegPaymentStubEndDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStubEndDateBusinessCenter {
	/// Required if NoLegPaymentStubEndDateBusinessCenters(42495) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42496")]
	pub leg_payment_stub_end_date_business_center: Option<String>,
}
