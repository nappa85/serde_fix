
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStubGrp {
	/// NoPaymentStubs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40872")]
	pub payment_stubs: Option<crate::entities::RepeatingValues<PaymentStub>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStub {
	/// Required if NoPaymentStubs(40872) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40873")]
	pub payment_stub_type: Option<PaymentStubType>,
	/// PaymentStubLength
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40874")]
	pub payment_stub_length: Option<PaymentStubLength>,
	/// PaymentStubRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40875")]
	pub payment_stub_rate: Option<f32>,
	/// PaymentStubFixedAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40876")]
	pub payment_stub_fixed_amount: Option<f64>,
	/// PaymentStubFixedCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40877")]
	pub payment_stub_fixed_currency: Option<String>,
	/// PaymentStubIndex
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40878")]
	pub payment_stub_index: Option<String>,
	/// PaymentStubIndexSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40879")]
	pub payment_stub_index_source: Option<i32>,
	/// Conditionally required when PaymentStubIndexCurveUnit(40881) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40880")]
	pub payment_stub_index_curve_period: Option<i32>,
	/// Conditionally required when PaymentStubIndexCurvePeriod(40880) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40881")]
	pub payment_stub_index_curve_unit: Option<String>,
	/// PaymentStubIndexRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40882")]
	pub payment_stub_index_rate_multiplier: Option<f64>,
	/// PaymentStubIndexRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40883")]
	pub payment_stub_index_rate_spread: Option<f64>,
	/// PaymentStubIndexRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40884")]
	pub payment_stub_index_rate_spread_position_type: Option<i32>,
	/// PaymentStubIndexRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40885")]
	pub payment_stub_index_rate_treatment: Option<i32>,
	/// PaymentStubIndexCapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40886")]
	pub payment_stub_index_cap_rate: Option<f32>,
	/// PaymentStubIndexCapRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40887")]
	pub payment_stub_index_cap_rate_buy_side: Option<i32>,
	/// PaymentStubIndexCapRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40888")]
	pub payment_stub_index_cap_rate_sell_side: Option<i32>,
	/// PaymentStubIndexFloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40889")]
	pub payment_stub_index_floor_rate: Option<f32>,
	/// PaymentStubIndexFloorRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40890")]
	pub payment_stub_index_floor_rate_buy_side: Option<i32>,
	/// PaymentStubIndexFloorRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40891")]
	pub payment_stub_index_floor_rate_sell_side: Option<i32>,
	/// PaymentStubIndex2
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40892")]
	pub payment_stub_index_2: Option<String>,
	/// PaymentStubIndex2Source
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40893")]
	pub payment_stub_index_2_source: Option<i32>,
	/// Conditionally required when PaymentStubIndex2CurveUnit(40895) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40894")]
	pub payment_stub_index_2_curve_period: Option<i32>,
	/// Conditionally required when PaymentStubIndex2CurvePeriod(40894) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40895")]
	pub payment_stub_index_2_curve_unit: Option<String>,
	/// PaymentStubIndex2RateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40896")]
	pub payment_stub_index_2_rate_multiplier: Option<f64>,
	/// PaymentStubIndex2RateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40897")]
	pub payment_stub_index_2_rate_spread: Option<f64>,
	/// PaymentStubIndex2RateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40898")]
	pub payment_stub_index_2_rate_spread_position_type: Option<i32>,
	/// PaymentStubIndex2RateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40899")]
	pub payment_stub_index_2_rate_treatment: Option<i32>,
	/// PaymentStubIndex2CapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40900")]
	pub payment_stub_index_2_cap_rate: Option<f32>,
	/// PaymentStubIndex2FloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40901")]
	pub payment_stub_index_2_floor_rate: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStubType {
	/// Initial
	#[serde(rename = "0")]
	Initial,
	/// Final
	#[serde(rename = "1")]
	Final,
	/// Compounding initial
	#[serde(rename = "2")]
	CompoundingInitial,
	/// Compounding final
	#[serde(rename = "3")]
	CompoundingFinal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PaymentStubLength {
	/// Short
	#[serde(rename = "0")]
	Short,
	/// Long
	#[serde(rename = "1")]
	Long,
}
