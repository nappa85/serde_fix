
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentScheduleFixingDateBusinessCenterGrp {
	/// NoPaymentScheduleFixingDateBusinessCenters
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40977")]
	pub payment_schedule_fixing_date_business_centers: Option<fix_common::RepeatingValues<PaymentScheduleFixingDateBusinessCenter>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentScheduleFixingDateBusinessCenter {
	/// Required if NoPaymentScheduleFixingDateBusinessCenters(40977) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40854")]
	pub payment_schedule_fixing_date_business_center: Option<String>,
}
