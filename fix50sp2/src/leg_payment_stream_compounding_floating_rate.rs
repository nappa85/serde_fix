
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamCompoundingFloatingRate {
	/// LegPaymentStreamCompoundingRateIndex
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42427")]
	pub leg_payment_stream_compounding_rate_index: Option<String>,
	/// Conditionally required if LegPaymentStreamCompoundingRateIndexCurveUnit(42429) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42428")]
	pub leg_payment_stream_compounding_rate_index_curve_period: Option<i32>,
	/// Conditionally required if LegPaymentStreamCompoundingRateIndexCurvePeriod(42428) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42429")]
	pub leg_payment_stream_compounding_rate_index_curve_unit: Option<String>,
	/// LegPaymentStreamCompoundingRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42430")]
	pub leg_payment_stream_compounding_rate_multiplier: Option<f64>,
	/// LegPaymentStreamCompoundingRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42431")]
	pub leg_payment_stream_compounding_rate_spread: Option<f64>,
	/// LegPaymentStreamCompoundingRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42432")]
	pub leg_payment_stream_compounding_rate_spread_position_type: Option<i32>,
	/// LegPaymentStreamCompoundingRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42433")]
	pub leg_payment_stream_compounding_rate_treatment: Option<i32>,
	/// LegPaymentStreamCompoundingCapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42434")]
	pub leg_payment_stream_compounding_cap_rate: Option<f32>,
	/// LegPaymentStreamCompoundingCapRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42435")]
	pub leg_payment_stream_compounding_cap_rate_buy_side: Option<i32>,
	/// LegPaymentStreamCompoundingCapRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42436")]
	pub leg_payment_stream_compounding_cap_rate_sell_side: Option<i32>,
	/// LegPaymentStreamCompoundingFloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42437")]
	pub leg_payment_stream_compounding_floor_rate: Option<f32>,
	/// LegPaymentStreamCompoundingFloorRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42438")]
	pub leg_payment_stream_compounding_floor_rate_buy_side: Option<i32>,
	/// LegPaymentStreamCompoundingFloorRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42439")]
	pub leg_payment_stream_compounding_floor_rate_sell_side: Option<i32>,
	/// LegPaymentStreamCompoundingInitialRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42440")]
	pub leg_payment_stream_compounding_initial_rate: Option<f32>,
	/// LegPaymentStreamCompoundingFinalRateRoundingDirection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42441")]
	pub leg_payment_stream_compounding_final_rate_rounding_direction: Option<char>,
	/// LegPaymentStreamCompoundingFinalRatePrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42442")]
	pub leg_payment_stream_compounding_final_rate_precision: Option<i32>,
	/// LegPaymentStreamCompoundingAveragingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42443")]
	pub leg_payment_stream_compounding_averaging_method: Option<i32>,
	/// LegPaymentStreamCompoundingNegativeRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42444")]
	pub leg_payment_stream_compounding_negative_rate_treatment: Option<i32>,
}
