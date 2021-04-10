
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Party {
	/// MsgType = CU
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// EntitlementRequestID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1770")]
	pub entitlement_request_id: Option<String>,
	/// SubscriptionRequestType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// May be used to identify the party making the request and their role.
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// Scope of the query/request for specific party(-ies).
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Scope of the query/request for specific party roles. For example, "all information for PartyRole=24"
	#[serde(flatten)]
	pub requested_party_role_grp: Option<super::super::requested_party_role_grp::RequestedPartyRoleGrp>,
	/// EntitlementPlatform
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1784")]
	pub entitlement_platform: Option<String>,
	/// Scope of the query/request for specific securities.
	#[serde(flatten)]
	pub instrument_scope_grp: Option<super::super::instrument_scope_grp::InstrumentScopeGrp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// EntitlementStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1883")]
	pub entitlement_status: Option<EntitlementStatus>,
	/// EntitlementTypeGrp
	#[serde(flatten)]
	pub entitlement_type_grp: Option<super::super::entitlement_type_grp::EntitlementTypeGrp>,
	/// MarketSegmentScopeGrp
	#[serde(flatten)]
	pub market_segment_scope_grp: Option<super::super::market_segment_scope_grp::MarketSegmentScopeGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SubscriptionRequestType {
	/// Snapshot
	#[serde(rename = "0")]
	Snapshot,
	/// Snapshot + Updates (Subscribe)
	#[serde(rename = "1")]
	SnapshotUpdates,
	/// Disable previous Snapshot + Update Request (Unsubscribe)
	#[serde(rename = "2")]
	DisablePreviousSnapshotUpdateRequest,
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
