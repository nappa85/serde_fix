
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApplicationMessageRequestAck {
	/// MsgType = BX
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'B', 'X'>,
	/// Identifier for the Application Message Request Ack.
	#[serde(rename = "1353")]
	pub appl_response_id: String,
	/// Identifier of the request associated with this ACK message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1346")]
	pub appl_req_id: Option<String>,
	/// ApplReqType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1347")]
	pub appl_req_type: Option<ApplReqType>,
	/// ApplResponseType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1348")]
	pub appl_response_type: Option<ApplResponseType>,
	/// Total number of messages included in transmission.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1349")]
	pub appl_total_message_count: Option<i32>,
	/// ApplIDRequestAckGrp
	#[serde(flatten)]
	pub appl_id_request_ack_grp: Option<super::super::appl_id_request_ack_grp::ApplIDRequestAckGrp>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ApplReqType {
	/// Retransmission of application messages for the specified Applications
	#[serde(rename = "0")]
	RetransmissionOfApplicationMessagesForTheSpecifiedApplications,
	/// Subscription to the specified Applications
	#[serde(rename = "1")]
	SubscriptionToTheSpecifiedApplications,
	/// Request for the last ApplLastSeqNum published for the specified Applications
	#[serde(rename = "2")]
	RequestForTheLastApplLastSeqNumPublishedForTheSpecifiedApplications,
	/// Request valid set of Applications
	#[serde(rename = "3")]
	RequestValidSetOfApplications,
	/// Unsubscribe to the specified Applications
	#[serde(rename = "4")]
	UnsubscribeToTheSpecifiedApplications,
	/// Cancel retransmission
	#[serde(rename = "5")]
	CancelRetransmission,
	/// Cancel retransmission and unsubscribe to the specified applications
	#[serde(rename = "6")]
	CancelRetransmissionAndUnsubscribeToTheSpecifiedApplications,
}

impl Default for ApplReqType {
	fn default() -> Self {
		ApplReqType::RetransmissionOfApplicationMessagesForTheSpecifiedApplications
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ApplResponseType {
	/// Request successfully processed
	#[serde(rename = "0")]
	RequestSuccessfullyProcessed,
	/// Application does not exist
	#[serde(rename = "1")]
	ApplicationDoesNotExist,
	/// Messages not available
	#[serde(rename = "2")]
	MessagesNotAvailable,
}

impl Default for ApplResponseType {
	fn default() -> Self {
		ApplResponseType::RequestSuccessfullyProcessed
	}
}
