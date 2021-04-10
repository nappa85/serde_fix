
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamCompoundingFloatingRate {
	/// PaymentStreamCompoundingRateIndex
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42628")]
	pub payment_stream_compounding_rate_index: Option<String>,
	/// Conditionally required if PaymentStreamCompoundingRateIndexCurveUnit(42630) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42629")]
	pub payment_stream_compounding_rate_index_curve_period: Option<i32>,
	/// Conditionally required if PaymentStreamCompoundingRateIndexCurvePeriod(42629) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42630")]
	pub payment_stream_compounding_rate_index_curve_unit: Option<String>,
	/// PaymentStreamCompoundingRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42631")]
	pub payment_stream_compounding_rate_multiplier: Option<f64>,
	/// PaymentStreamCompoundingRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42632")]
	pub payment_stream_compounding_rate_spread: Option<f64>,
	/// PaymentStreamCompoundingRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42633")]
	pub payment_stream_compounding_rate_spread_position_type: Option<i32>,
	/// PaymentStreamCompoundingRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42634")]
	pub payment_stream_compounding_rate_treatment: Option<i32>,
	/// PaymentStreamCompoundingCapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42635")]
	pub payment_stream_compounding_cap_rate: Option<f32>,
	/// PaymentStreamCompoundingCapRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42636")]
	pub payment_stream_compounding_cap_rate_buy_side: Option<i32>,
	/// PaymentStreamCompoundingCapRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42637")]
	pub payment_stream_compounding_cap_rate_sell_side: Option<i32>,
	/// PaymentStreamCompoundingFloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42638")]
	pub payment_stream_compounding_floor_rate: Option<f32>,
	/// PaymentStreamCompoundingFloorRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42639")]
	pub payment_stream_compounding_floor_rate_buy_side: Option<i32>,
	/// PaymentStreamCompoundingFloorRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42640")]
	pub payment_stream_compounding_floor_rate_sell_side: Option<i32>,
	/// PaymentStreamCompoundingInitialRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42641")]
	pub payment_stream_compounding_initial_rate: Option<f32>,
	/// PaymentStreamCompoundingFinalRateRoundingDirection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42642")]
	pub payment_stream_compounding_final_rate_rounding_direction: Option<char>,
	/// PaymentStreamCompoundingFinalRatePrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42643")]
	pub payment_stream_compounding_final_rate_precision: Option<i32>,
	/// PaymentStreamCompoundingAveragingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42644")]
	pub payment_stream_compounding_averaging_method: Option<i32>,
	/// PaymentStreamCompoundingNegativeRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42645")]
	pub payment_stream_compounding_negative_rate_treatment: Option<i32>,
}
