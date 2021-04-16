
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutionAggregationGrp {
	/// NoExecs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "124")]
	pub execs: Option<fix_common::RepeatingValues<Exec>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Exec {
	/// Required if NoExecs(124) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "32")]
	pub last_qty: Option<f64>,
	/// Either <a href="tag_17_ExecID.html" target="bottom">ExecID(17)&nbsp;(17)</a> or <a href="tag_1003_TradeID.html" target="bottom">TradeID(1003)&nbsp;(1003)</a> must be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "17")]
	pub exec_id: Option<String>,
	/// Either <a href="tag_17_ExecID.html" target="bottom">ExecID(17)&nbsp;(17)</a> or <a href="tag_1003_TradeID.html" target="bottom">TradeID(1003)&nbsp;(1003)</a> must be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1003")]
	pub trade_id: Option<String>,
	/// LastPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
}
