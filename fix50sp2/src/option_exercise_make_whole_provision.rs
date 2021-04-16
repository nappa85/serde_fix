
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseMakeWholeProvision {
	/// MakeWholeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42591")]
	pub make_whole_date: Option<fix_common::LocalMktDate>,
	/// MakeWholeAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42592")]
	pub make_whole_amount: Option<f64>,
	/// MakeWholeBenchmarkCurveName
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42593")]
	pub make_whole_benchmark_curve_name: Option<String>,
	/// MakeWholeBenchmarkCurvePoint
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42594")]
	pub make_whole_benchmark_curve_point: Option<String>,
	/// MakeWholeRecallSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42595")]
	pub make_whole_recall_spread: Option<f64>,
	/// MakeWholeBenchmarkQuote
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42596")]
	pub make_whole_benchmark_quote: Option<i32>,
	/// MakeWholeInterpolationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42597")]
	pub make_whole_interpolation_method: Option<i32>,
}
