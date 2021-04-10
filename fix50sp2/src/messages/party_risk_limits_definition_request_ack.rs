
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Party {
	/// MsgType = CT
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// RiskLimitRequestID
	#[serde(rename = "1666")]
	pub risk_limit_request_id: String,
	/// RiskLimitRequestResult
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1761")]
	pub risk_limit_request_result: Option<RiskLimitRequestResult>,
	/// RiskLimitRequestStatus
	#[serde(rename = "1762")]
	pub risk_limit_request_status: RiskLimitRequestStatus,
	/// RequestingPartyGrp
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// PartyRiskLimitsAckGrp
	#[serde(flatten)]
	pub party_risk_limits_ack_grp: Option<super::super::party_risk_limits_ack_grp::PartyRiskLimitsAckGrp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RiskLimitRequestResult {
	/// Successful
	#[serde(rename = "0")]
	Successful,
	/// Invalid party
	#[serde(rename = "1")]
	InvalidParty,
	/// Invalid related party
	#[serde(rename = "2")]
	InvalidRelatedParty,
	/// Invalid risk limit type
	#[serde(rename = "3")]
	InvalidRiskLimitType,
	/// Invalid risk limit ID
	#[serde(rename = "4")]
	InvalidRiskLimitId,
	/// Invalid risk limit amount
	#[serde(rename = "5")]
	InvalidRiskLimitAmount,
	/// Invalid risk warning level action
	#[serde(rename = "6")]
	InvalidRiskWarningLevelAction,
	/// Invalid risk instrument scope
	#[serde(rename = "7")]
	InvalidRiskInstrumentScope,
	/// Risk limit actions not supported
	#[serde(rename = "8")]
	RiskLimitActionsNotSupported,
	/// Warning levels not supported
	#[serde(rename = "9")]
	WarningLevelsNotSupported,
	/// Warning level actions not supported
	#[serde(rename = "10")]
	WarningLevelActionsNotSupported,
	/// Risk instrument scope not supported
	#[serde(rename = "11")]
	RiskInstrumentScopeNotSupported,
	/// Risk limit not approved for party
	#[serde(rename = "12")]
	RiskLimitNotApprovedForParty,
	/// Risk limit already defined for party
	#[serde(rename = "13")]
	RiskLimitAlreadyDefinedForParty,
	/// Instrument not approved for party
	#[serde(rename = "14")]
	InstrumentNotApprovedForParty,
	/// Not authorized
	#[serde(rename = "98")]
	NotAuthorized,
	/// Other
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RiskLimitRequestStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Accepted with changes
	#[serde(rename = "1")]
	AcceptedWithChanges,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
	/// Acceptance pending
	#[serde(rename = "3")]
	AcceptancePending,
}
