
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCommoditySettlDayGrp {
	/// NoStreamCommoditySettlDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41283")]
	pub stream_commodity_settl_days: Option<fix_common::RepeatingValues<StreamCommoditySettlDay>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamCommoditySettlDay {
	/// Required if NoStreamCommoditySettlDays(41283) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41284")]
	pub stream_commodity_settl_day_item: Option<StreamCommoditySettlDayItem>,
	/// StreamCommoditySettlTotalHours
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41285")]
	pub stream_commodity_settl_total_hours: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StreamCommoditySettlDayItem {
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

impl Default for StreamCommoditySettlDayItem {
	fn default() -> Self {
		StreamCommoditySettlDayItem::Monday
	}
}
