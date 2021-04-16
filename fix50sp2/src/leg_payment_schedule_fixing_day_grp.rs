
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentScheduleFixingDayGrp {
	/// NoLegPaymentScheduleFixingDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41530")]
	pub leg_payment_schedule_fixing_days: Option<fix_common::RepeatingValues<LegPaymentScheduleFixingDay>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentScheduleFixingDay {
	/// Required if NoLegPaymentScheduleFixingDays(41530) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41531")]
	pub leg_payment_schedule_fixing_day_of_week: Option<LegPaymentScheduleFixingDayOfWeek>,
	/// LegPaymentScheduleFixingDayNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41532")]
	pub leg_payment_schedule_fixing_day_number: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentScheduleFixingDayOfWeek {
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

impl Default for LegPaymentScheduleFixingDayOfWeek {
	fn default() -> Self {
		LegPaymentScheduleFixingDayOfWeek::EveryDay
	}
}
