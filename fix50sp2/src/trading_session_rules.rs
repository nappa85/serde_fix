
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradingSessionRules {
	/// Specifies the order types that are valid for trading. The scope of the rule is determined by the context in which the component
	/// is used. In this case, the scope is trading session.
	#[serde(flatten)]
	pub ord_type_rules: Option<super::ord_type_rules::OrdTypeRules>,
	/// Specifies the time in force rules that are valid for trading. The scope of the rule is determined by the context in which
	/// the component is used. In this case, the scope is trading session.
	#[serde(flatten)]
	pub time_in_force_rules: Option<super::time_in_force_rules::TimeInForceRules>,
	/// Specifies the execution instructions that are valid for trading. The scope of the rule is determined by the context in which
	/// the component is used. In this case, the scope is trading session.
	#[serde(flatten)]
	pub exec_inst_rules: Option<super::exec_inst_rules::ExecInstRules>,
	/// Specifies the matching rules that are valid for trading. The scope of the rule is determined by the context in which the component
	/// is used. In this case, the scope is trading session.
	#[serde(flatten)]
	pub match_rules: Option<super::match_rules::MatchRules>,
	/// Specifies the market data feed types that are valid for trading. The scope of the rule is determined by the context in which
	/// the component is used. In this case, the scope is trading session.
	#[serde(flatten)]
	pub market_data_feed_types: Option<super::market_data_feed_types::MarketDataFeedTypes>,
	/// Specifies the auction order types that are valid for trading on the identified. The scope of the rule is determined by the
	/// context in which the component is used. In this case, the scope is trading session.
	#[serde(flatten)]
	pub auction_type_rule_grp: Option<super::auction_type_rule_grp::AuctionTypeRuleGrp>,
}
