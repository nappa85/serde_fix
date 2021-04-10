
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRatePriceGrp {
	/// NoUnderlyingReturnRatePrices
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43064")]
	pub underlying_return_rate_prices: Option<crate::entities::RepeatingValues<UnderlyingReturnRatePrice>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRatePrice {
	/// Required if NoUnderlyingReturnRatePrices(43064) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43065")]
	pub underlying_return_rate_price_basis: Option<i32>,
	/// UnderlyingReturnRatePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43066")]
	pub underlying_return_rate_price: Option<f64>,
	/// UnderlyingReturnRatePriceCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43067")]
	pub underlying_return_rate_price_currency: Option<String>,
	/// UnderlyingReturnRatePriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43068")]
	pub underlying_return_rate_price_type: Option<i32>,
}
