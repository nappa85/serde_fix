
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TickRules {
	/// Number of tick rules. This block specifies the rules for determining how a security ticks, i.e. the price increments at which
	/// it can be quoted and traded, depending on the current price of the security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1205")]
	pub tick_rules: Option<fix_common::RepeatingValues<TickRule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TickRule {
	/// Required if NoTickRules(1205) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1206")]
	pub start_tick_price_range: Option<f64>,
	/// Ending price range for the specified tick increment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1207")]
	pub end_tick_price_range: Option<f64>,
	/// Tick increment for stated price range. Specifies the valid price increments at which a security can be quoted and traded
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1208")]
	pub tick_increment: Option<f64>,
	/// Specifies the type of tick rule which is being described.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1209")]
	pub tick_rule_type: Option<TickRuleType>,
	/// SettlPriceIncrement
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1830")]
	pub settl_price_increment: Option<f64>,
	/// SettlPriceSecondaryIncrement
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1831")]
	pub settl_price_secondary_increment: Option<f64>,
	/// Can be used to limit tick rule to specific product suite.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2571")]
	pub tick_rule_product_complex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TickRuleType {
	/// Regular
	#[serde(rename = "0")]
	Regular,
	/// Variable
	#[serde(rename = "1")]
	Variable,
	/// Fixed
	#[serde(rename = "2")]
	Fixed,
	/// Traded as a spread leg
	#[serde(rename = "3")]
	TradedAsASpreadLeg,
	/// Settled as a spread leg
	#[serde(rename = "4")]
	SettledAsASpreadLeg,
	/// Traded as spread (basis point spread)
	#[serde(rename = "5")]
	TradedAsSpread,
}
