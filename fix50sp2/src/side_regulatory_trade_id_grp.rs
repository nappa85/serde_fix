
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideRegulatoryTradeIDGrp {
	/// NoSideRegulatoryTradeIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1971")]
	pub side_regulatory_trade_i_ds: Option<fix_common::RepeatingValues<SideRegulatoryTradeID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SideRegulatoryTradeID {
	/// Required if NoSideRegulatoryTradeIDs(1971) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1972")]
	pub side_regulatory_trade_id: Option<String>,
	/// SideRegulatoryTradeIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1973")]
	pub side_regulatory_trade_id_source: Option<String>,
	/// SideRegulatoryTradeIDEvent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1974")]
	pub side_regulatory_trade_id_event: Option<SideRegulatoryTradeIDEvent>,
	/// SideRegulatoryTradeIDType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1975")]
	pub side_regulatory_trade_id_type: Option<SideRegulatoryTradeIDType>,
	/// This field may be is used for multi-leg trades sent as a single message to indicate that the entry applies only to a specific
	/// leg.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2416")]
	pub side_regulatory_leg_ref_id: Option<String>,
	/// SideRegulatoryTradeIDScope
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2398")]
	pub side_regulatory_trade_id_scope: Option<SideRegulatoryTradeIDScope>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SideRegulatoryTradeIDEvent {
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
pub enum SideRegulatoryTradeIDType {
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
pub enum SideRegulatoryTradeIDScope {
	/// Clearing member
	#[serde(rename = "1")]
	ClearingMember,
	/// Client
	#[serde(rename = "2")]
	Client,
}
