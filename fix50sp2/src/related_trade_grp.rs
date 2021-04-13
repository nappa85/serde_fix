
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedTradeGrp {
	/// NoRelatedTrades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1855")]
	pub related_trades: Option<fix_common::RepeatingValues<RelatedTrade>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedTrade {
	/// Required if NoRelatedTrades(1855) &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1856")]
	pub related_trade_id: Option<String>,
	/// RelatedTradeIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1857")]
	pub related_trade_id_source: Option<RelatedTradeIDSource>,
	/// Optionally used to help identify the trade when <a href="tag_1856_RelatedTradeID.html" target="bottom">RelatedTradeID&nbsp;(1856)</a> is not unique across multiple days.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1858")]
	pub related_trade_date: Option<fix_common::LocalMktDate>,
	/// Optionally used to help identify the trade when <a href="tag_1856_RelatedTradeID.html" target="bottom">RelatedTradeID&nbsp;(1856)</a> is not unique across multiple days.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1859")]
	pub related_trade_market_id: Option<String>,
	/// RelatedTradeQuantity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1860")]
	pub related_trade_quantity: Option<f64>,
	/// Optionally used for RelatedTradeIDSource(1857) = 6 (Regulatory trade ID) when RelatedTradeID(1856) is not unique across multiple
	/// reporting entities.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2103")]
	pub related_regulatory_trade_id_source: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RelatedTradeIDSource {
	/// Non-FIX source
	#[serde(rename = "0")]
	NonFixSource,
	/// Trade ID
	#[serde(rename = "1")]
	TradeId,
	/// Secondary trade ID
	#[serde(rename = "2")]
	SecondaryTradeId,
	/// Trade report ID
	#[serde(rename = "3")]
	TradeReportId,
	/// Firm trade ID
	#[serde(rename = "4")]
	FirmTradeId,
	/// Secondary firm trade ID
	#[serde(rename = "5")]
	SecondaryFirmTradeId,
	/// Regulatory trade ID
	#[serde(rename = "6")]
	RegulatoryTradeId,
}

impl Default for RelatedTradeIDSource {
	fn default() -> Self {
		RelatedTradeIDSource::NonFixSource
	}
}
