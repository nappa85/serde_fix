
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyDetailsDefinitionRequestAck {
	/// MsgType = CY
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'C', 'Y'>,
	/// PartyDetailsListRequestID
	#[serde(rename = "1505")]
	pub party_details_list_request_id: String,
	/// PartyDetailRequestStatus
	#[serde(rename = "1878")]
	pub party_detail_request_status: PartyDetailRequestStatus,
	/// PartyDetailRequestResult
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1877")]
	pub party_detail_request_result: Option<PartyDetailRequestResult>,
	/// RequestingPartyGrp
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// PartyDetailAckGrp
	#[serde(flatten)]
	pub party_detail_ack_grp: Option<super::super::party_detail_ack_grp::PartyDetailAckGrp>,
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
pub enum PartyDetailRequestStatus {
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

impl Default for PartyDetailRequestStatus {
	fn default() -> Self {
		PartyDetailRequestStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PartyDetailRequestResult {
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

impl Default for PartyDetailRequestResult {
	fn default() -> Self {
		PartyDetailRequestResult::Successful
	}
}
