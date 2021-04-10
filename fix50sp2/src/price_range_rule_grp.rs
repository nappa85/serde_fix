
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceRangeRuleGrp {
	/// NoPriceRangeRules
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2550")]
	pub price_range_rules: Option<crate::entities::RepeatingValues<PriceRangeRule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriceRangeRule {
	/// Required if NoPriceRangeRules(2550) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2551")]
	pub start_price_range: Option<f64>,
	/// EndPriceRange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2552")]
	pub end_price_range: Option<f64>,
	/// Mutually exclusive with PriceRangePercentage(2554).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2553")]
	pub price_range_value: Option<f64>,
	/// Mutually exclusive with PriceRangeValue(2553).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2554")]
	pub price_range_percentage: Option<f32>,
	/// Can be used to provide an identifier so that the rule can be reference via the ID elsewhere.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2556")]
	pub price_range_rule_id: Option<String>,
	/// Can be used to limit price range to specific product suite.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2555")]
	pub price_range_product_complex: Option<String>,
}
