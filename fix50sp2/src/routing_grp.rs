
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RoutingGrp {
	/// Required if any RoutingType and RoutingIDs are specified. Indicates the number within repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "215")]
	pub routing_i_ds: Option<fix_common::RepeatingValues<RoutingID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RoutingID {
	/// Indicates type of RoutingID. Required if NoRoutingIDs is &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "216")]
	pub routing_type: Option<RoutingType>,
	/// Identifies routing destination. Required if NoRoutingIDs is &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "217")]
	pub routing_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RoutingType {
	/// Target Firm
	#[serde(rename = "1")]
	TargetFirm,
	/// Target List
	#[serde(rename = "2")]
	TargetList,
	/// Block Firm
	#[serde(rename = "3")]
	BlockFirm,
	/// Block List
	#[serde(rename = "4")]
	BlockList,
	/// Target Person
	#[serde(rename = "5")]
	TargetPerson,
	/// Block Person
	#[serde(rename = "6")]
	BlockPerson,
}

impl Default for RoutingType {
	fn default() -> Self {
		RoutingType::TargetFirm
	}
}
