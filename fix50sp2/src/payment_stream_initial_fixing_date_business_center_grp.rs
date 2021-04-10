
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamInitialFixingDateBusinessCenterGrp {
	/// NoPaymentStreamInitialFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40949")]
	pub payment_stream_initial_fixing_date_business_centers: Option<fix_common::RepeatingValues<PaymentStreamInitialFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamInitialFixingDateBusinessCenter {
	/// Required if NoPaymentStreamInitialFixindDateBusinessCenters(40949) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40769")]
	pub payment_stream_initial_fixing_date_business_center: Option<String>,
}
