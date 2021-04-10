
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStubGrp {
	/// NoUnderlyingPaymentStubs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40708")]
	pub underlying_payment_stubs: Option<fix_common::RepeatingValues<UnderlyingPaymentStub>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStub {
	/// Required if NoUnderlyingPaymentStubs(40708) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40709")]
	pub underlying_payment_stub_type: Option<i32>,
	/// UnderlyingPaymentStubLength
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40710")]
	pub underlying_payment_stub_length: Option<i32>,
	/// UnderlyingPaymentStubRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40711")]
	pub underlying_payment_stub_rate: Option<f32>,
	/// UnderlyingPaymentStubFixedAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40712")]
	pub underlying_payment_stub_fixed_amount: Option<f64>,
	/// UnderlyingPaymentStubFixedCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40713")]
	pub underlying_payment_stub_fixed_currency: Option<String>,
	/// UnderlyingPaymentStubIndex
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40714")]
	pub underlying_payment_stub_index: Option<String>,
	/// UnderlyingPaymentStubIndexSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40715")]
	pub underlying_payment_stub_index_source: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStubIndexCurveUnit(40717) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40716")]
	pub underlying_payment_stub_index_curve_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStubIndexCurvePeriod(40716) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40717")]
	pub underlying_payment_stub_index_curve_unit: Option<String>,
	/// UnderlyingPaymentStubIndexRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40718")]
	pub underlying_payment_stub_index_rate_multiplier: Option<f64>,
	/// UnderlyingPaymentStubIndexRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40719")]
	pub underlying_payment_stub_index_rate_spread: Option<f64>,
	/// UnderlyingPaymentStubIndexRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40720")]
	pub underlying_payment_stub_index_rate_spread_position_type: Option<i32>,
	/// UnderlyingPaymentStubIndexRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40721")]
	pub underlying_payment_stub_index_rate_treatment: Option<i32>,
	/// UnderlyingPaymentStubIndexCapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40722")]
	pub underlying_payment_stub_index_cap_rate: Option<f32>,
	/// UnderlyingPaymentStubIndexCapRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40723")]
	pub underlying_payment_stub_index_cap_rate_buy_side: Option<i32>,
	/// UnderlyingPaymentStubIndexCapRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40724")]
	pub underlying_payment_stub_index_cap_rate_sell_side: Option<i32>,
	/// UnderlyingPaymentStubIndexFloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40725")]
	pub underlying_payment_stub_index_floor_rate: Option<f32>,
	/// UnderlyingPaymentStubIndexFloorRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40726")]
	pub underlying_payment_stub_index_floor_rate_buy_side: Option<i32>,
	/// UnderlyingPaymentStubIndexFloorRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40727")]
	pub underlying_payment_stub_index_floor_rate_sell_side: Option<i32>,
	/// UnderlyingPaymentStubIndex2
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40728")]
	pub underlying_payment_stub_index_2: Option<String>,
	/// UnderlyingPaymentStubIndex2Source
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40729")]
	pub underlying_payment_stub_index_2_source: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStubIndex2CurveUnit(40731) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40730")]
	pub underlying_payment_stub_index_2_curve_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStubIndex2CurvePeriod(40730) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40731")]
	pub underlying_payment_stub_index_2_curve_unit: Option<String>,
	/// UnderlyingPaymentStubIndex2RateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40732")]
	pub underlying_payment_stub_index_2_rate_multiplier: Option<f64>,
	/// UnderlyingPaymentStubIndex2RateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40733")]
	pub underlying_payment_stub_index_2_rate_spread: Option<f64>,
	/// UnderlyingPaymentStubIndex2RateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40734")]
	pub underlying_payment_stub_index_2_rate_spread_position_type: Option<i32>,
	/// UnderlyingPaymentStubIndex2RateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40735")]
	pub underlying_payment_stub_index_2_rate_treatment: Option<i32>,
	/// UnderlyingPaymentStubIndex2CapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40736")]
	pub underlying_payment_stub_index_2_cap_rate: Option<f32>,
	/// UnderlyingPaymentStubIndex2FloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40737")]
	pub underlying_payment_stub_index_2_floor_rate: Option<f32>,
}
