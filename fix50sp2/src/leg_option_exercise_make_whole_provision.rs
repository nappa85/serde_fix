
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseMakeWholeProvision {
	/// LegMakeWholeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42392")]
	pub leg_make_whole_date: Option<crate::entities::LocalMktDate>,
	/// LegMakeWholeAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42393")]
	pub leg_make_whole_amount: Option<f64>,
	/// LegMakeWholeBenchmarkCurveName
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42394")]
	pub leg_make_whole_benchmark_curve_name: Option<String>,
	/// LegMakeWholeBenchmarkCurvePoint
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42395")]
	pub leg_make_whole_benchmark_curve_point: Option<String>,
	/// LegMakeWholeRecallSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42396")]
	pub leg_make_whole_recall_spread: Option<f64>,
	/// LegMakeWholeBenchmarkQuote
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42397")]
	pub leg_make_whole_benchmark_quote: Option<i32>,
	/// LegMakeWholeInterpolationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42398")]
	pub leg_make_whole_interpolation_method: Option<i32>,
}
