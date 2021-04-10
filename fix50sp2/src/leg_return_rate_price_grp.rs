
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRatePriceGrp {
	/// NoLegReturnRatePrices
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42564")]
	pub leg_return_rate_prices: Option<crate::entities::RepeatingValues<LegReturnRatePrice>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRatePrice {
	/// Required if NoLegReturnRatePrices(42564) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42565")]
	pub leg_return_rate_price_basis: Option<i32>,
	/// LegReturnRatePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42566")]
	pub leg_return_rate_price: Option<f64>,
	/// LegReturnRatePriceCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42567")]
	pub leg_return_rate_price_currency: Option<String>,
	/// LegReturnRatePriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42568")]
	pub leg_return_rate_price_type: Option<i32>,
}
