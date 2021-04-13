
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MatchingInstructions {
	/// Number of Instructions. Required if MatchingInstructions block used
	#[serde(rename = "1624")]
	pub match_inst: fix_common::RepeatingValues<MatchIns>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MatchIns {
	/// Required if NoMatchInst (1624) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1625")]
	pub match_inst: Option<MatchInst>,
	/// MatchInstMarketID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1673")]
	pub match_inst_market_id: Option<String>,
	/// Required if NoMatchInst (1624) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1626")]
	pub match_attrib_tag_id: Option<u16>,
	/// Required if NoMatchInst (1624) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1627")]
	pub match_attrib_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchInst {
	/// Match
	#[serde(rename = "1")]
	Match,
	/// Do Not Match
	#[serde(rename = "2")]
	DoNotMatch,
}

impl Default for MatchInst {
	fn default() -> Self {
		MatchInst::Match
	}
}
