
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Party {
	/// MsgType = CY
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
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
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
