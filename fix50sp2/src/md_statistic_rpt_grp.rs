
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDStatisticRptGrp {
	/// NoMDStatistics
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2474")]
	pub md_statistics: Option<fix_common::RepeatingValues<MDStatistic>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDStatistic {
	/// Required if <a href="tag_2474_NoMDStatistics.html" target="bottom">NoMDStatistics (2474)&nbsp;(2474)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2475")]
	pub md_statistic_id: Option<String>,
	/// Required if <a href="tag_2474_NoMDStatistics.html" target="bottom">NoMDStatistics (2474)&nbsp;(2474)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2476")]
	pub md_statistic_time: Option<fix_common::UTCTimestamp>,
	/// Required if <a href="tag_2474_NoMDStatistics.html" target="bottom">NoMDStatistics (2474)&nbsp;(2474)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2477")]
	pub md_statistic_status: Option<MDStatisticStatus>,
	/// Calculated statistic value. Conditionally required unless sending reference data only to establish MDStatisticID(2475) as
	/// a shortcut to a set parameters given by the MDStatsParameters component.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2478")]
	pub md_statistic_value: Option<f64>,
	/// MDStatisticValueType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2479")]
	pub md_statistic_value_type: Option<MDStatisticValueType>,
	/// MDStatisticValueUnit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2480")]
	pub md_statistic_value_unit: Option<MDStatisticValueUnit>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDStatisticStatus {
	/// Active (default)
	#[serde(rename = "1")]
	Active,
	/// Inactive (not disseminated)
	#[serde(rename = "2")]
	Inactive,
}

impl Default for MDStatisticStatus {
	fn default() -> Self {
		MDStatisticStatus::Active
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDStatisticValueType {
	/// Absolute
	#[serde(rename = "1")]
	Absolute,
	/// Percentage
	#[serde(rename = "2")]
	Percentage,
}

impl Default for MDStatisticValueType {
	fn default() -> Self {
		MDStatisticValueType::Absolute
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDStatisticValueUnit {
	/// Seconds (default if not specified)
	#[serde(rename = "0")]
	Seconds,
	/// Tenths of a second
	#[serde(rename = "1")]
	TenthsOfASecond,
	/// Hundredths of a second
	#[serde(rename = "2")]
	HundredthsOfASecond,
	/// milliseconds
	#[serde(rename = "3")]
	Milliseconds,
	/// microseconds
	#[serde(rename = "4")]
	Microseconds,
	/// nanoseconds
	#[serde(rename = "5")]
	Nanoseconds,
	/// minutes
	#[serde(rename = "10")]
	Minutes,
	/// hours
	#[serde(rename = "11")]
	Hours,
	/// days
	#[serde(rename = "12")]
	Days,
	/// weeks
	#[serde(rename = "13")]
	Weeks,
	/// months
	#[serde(rename = "14")]
	Months,
	/// years
	#[serde(rename = "15")]
	Years,
}

impl Default for MDStatisticValueUnit {
	fn default() -> Self {
		MDStatisticValueUnit::Seconds
	}
}
