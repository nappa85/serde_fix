
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Party {
	/// MsgType = DI
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Conditionally required when responding to a Party Action Request (35=DH) message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2328")]
	pub party_action_request_id: Option<String>,
	/// PartyActionReportID
	#[serde(rename = "2331")]
	pub party_action_report_id: String,
	/// PartyActionType
	#[serde(rename = "2329")]
	pub party_action_type: PartyActionType,
	/// PartyActionResponse
	#[serde(rename = "2332")]
	pub party_action_response: PartyActionResponse,
	/// Conditionally required when PartyActionResponse(2332) = 2 (Rejected).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2333")]
	pub party_action_reject_reason: Option<PartyActionRejectReason>,
	/// Conditionally required if present in the Party Action Request (35=DH) message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2330")]
	pub appl_test_message_indicator: Option<ApplTestMessageIndicator>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if EncodedRejectText(1665) field is specified and must immediately precede it.
	#[serde(rename = "1664")]
	/// Encoded (non-ASCII characters) representation of the RejectText(1328) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "1665")]
	pub encoded_reject_text: Option<fix_common::EncodedText<1665>>,
	/// RequestingPartyGrp
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// Parties
	#[serde(flatten)]
	pub parties: super::super::parties::Parties,
	/// RelatedPartyDetailGrp
	#[serde(flatten)]
	pub related_party_detail_grp: Option<super::super::related_party_detail_grp::RelatedPartyDetailGrp>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedText(355) field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the Text(58) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// CopyMsgIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "797")]
	pub copy_msg_indicator: Option<fix_common::Boolean>,
	/// Date for which the action applies.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2400")]
	pub effective_business_date: Option<fix_common::LocalMktDate>,
	/// MarketID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// InstrumentScope
	#[serde(flatten)]
	pub instrument_scope: Option<super::super::instrument_scope::InstrumentScope>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PartyActionType {
	/// Suspend
	#[serde(rename = "0")]
	Suspend,
	/// Halt trading
	#[serde(rename = "1")]
	HaltTrading,
	/// Reinstate
	#[serde(rename = "2")]
	Reinstate,
}

impl Default for PartyActionType {
	fn default() -> Self {
		PartyActionType::Suspend
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PartyActionResponse {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Completed
	#[serde(rename = "1")]
	Completed,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
}

impl Default for PartyActionResponse {
	fn default() -> Self {
		PartyActionResponse::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PartyActionRejectReason {
	/// Invalid party or parties
	#[serde(rename = "0")]
	InvalidPartyOrParties,
	/// Unknown requesting party
	#[serde(rename = "1")]
	UnknownRequestingParty,
	/// Not authorized
	#[serde(rename = "98")]
	NotAuthorized,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for PartyActionRejectReason {
	fn default() -> Self {
		PartyActionRejectReason::InvalidPartyOrParties
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApplTestMessageIndicator {
	/// Not a test message
	#[serde(rename = "N")]
	NotATestMessage,
	/// Test message
	#[serde(rename = "Y")]
	TestMessage,
}

impl Default for ApplTestMessageIndicator {
	fn default() -> Self {
		ApplTestMessageIndicator::NotATestMessage
	}
}
