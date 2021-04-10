
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityTradingRules {
	/// This block contains the base trading rules
	#[serde(flatten)]
	pub base_trading_rules: Option<super::base_trading_rules::BaseTradingRules>,
	/// This block contains the trading rules specific to a trading session
	#[serde(flatten)]
	pub trading_session_rules_grp: Option<super::trading_session_rules_grp::TradingSessionRulesGrp>,
	/// NestedInstrumentAttribute
	#[serde(flatten)]
	pub nested_instrument_attribute: Option<super::nested_instrument_attribute::NestedInstrumentAttribute>,
}
