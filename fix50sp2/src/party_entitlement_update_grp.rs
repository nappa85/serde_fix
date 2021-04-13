
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyEntitlementUpdateGrp {
	/// NoPartyEntitlements
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1772")]
	pub party_entitlements: Option<fix_common::RepeatingValues<PartyEntitlement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyEntitlement {
	/// <p>Required if NoPartyEntitlements(1772) &gt; 0.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1324")]
	pub list_update_action: Option<ListUpdateAction>,
	/// Optional when PartyDetailGrp is provided or <a href="tag_1324_ListUpdateAction.html" target="bottom">ListUpdateAction&nbsp;(1324)</a> = A(Add).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1885")]
	pub entitlement_ref_id: Option<String>,
	/// EntitlementStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1883")]
	pub entitlement_status: Option<EntitlementStatus>,
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

impl Default for ListUpdateAction {
	fn default() -> Self {
		ListUpdateAction::Add
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EntitlementStatus {
	/// Accepted
	#[serde(rename = "0")]
	N0,
	/// Accepted with changes
	#[serde(rename = "1")]
	N1,
	/// Rejected
	#[serde(rename = "2")]
	N2,
	/// Pending
	#[serde(rename = "3")]
	N3,
	/// Requested
	#[serde(rename = "4")]
	N4,
	/// Deferred (Entitlement definition request is being postponed or delayed)
	#[serde(rename = "5")]
	N5,
}

impl Default for EntitlementStatus {
	fn default() -> Self {
		EntitlementStatus::N0
	}
}
