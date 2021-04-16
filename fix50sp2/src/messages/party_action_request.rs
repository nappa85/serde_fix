
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PartyActionRequest {
	/// MsgType = DH
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'D', 'H'>,
	/// PartyActionRequestID
	#[serde(rename = "2328")]
	pub party_action_request_id: String,
	/// PartyActionType
	#[serde(rename = "2329")]
	pub party_action_type: PartyActionType,
	/// ApplTestMessageIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2330")]
	pub appl_test_message_indicator: Option<ApplTestMessageIndicator>,
	/// RequestingPartyGrp
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// Used to specify the trading party on which the action is applied to.
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
	/// Use to reduce the scope to a market.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Use to reduce the scope to a market segment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Use to reduce the scope of instruments.
	#[serde(flatten)]
	pub instrument_scope: Option<super::super::instrument_scope::InstrumentScope>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
