
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AllocRegulatoryTradeIDGrp {
	/// NoAllocRegulatoryTradeIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1908")]
	pub alloc_regulatory_trade_i_ds: Option<crate::entities::RepeatingValues<AllocRegulatoryTradeID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AllocRegulatoryTradeID {
	/// Required if NoAllocRegulatoryTradeIDs(1908) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1909")]
	pub alloc_regulatory_trade_id: Option<String>,
	/// AllocRegulatoryTradeIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1910")]
	pub alloc_regulatory_trade_id_source: Option<String>,
	/// AllocRegulatoryTradeIDEvent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1911")]
	pub alloc_regulatory_trade_id_event: Option<AllocRegulatoryTradeIDEvent>,
	/// AllocRegulatoryTradeIDType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1912")]
	pub alloc_regulatory_trade_id_type: Option<AllocRegulatoryTradeIDType>,
	/// This field may be is used for multi-leg trades sent as a single message to indicate that the entry applies only to a specific
	/// leg.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2406")]
	pub alloc_regulatory_leg_ref_id: Option<String>,
	/// AllocRegulatoryTradeIDScope
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2399")]
	pub alloc_regulatory_trade_id_scope: Option<AllocRegulatoryTradeIDScope>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocRegulatoryTradeIDEvent {
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocRegulatoryTradeIDType {
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocRegulatoryTradeIDScope {
	/// Clearing member
	#[serde(rename = "1")]
	ClearingMember,
	/// Client
	#[serde(rename = "2")]
	Client,
}
