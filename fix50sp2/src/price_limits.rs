
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceLimits {
	/// Describes the how the price limits are expressed
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1306")]
	pub price_limit_type: Option<PriceLimitType>,
	/// Allowable low limit price for the trading day. A key parameter in validating order price. Used as the lower band for validating
	/// order prices. Orders submitted with prices below the lower limit will be rejected
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1148")]
	pub low_limit_price: Option<f64>,
	/// Allowable high limit price for the trading day. A key parameter in validating order price. Used as the upper band for validating
	/// order prices. Orders submitted with prices above the upper limit will be rejected
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1149")]
	pub high_limit_price: Option<f64>,
	/// Reference price for the current trading price range usually representing the mid price between the HighLimitPrice and LowLimitPrice.
	/// The value may be the settlement price or closing price of the prior trading day.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1150")]
	pub trading_reference_price: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceLimitType {
	/// Price (default)
	#[serde(rename = "0")]
	Price,
	/// Ticks
	#[serde(rename = "1")]
	Ticks,
	/// Percentage
	#[serde(rename = "2")]
	Percentage,
}

impl Default for PriceLimitType {
	fn default() -> Self {
		PriceLimitType::Price
	}
}
