
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyRiskLimitsDefinitionRequest {
	/// MsgType = CS
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// RiskLimitRequestID
	#[serde(rename = "1666")]
	pub risk_limit_request_id: String,
	/// <p>May be used to identify the party making the request and their role.</p>
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// <p>Risk limits to be enforced for given party(-ies) and related party(-ies).</p>
	#[serde(flatten)]
	pub party_risk_limits_update_grp: Option<super::super::party_risk_limits_update_grp::PartyRiskLimitsUpdateGrp>,
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
