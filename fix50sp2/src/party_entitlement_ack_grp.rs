
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyEntitlementAckGrp {
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
	/// <p>Required if NoPartyEntitlements(1772) &gt; 0.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1883")]
	pub entitlement_status: Option<EntitlementStatus>,
	/// EntitlementResult
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1884")]
	pub entitlement_result: Option<EntitlementResult>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// EncodedRejectTextLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1664")]
	pub encoded_reject_text_len: Option<usize>,
	/// EncodedRejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1665")]
	pub encoded_reject_text: Option<String>,
	/// Optional when PartyDetailGrp is provided or <a href="tag_1324_ListUpdateAction.html" target="bottom">ListUpdateAction&nbsp;(1324)</a> = A(Add).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1885")]
	pub entitlement_ref_id: Option<String>,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EntitlementResult {
	/// Successful(default)
	#[serde(rename = "0")]
	Successful,
	/// Invalid party(-ies)
	#[serde(rename = "1")]
	InvalidParty,
	/// Invalid related party(-ies)
	#[serde(rename = "2")]
	InvalidRelatedParty,
	/// Invalid entitlement type(s)
	#[serde(rename = "3")]
	InvalidEntitlementType,
	/// Invalid entitlement ID(s)/ref ID(s)
	#[serde(rename = "4")]
	InvalidEntitlementIdRefId,
	/// Invalid entitlement attribute(s)
	#[serde(rename = "5")]
	InvalidEntitlementAttribute,
	/// Invalid instrument scope(s)
	#[serde(rename = "6")]
	InvalidInstrumentScope,
	/// Invalid market segment scope(s)
	#[serde(rename = "7")]
	InvalidMarketSegmentScope,
	/// Invalid start date
	#[serde(rename = "8")]
	InvalidStartDate,
	/// Invalid end date
	#[serde(rename = "9")]
	InvalidEndDate,
	/// Instrument scope not supported
	#[serde(rename = "10")]
	InstrumentScopeNotSupported,
	/// Market segment scope not supported
	#[serde(rename = "11")]
	MarketSegmentScopeNotSupported,
	/// Entitlement not approved for party(-ies)
	#[serde(rename = "12")]
	EntitlementNotApprovedForParty,
	/// Entitlement already defined for party(-ies)
	#[serde(rename = "13")]
	EntitlementAlreadyDefinedForParty,
	/// Instrument not approved for party(-ies)
	#[serde(rename = "14")]
	InstrumentNotApprovedForParty,
	/// Not authorized
	#[serde(rename = "98")]
	NotAuthorized,
	/// Other
	#[serde(rename = "99")]
	Other,
}
