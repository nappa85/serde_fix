
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingDividendAccrualFloatingRate {
	/// UnderlyingDividendFloatingRateIndex
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42801")]
	pub underlying_dividend_floating_rate_index: Option<String>,
	/// Conditionally required when UnderlyingDividendFloatingRateIndexCurveUnit(42803) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42802")]
	pub underlying_dividend_floating_rate_index_curve_period: Option<i32>,
	/// Conditionally required when UnderlyingDividendFloatingRateIndexCurvePeriod(42802) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42803")]
	pub underlying_dividend_floating_rate_index_curve_unit: Option<String>,
	/// UnderlyingDividendFloatingRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42804")]
	pub underlying_dividend_floating_rate_multiplier: Option<f64>,
	/// UnderlyingDividendFloatingRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42805")]
	pub underlying_dividend_floating_rate_spread: Option<f64>,
	/// UnderlyingDividendFloatingRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42806")]
	pub underlying_dividend_floating_rate_spread_position_type: Option<i32>,
	/// UnderlyingDividendFloatingRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42807")]
	pub underlying_dividend_floating_rate_treatment: Option<i32>,
	/// UnderlyingDividendCapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42808")]
	pub underlying_dividend_cap_rate: Option<f32>,
	/// UnderlyingDividendCapRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42809")]
	pub underlying_dividend_cap_rate_buy_side: Option<i32>,
	/// UnderlyingDividendCapRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42810")]
	pub underlying_dividend_cap_rate_sell_side: Option<i32>,
	/// UnderlyingDividendFloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42811")]
	pub underlying_dividend_floor_rate: Option<f32>,
	/// UnderlyingDividendFloorRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42812")]
	pub underlying_dividend_floor_rate_buy_side: Option<i32>,
	/// UnderlyingDividendFloorRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42813")]
	pub underlying_dividend_floor_rate_sell_side: Option<i32>,
	/// UnderlyingDividendInitialRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42814")]
	pub underlying_dividend_initial_rate: Option<f32>,
	/// UnderlyingDividendFinalRateRoundingDirection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42815")]
	pub underlying_dividend_final_rate_rounding_direction: Option<char>,
	/// UnderlyingDividendFinalRatePrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42816")]
	pub underlying_dividend_final_rate_precision: Option<i32>,
	/// UnderlyingDividendAveragingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42817")]
	pub underlying_dividend_averaging_method: Option<i32>,
	/// UnderlyingDividendNegativeRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42818")]
	pub underlying_dividend_negative_rate_treatment: Option<i32>,
}
