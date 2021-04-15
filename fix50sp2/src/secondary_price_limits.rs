
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecondaryPriceLimits {
	/// SecondaryPriceLimitType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1305")]
	pub secondary_price_limit_type: Option<SecondaryPriceLimitType>,
	/// SecondaryLowLimitPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1221")]
	pub secondary_low_limit_price: Option<f64>,
	/// SecondaryHighLimitPrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1230")]
	pub secondary_high_limit_price: Option<f64>,
	/// SecondaryTradingReferencePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1240")]
	pub secondary_trading_reference_price: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecondaryPriceLimitType {
	/// Price
	#[serde(rename = "0")]
	Price,
	/// Ticks
	#[serde(rename = "1")]
	Ticks,
	/// Percentage
	#[serde(rename = "2")]
	Percentage,
}

impl Default for SecondaryPriceLimitType {
	fn default() -> Self {
		SecondaryPriceLimitType::Price
	}
}
