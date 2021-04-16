
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyDetailAckGrp {
	/// NoPartyUpdates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1676")]
	pub party_updates: Option<fix_common::RepeatingValues<PartyUpdate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyUpdate {
	/// <p>Required if NoPartyUpdates(1676) &gt; 0.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1324")]
	pub list_update_action: Option<ListUpdateAction>,
	/// <p>Required if NoPartyUpdates(1676) &gt; 0.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1879")]
	pub party_detail_definition_status: Option<PartyDetailDefinitionStatus>,
	/// PartyDetailDefinitionResult
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1880")]
	pub party_detail_definition_result: Option<PartyDetailDefinitionResult>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// EncodedRejectTextLen
	#[serde(rename = "1664")]
	/// EncodedRejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for ListUpdateAction {
	fn default() -> Self {
		ListUpdateAction::Add
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PartyDetailDefinitionStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Accepted with changes
	#[serde(rename = "1")]
	AcceptedWithChanges,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
}

impl Default for PartyDetailDefinitionStatus {
	fn default() -> Self {
		PartyDetailDefinitionStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PartyDetailDefinitionResult {
	/// Successful (default)
	#[serde(rename = "0")]
	Successful,
	/// Invalid party(-ies)
	#[serde(rename = "1")]
	InvalidParty,
	/// Invalid related party(-ies)
	#[serde(rename = "2")]
	InvalidRelatedParty,
	/// Invalid party status(es)
	#[serde(rename = "3")]
	InvalidPartyStatus,
	/// Not authorized
	#[serde(rename = "98")]
	NotAuthorized,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for PartyDetailDefinitionResult {
	fn default() -> Self {
		PartyDetailDefinitionResult::Successful
	}
}
