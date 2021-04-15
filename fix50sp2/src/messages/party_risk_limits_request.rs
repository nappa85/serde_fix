
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimitsRequest {
	/// MsgType = CL
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'C', 'L'>,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// RiskLimitRequestID
	#[serde(rename = "1666")]
	pub risk_limit_request_id: String,
	/// SubscriptionRequestType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// May be used to identify the party making the request and their role
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// Scope of the query/request for specific party(-ies)
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Scope of the query/request for specific party role(s). For example, "all information for PartyRole=24."
	#[serde(flatten)]
	pub requested_party_role_grp: Option<super::super::requested_party_role_grp::RequestedPartyRoleGrp>,
	/// Scope of the query/request for specific securities
	#[serde(flatten)]
	pub requested_risk_limit_types_grp: Option<super::super::requested_risk_limit_types_grp::RequestedRiskLimitTypesGrp>,
	/// RiskLimitPlatform
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1533")]
	pub risk_limit_platform: Option<String>,
	/// RiskInstrumentScopeGrp
	#[serde(flatten)]
	pub risk_instrument_scope_grp: Option<super::super::risk_instrument_scope_grp::RiskInstrumentScopeGrp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// <p>Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	/// </p>
	#[serde(rename = "354")]
	/// <p>Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// <p>Scope of risk limit information</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1760")]
	pub risk_limit_request_type: Option<RiskLimitRequestType>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for SubscriptionRequestType {
	fn default() -> Self {
		SubscriptionRequestType::Snapshot
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RiskLimitRequestType {
	/// Definitions (default)
	#[serde(rename = "1")]
	Definitions,
	/// Utilization
	#[serde(rename = "2")]
	Utilization,
	/// Definitions and utilizations
	#[serde(rename = "3")]
	DefinitionsAndUtilizations,
}

impl Default for RiskLimitRequestType {
	fn default() -> Self {
		RiskLimitRequestType::Definitions
	}
}
