
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendAccrualFloatingRate {
	/// DividendFloatingRateIndex
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42218")]
	pub dividend_floating_rate_index: Option<String>,
	/// Conditionally required when DividendFloatingRateIndexCurveUnit(42220) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42219")]
	pub dividend_floating_rate_index_curve_period: Option<i32>,
	/// Conditionally required when DividendFloatingRateIndexCurvePeriod(42219) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42220")]
	pub dividend_floating_rate_index_curve_unit: Option<String>,
	/// DividendFloatingRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42221")]
	pub dividend_floating_rate_multiplier: Option<f64>,
	/// DividendFloatingRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42222")]
	pub dividend_floating_rate_spread: Option<f64>,
	/// DividendFloatingRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42223")]
	pub dividend_floating_rate_spread_position_type: Option<i32>,
	/// DividendFloatingRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42224")]
	pub dividend_floating_rate_treatment: Option<i32>,
	/// DividendCapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42225")]
	pub dividend_cap_rate: Option<f32>,
	/// DividendCapRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42226")]
	pub dividend_cap_rate_buy_side: Option<i32>,
	/// DividendCapRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42227")]
	pub dividend_cap_rate_sell_side: Option<i32>,
	/// DividendFloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42228")]
	pub dividend_floor_rate: Option<f32>,
	/// DividendFloorRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42229")]
	pub dividend_floor_rate_buy_side: Option<i32>,
	/// DividendFloorRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42230")]
	pub dividend_floor_rate_sell_side: Option<i32>,
	/// DividendInitialRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42231")]
	pub dividend_initial_rate: Option<f32>,
	/// DividendFinalRateRoundingDirection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42232")]
	pub dividend_final_rate_rounding_direction: Option<char>,
	/// DividendFinalRatePrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42233")]
	pub dividend_final_rate_precision: Option<i32>,
	/// DividendAveragingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42234")]
	pub dividend_averaging_method: Option<i32>,
	/// DividendNegativeRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42235")]
	pub dividend_negative_rate_treatment: Option<i32>,
}
