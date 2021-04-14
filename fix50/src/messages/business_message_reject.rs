
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BusinessMessageReject {
	/// MsgType = j
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// MsgSeqNum of rejected message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "45")]
	pub ref_seq_num: Option<usize>,
	/// The <a href="tag_35_MsgType.html" target="bottom">MsgType&nbsp;(35)</a> of the FIX message being referenced.
	#[serde(rename = "372")]
	pub ref_msg_type: String,
	/// The value of the business-level ID field on the message being referenced. Required unless the corresponding ID field (see
	/// list above) was not specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "379")]
	pub business_reject_ref_id: Option<String>,
	/// Code to identify reason for a <a href="message_Business_Message_Reject_j.html" target="main">Business Message Reject&nbsp;(j)</a> message.
	#[serde(rename = "380")]
	pub business_reject_reason: BusinessRejectReason,
	/// Where possible, message to explain reason for rejection
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BusinessRejectReason {
	/// Other
	#[serde(rename = "0")]
	Other,
	/// Unknown ID
	#[serde(rename = "1")]
	UnknownId,
	/// Unknown Security
	#[serde(rename = "2")]
	UnknownSecurity,
	/// Unknown Message Type
	#[serde(rename = "3")]
	UnknownMessageType,
	/// Application not available
	#[serde(rename = "4")]
	ApplicationNotAvailable,
	/// Conditionally required field missing
	#[serde(rename = "5")]
	ConditionallyRequiredFieldMissing,
	/// Not Authorized
	#[serde(rename = "6")]
	NotAuthorized,
	/// DeliverTo firm not available at this time
	#[serde(rename = "7")]
	DeliverToFirmNotAvailableAtThisTime,
	/// Invalid price increment
	#[serde(rename = "18")]
	InvalidPriceIncrement,
}

impl Default for BusinessRejectReason {
	fn default() -> Self {
		BusinessRejectReason::Other
	}
}
