
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentScheduleFixingDateBusinessCenterGrp {
	/// NoUnderlyingPaymentScheduleFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40966")]
	pub underlying_payment_schedule_fixing_date_business_centers: Option<crate::entities::RepeatingValues<UnderlyingPaymentScheduleFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentScheduleFixingDateBusinessCenter {
	/// Required if NoUnderlyingPaymentScheduleFixingDateBusinessCenters(40966) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40690")]
	pub underlying_payment_schedule_fixing_date_business_center: Option<String>,
}
