
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimitsDefinitionRequestAck {
	/// MsgType = CT
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'C', 'T'>,
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
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for RiskLimitRequestResult {
	fn default() -> Self {
		RiskLimitRequestResult::Successful
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for RiskLimitRequestStatus {
	fn default() -> Self {
		RiskLimitRequestStatus::Accepted
	}
}
