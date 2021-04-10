
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamCompoundingFloatingRate {
	/// UnderlyingPaymentStreamCompoundingRateIndex
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42923")]
	pub underlying_payment_stream_compounding_rate_index: Option<String>,
	/// Conditionally required if UnderlyingPaymentStreamCompoundingRateIndexCurveUnit(42925) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42924")]
	pub underlying_payment_stream_compounding_rate_index_curve_period: Option<i32>,
	/// Conditionally required if UnderlyingPaymentStreamCompoundingRateIndexCurvePeriod(42924) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42925")]
	pub underlying_payment_stream_compounding_rate_index_curve_unit: Option<String>,
	/// UnderlyingPaymentStreamCompoundingRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42926")]
	pub underlying_payment_stream_compounding_rate_multiplier: Option<f64>,
	/// UnderlyingPaymentStreamCompoundingRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42927")]
	pub underlying_payment_stream_compounding_rate_spread: Option<f64>,
	/// UnderlyingPaymentStreamCompoundingRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42928")]
	pub underlying_payment_stream_compounding_rate_spread_position_type: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42929")]
	pub underlying_payment_stream_compounding_rate_treatment: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingCapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42930")]
	pub underlying_payment_stream_compounding_cap_rate: Option<f32>,
	/// UnderlyingPaymentStreamCompoundingCapRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42931")]
	pub underlying_payment_stream_compounding_cap_rate_buy_side: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingCapRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42932")]
	pub underlying_payment_stream_compounding_cap_rate_sell_side: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingFloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42933")]
	pub underlying_payment_stream_compounding_floor_rate: Option<f32>,
	/// UnderlyingPaymentStreamCompoundingFloorRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42934")]
	pub underlying_payment_stream_compounding_floor_rate_buy_side: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingFloorRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42935")]
	pub underlying_payment_stream_compounding_floor_rate_sell_side: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingInitialRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42936")]
	pub underlying_payment_stream_compounding_initial_rate: Option<f32>,
	/// UnderlyingPaymentStreamCompoundingFinalRateRoundingDirection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42937")]
	pub underlying_payment_stream_compounding_final_rate_rounding_direction: Option<char>,
	/// UnderlyingPaymentStreamCompoundingFinalRatePrecision
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42938")]
	pub underlying_payment_stream_compounding_final_rate_precision: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingAveragingMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42939")]
	pub underlying_payment_stream_compounding_averaging_method: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingNegativeRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42940")]
	pub underlying_payment_stream_compounding_negative_rate_treatment: Option<i32>,
}
