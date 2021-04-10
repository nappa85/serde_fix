
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Application {
	/// MsgType = BW
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for request.
	#[serde(rename = "1346")]
	pub appl_req_id: String,
	/// Type of Application Message Request being made.
	#[serde(rename = "1347")]
	pub appl_req_type: ApplReqType,
	/// ApplIDRequestGrp
	#[serde(flatten)]
	pub appl_id_request_grp: Option<super::super::appl_id_request_grp::ApplIDRequestGrp>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Allows user to provide reason for request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
