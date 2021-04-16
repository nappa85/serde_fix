
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FloatingRateIndex {
	/// Conditionally required when FloatingRateIndexIDSource(2732) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2731")]
	pub floating_rate_index_id: Option<String>,
	/// Conditionally required when FloatingRateIndexID(2731) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2732")]
	pub floating_rate_index_id_source: Option<String>,
	/// Conditionally required when FloatingRateIndexCurvePeriod(2728) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2730")]
	pub floating_rate_index_curve_unit: Option<String>,
	/// Conditionally required when FloatingRateIndexCurveUnit(2730) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2728")]
	pub floating_rate_index_curve_period: Option<i32>,
	/// FloatingRateIndexCurveSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2729")]
	pub floating_rate_index_curve_spread: Option<f64>,
}
