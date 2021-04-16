
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamInitialFixingDateBusinessCenterGrp {
	/// NoUnderlyingPaymentStreamInitialFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40971")]
	pub underlying_payment_stream_initial_fixing_date_business_centers: Option<fix_common::RepeatingValues<UnderlyingPaymentStreamInitialFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamInitialFixingDateBusinessCenter {
	/// Required if NoUnderlyingPaymentStreamInitialFixingDateBusinessCenters(40971) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40600")]
	pub underlying_payment_stream_initial_fixing_date_business_center: Option<String>,
}
