
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyDetailsUpdateGrp {
	/// NoPartyUpdates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1676")]
	pub party_updates: Option<fix_common::RepeatingValues<PartyUpdate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyUpdate {
	/// <p>Required when <a href="tag_1676_NoPartyUpdates.html" target="bottom">NoPartyUpdates (1676)&nbsp;(1676)</a> &gt; 0
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1324")]
	pub list_update_action: Option<ListUpdateAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListUpdateAction {
	/// Add
	#[serde(rename = "A")]
	Add,
	/// Delete
	#[serde(rename = "D")]
	Delete,
	/// Modify
	#[serde(rename = "M")]
	Modify,
	/// Snapshot
	#[serde(rename = "S")]
	Snapshot,
}
