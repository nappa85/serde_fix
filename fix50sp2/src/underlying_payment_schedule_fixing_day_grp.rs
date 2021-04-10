
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentScheduleFixingDayGrp {
	/// NoUnderlyingPaymentScheduleFixingDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41878")]
	pub underlying_payment_schedule_fixing_days: Option<crate::entities::RepeatingValues<UnderlyingPaymentScheduleFixingDay>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentScheduleFixingDay {
	/// Required if NoUnderlyingPaymentScheduleFixingDays(41878) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41879")]
	pub underlying_payment_schedule_fixing_day_of_week: Option<UnderlyingPaymentScheduleFixingDayOfWeek>,
	/// UnderlyingPaymentScheduleFixingDayNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41880")]
	pub underlying_payment_schedule_fixing_day_number: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingPaymentScheduleFixingDayOfWeek {
	/// Every day (the default if not specified)
	#[serde(rename = "0")]
	EveryDay,
	/// Monday
	#[serde(rename = "1")]
	Monday,
	/// Tuesday
	#[serde(rename = "2")]
	Tuesday,
	/// Wednesday
	#[serde(rename = "3")]
	Wednesday,
	/// Thursday
	#[serde(rename = "4")]
	Thursday,
	/// Friday
	#[serde(rename = "5")]
	Friday,
	/// Saturday
	#[serde(rename = "6")]
	Saturday,
	/// Sunday
	#[serde(rename = "7")]
	Sunday,
}
