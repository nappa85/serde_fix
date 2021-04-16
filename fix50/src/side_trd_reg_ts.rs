
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideTrdRegTS {
	/// NoSideTrdRegTS
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1016")]
	pub side_trd_reg_ts: Option<fix_common::RepeatingValues<SideTrdRegT>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideTrdRegT {
	/// SideTrdRegTimestamp
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1012")]
	pub side_trd_reg_timestamp: Option<fix_common::UTCTimestamp>,
	/// SideTrdRegTimestampType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1013")]
	pub side_trd_reg_timestamp_type: Option<i32>,
	/// SideTrdRegTimestampSrc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1014")]
	pub side_trd_reg_timestamp_src: Option<String>,
}
