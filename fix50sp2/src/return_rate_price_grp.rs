
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRatePriceGrp {
	/// NoReturnRatePrices
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42765")]
	pub return_rate_prices: Option<fix_common::RepeatingValues<ReturnRatePrice>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReturnRatePrice {
	/// Required if NoReturnRatePrices(42765) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42766")]
	pub return_rate_price_basis: Option<ReturnRatePriceBasis>,
	/// ReturnRatePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42767")]
	pub return_rate_price: Option<f64>,
	/// ReturnRatePriceCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42768")]
	pub return_rate_price_currency: Option<String>,
	/// ReturnRatePriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42769")]
	pub return_rate_price_type: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ReturnRatePriceBasis {
	/// Gross
	#[serde(rename = "0")]
	Gross,
	/// Net
	#[serde(rename = "1")]
	Net,
	/// Accrued
	#[serde(rename = "2")]
	Accrued,
	/// Clean net
	#[serde(rename = "3")]
	CleanNet,
}

impl Default for ReturnRatePriceBasis {
	fn default() -> Self {
		ReturnRatePriceBasis::Gross
	}
}
