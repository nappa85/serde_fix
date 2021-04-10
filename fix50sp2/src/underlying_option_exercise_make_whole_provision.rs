
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseMakeWholeProvision {
	/// UnderlyingMakeWholeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42888")]
	pub underlying_make_whole_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingMakeWholeAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42889")]
	pub underlying_make_whole_amount: Option<f64>,
	/// UnderlyingMakeWholeBenchmarkCurveName
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42890")]
	pub underlying_make_whole_benchmark_curve_name: Option<String>,
	/// UnderlyingMakeWholeBenchmarkCurvePoint
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42891")]
	pub underlying_make_whole_benchmark_curve_point: Option<String>,
	/// UnderlyingMakeWholeRecallSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42892")]
	pub underlying_make_whole_recall_spread: Option<f64>,
	/// UnderlyingMakeWholeBenchmarkQuote
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42893")]
	pub underlying_make_whole_benchmark_quote: Option<i32>,
	/// UnderlyingMakeWholeInterpolationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42894")]
	pub underlying_make_whole_interpolation_method: Option<i32>,
}
