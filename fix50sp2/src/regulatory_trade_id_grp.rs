
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RegulatoryTradeIDGrp {
	/// NoRegulatoryTradeIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1907")]
	pub regulatory_trade_i_ds: Option<fix_common::RepeatingValues<RegulatoryTradeID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RegulatoryTradeID {
	/// Required if NoRegulatoryTradeIDs(1907) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1903")]
	pub regulatory_trade_id: Option<String>,
	/// RegulatoryTradeIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1905")]
	pub regulatory_trade_id_source: Option<String>,
	/// RegulatoryTradeIDEvent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1904")]
	pub regulatory_trade_id_event: Option<RegulatoryTradeIDEvent>,
	/// RegulatoryTradeIDType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1906")]
	pub regulatory_trade_id_type: Option<RegulatoryTradeIDType>,
	/// This field may be is used for multi-leg trades sent as a single message to indicate that the entry applies only to a specific
	/// leg
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2411")]
	pub regulatory_leg_ref_id: Option<String>,
	/// RegulatoryTradeIDScope
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2397")]
	pub regulatory_trade_id_scope: Option<RegulatoryTradeIDScope>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RegulatoryTradeIDEvent {
	/// Initial block trade
	#[serde(rename = "0")]
	InitialBlockTrade,
	/// Allocation
	#[serde(rename = "1")]
	Allocation,
	/// Clearing
	#[serde(rename = "2")]
	Clearing,
	/// Compression
	#[serde(rename = "3")]
	Compression,
	/// Novation
	#[serde(rename = "4")]
	Novation,
	/// Termination
	#[serde(rename = "5")]
	Termination,
	/// Post-trade valuation
	#[serde(rename = "6")]
	PostTradeValuation,
}

impl Default for RegulatoryTradeIDEvent {
	fn default() -> Self {
		RegulatoryTradeIDEvent::InitialBlockTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RegulatoryTradeIDType {
	/// Current
	#[serde(rename = "0")]
	Current,
	/// Previous
	#[serde(rename = "1")]
	Previous,
	/// Block
	#[serde(rename = "2")]
	Block,
	/// Related
	#[serde(rename = "3")]
	Related,
	/// Cleared block trade (Assigned by the CCP to a bunched order/trade when it needs to be cleared with the standby clearing firm
	/// prior to post-trade allocation)
	#[serde(rename = "4")]
	ClearedBlockTrade,
	/// Trading venue transaction identifier
	#[serde(rename = "5")]
	TradingVenueTransactionIdentifier,
}

impl Default for RegulatoryTradeIDType {
	fn default() -> Self {
		RegulatoryTradeIDType::Current
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RegulatoryTradeIDScope {
	/// Clearing member
	#[serde(rename = "1")]
	ClearingMember,
	/// Client
	#[serde(rename = "2")]
	Client,
}

impl Default for RegulatoryTradeIDScope {
	fn default() -> Self {
		RegulatoryTradeIDScope::ClearingMember
	}
}
