
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommoditySettlDayGrp {
	/// NoUnderlyingStreamCommoditySettlDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41996")]
	pub underlying_stream_commodity_settl_days: Option<fix_common::RepeatingValues<UnderlyingStreamCommoditySettlDay>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingStreamCommoditySettlDay {
	/// Required if NoUnderlyingStreamCommoditySettlDays(41996) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41997")]
	pub underlying_stream_commodity_settl_day_item: Option<UnderlyingStreamCommoditySettlDayItem>,
	/// UnderlyingStreamCommoditySettlTotalHours
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41998")]
	pub underlying_stream_commodity_settl_total_hours: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingStreamCommoditySettlDayItem {
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

impl Default for UnderlyingStreamCommoditySettlDayItem {
	fn default() -> Self {
		UnderlyingStreamCommoditySettlDayItem::Monday
	}
}
