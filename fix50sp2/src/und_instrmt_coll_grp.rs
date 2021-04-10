
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UndInstrmtCollGrp {
	/// Number of legs that make up the Security
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<Underlying>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Underlying {
	/// Required if <a href="tag_711_NoUnderlyings.html" target="bottom">NoUnderlyings&nbsp;(711)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "944")]
	pub coll_action: Option<CollAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollAction {
	/// Retain
	#[serde(rename = "0")]
	Retain,
	/// Add
	#[serde(rename = "1")]
	Add,
	/// Remove
	#[serde(rename = "2")]
	Remove,
}
