
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteSizeRuleGrp {
	/// NoQuoteSizeRules
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2558")]
	pub quote_size_rules: Option<fix_common::RepeatingValues<QuoteSizeRule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteSizeRule {
	/// Required if NoQuoteSizeRules(2558) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "647")]
	pub min_bid_size: Option<f64>,
	/// Required if NoQuoteSizeRules(2558) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "648")]
	pub min_offer_size: Option<f64>,
	/// Used to define the sizes applicable for fast market conditions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2447")]
	pub fast_market_indicator: Option<fix_common::Boolean>,
}
