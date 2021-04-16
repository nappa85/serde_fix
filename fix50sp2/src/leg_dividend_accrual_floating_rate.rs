
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegDividendAccrualFloatingRate {
	/// LegDividendFloatingRateIndex
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42312")]
	pub leg_dividend_floating_rate_index: Option<String>,
	/// Conditionally required when LegDividendFloatingRateIndexCurveUnit(42314) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42313")]
	pub leg_dividend_floating_rate_index_curve_period: Option<i32>,
	/// Conditionally required when LegDividendFloatingRateIndexCurvePeriod(42313) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42314")]
	pub leg_dividend_floating_rate_index_curve_unit: Option<String>,
	/// LegDividendFloatingRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42315")]
	pub leg_dividend_floating_rate_multiplier: Option<f64>,
	/// LegDividendFloatingRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42316")]
	pub leg_dividend_floating_rate_spread: Option<f64>,
	/// LegDividendFloatingRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42317")]
	pub leg_dividend_floating_rate_spread_position_type: Option<i32>,
	/// LegDividendFloatingRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42318")]
	pub leg_dividend_floating_rate_treatment: Option<i32>,
	/// LegDividendCapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42319")]
	pub leg_dividend_cap_rate: Option<f32>,
	/// LegDividendCapRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42320")]
	pub leg_dividend_cap_rate_buy_side: Option<i32>,
	/// LegDividendCapRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42321")]
	pub leg_dividend_cap_rate_sell_side: Option<i32>,
	/// LegDividendFloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42322")]
	pub leg_dividend_floor_rate: Option<f32>,
	/// LegDividendFloorRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42323")]
	pub leg_dividend_floor_rate_buy_side: Option<i32>,
	/// LegDividendFloorRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42324")]
	pub leg_dividend_floor_rate_sell_side: Option<i32>,
	/// LegDividendInitialRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42325")]
	pub leg_dividend_initial_rate: Option<f32>,
	/// LegDividendFinalRateRoundingDirection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42326")]
	pub leg_dividend_final_rate_rounding_direction: Option<char>,
	/// LegDividendFinalRatePrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42327")]
	pub leg_dividend_final_rate_precision: Option<i32>,
	/// LegDividendAveragingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42328")]
	pub leg_dividend_averaging_method: Option<i32>,
	/// LegDividendNegativeRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42329")]
	pub leg_dividend_negative_rate_treatment: Option<i32>,
}
