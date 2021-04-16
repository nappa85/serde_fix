
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDeliveryScheduleSettlDayGrp {
	/// NoLegDeliveryScheduleSettlDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41422")]
	pub leg_delivery_schedule_settl_days: Option<fix_common::RepeatingValues<LegDeliveryScheduleSettlDay>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDeliveryScheduleSettlDay {
	/// Required if NoLegDeliveryScheduleSettlDays(41422) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41423")]
	pub leg_delivery_schedule_settl_day_item: Option<LegDeliveryScheduleSettlDayItem>,
	/// LegDeliveryScheduleSettlTotalHours
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41424")]
	pub leg_delivery_schedule_settl_total_hours: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegDeliveryScheduleSettlDayItem {
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
	/// All weekdays
	#[serde(rename = "8")]
	AllWeekdays,
	/// All days
	#[serde(rename = "9")]
	AllDays,
	/// All weekends
	#[serde(rename = "10")]
	AllWeekends,
}

impl Default for LegDeliveryScheduleSettlDayItem {
	fn default() -> Self {
		LegDeliveryScheduleSettlDayItem::Monday
	}
}
