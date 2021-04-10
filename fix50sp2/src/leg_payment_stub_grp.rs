
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStubGrp {
	/// NoLegPaymentStubs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40418")]
	pub leg_payment_stubs: Option<crate::entities::RepeatingValues<LegPaymentStub>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStub {
	/// Required if NoLegPaymentStubs(40418) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40419")]
	pub leg_payment_stub_type: Option<LegPaymentStubType>,
	/// LegPaymentStubLength
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40420")]
	pub leg_payment_stub_length: Option<LegPaymentStubLength>,
	/// LegPaymentStubRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40421")]
	pub leg_payment_stub_rate: Option<f32>,
	/// LegPaymentStubFixedAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40422")]
	pub leg_payment_stub_fixed_amount: Option<f64>,
	/// LegPaymentStubFixedCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40423")]
	pub leg_payment_stub_fixed_currency: Option<String>,
	/// LegPaymentStubIndex
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40424")]
	pub leg_payment_stub_index: Option<String>,
	/// LegPaymentStubIndexSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40425")]
	pub leg_payment_stub_index_source: Option<LegPaymentStubIndexSource>,
	/// Conditionally required when LegPaymentStubIndexCurveUnit(40427) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40426")]
	pub leg_payment_stub_index_curve_period: Option<i32>,
	/// Conditionally required when LegPaymentStubIndexCurvePeriod(40426) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40427")]
	pub leg_payment_stub_index_curve_unit: Option<LegPaymentStubIndexCurveUnit>,
	/// LegPaymentStubIndexRateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40428")]
	pub leg_payment_stub_index_rate_multiplier: Option<f64>,
	/// LegPaymentStubIndexRateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40429")]
	pub leg_payment_stub_index_rate_spread: Option<f64>,
	/// LegPaymentStubIndexRateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40430")]
	pub leg_payment_stub_index_rate_spread_position_type: Option<LegPaymentStubIndexRateSpreadPositionType>,
	/// LegPaymentStubIndexRateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40431")]
	pub leg_payment_stub_index_rate_treatment: Option<LegPaymentStubIndexRateTreatment>,
	/// LegPaymentStubIndexCapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40432")]
	pub leg_payment_stub_index_cap_rate: Option<f32>,
	/// LegPaymentStubIndexCapRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40433")]
	pub leg_payment_stub_index_cap_rate_buy_side: Option<LegPaymentStubIndexCapRateBuySide>,
	/// LegPaymentStubIndexCapRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40434")]
	pub leg_payment_stub_index_cap_rate_sell_side: Option<LegPaymentStubIndexCapRateSellSide>,
	/// LegPaymentStubIndexFloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40435")]
	pub leg_payment_stub_index_floor_rate: Option<f32>,
	/// LegPaymentStubIndexFloorRateBuySide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40436")]
	pub leg_payment_stub_index_floor_rate_buy_side: Option<LegPaymentStubIndexFloorRateBuySide>,
	/// LegPaymentStubIndexFloorRateSellSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40437")]
	pub leg_payment_stub_index_floor_rate_sell_side: Option<LegPaymentStubIndexFloorRateSellSide>,
	/// LegPaymentStubIndex2
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40438")]
	pub leg_payment_stub_index_2: Option<String>,
	/// LegPaymentStubIndex2Source
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40439")]
	pub leg_payment_stub_index_2_source: Option<i32>,
	/// Conditionally required when LegPaymentStubIndex2CurveUnit(40441) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40440")]
	pub leg_payment_stub_index_2_curve_period: Option<i32>,
	/// Conditionally required when LegPaymentStubIndex2CurvePeriod(40440) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40441")]
	pub leg_payment_stub_index_2_curve_unit: Option<LegPaymentStubIndex2CurveUnit>,
	/// LegPaymentStubIndex2RateMultiplier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40442")]
	pub leg_payment_stub_index_2_rate_multiplier: Option<f64>,
	/// LegPaymentStubIndex2RateSpread
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40443")]
	pub leg_payment_stub_index_2_rate_spread: Option<f64>,
	/// LegPaymentStubIndex2RateSpreadPositionType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40444")]
	pub leg_payment_stub_index_2_rate_spread_position_type: Option<LegPaymentStubIndex2RateSpreadPositionType>,
	/// LegPaymentStubIndex2RateTreatment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40445")]
	pub leg_payment_stub_index_2_rate_treatment: Option<LegPaymentStubIndex2RateTreatment>,
	/// LegPaymentStubIndex2CapRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40446")]
	pub leg_payment_stub_index_2_cap_rate: Option<f32>,
	/// LegPaymentStubIndex2FloorRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40447")]
	pub leg_payment_stub_index_2_floor_rate: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubType {
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
pub enum LegPaymentStubLength {
	/// Short
	#[serde(rename = "0")]
	Short,
	/// Long
	#[serde(rename = "1")]
	Long,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndexSource {
	/// Bloomberg
	#[serde(rename = "0")]
	Bloomberg,
	/// Reuters
	#[serde(rename = "1")]
	Reuters,
	/// Telerate
	#[serde(rename = "2")]
	Telerate,
	/// Other
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndexCurveUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndexRateSpreadPositionType {
	/// Short
	#[serde(rename = "0")]
	Short,
	/// Long
	#[serde(rename = "1")]
	Long,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndexRateTreatment {
	/// Bond equivalent yield
	#[serde(rename = "0")]
	BondEquivalentYield,
	/// Money market yield
	#[serde(rename = "1")]
	MoneyMarketYield,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndexCapRateBuySide {
	/// Buyer of the trade
	#[serde(rename = "1")]
	BuyerOfTheTrade,
	/// Seller of the trade
	#[serde(rename = "2")]
	SellerOfTheTrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndexCapRateSellSide {
	/// Buyer of the trade
	#[serde(rename = "1")]
	BuyerOfTheTrade,
	/// Seller of the trade
	#[serde(rename = "2")]
	SellerOfTheTrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndexFloorRateBuySide {
	/// Buyer of the trade
	#[serde(rename = "1")]
	BuyerOfTheTrade,
	/// Seller of the trade
	#[serde(rename = "2")]
	SellerOfTheTrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndexFloorRateSellSide {
	/// Buyer of the trade
	#[serde(rename = "1")]
	BuyerOfTheTrade,
	/// Seller of the trade
	#[serde(rename = "2")]
	SellerOfTheTrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndex2CurveUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndex2RateSpreadPositionType {
	/// Short
	#[serde(rename = "0")]
	Short,
	/// Long
	#[serde(rename = "1")]
	Long,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegPaymentStubIndex2RateTreatment {
	/// Bond equivalent yield
	#[serde(rename = "0")]
	BondEquivalentYield,
	/// Money market yield
	#[serde(rename = "1")]
	MoneyMarketYield,
}
