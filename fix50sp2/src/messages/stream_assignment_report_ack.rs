
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Stream {
	/// MsgType = CE
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// StreamAsgnAckType
	#[serde(rename = "1503")]
	pub stream_asgn_ack_type: StreamAsgnAckType,
	/// StreamAsgnRptID
	#[serde(rename = "1501")]
	pub stream_asgn_rpt_id: String,
	/// StreamAsgnRejReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1502")]
	pub stream_asgn_rej_reason: Option<StreamAsgnRejReason>,
	/// Can be used to provide additional information regarding the assignment report, such as reject description.
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
pub enum StreamAsgnAckType {
	/// Assignment Accepted
	#[serde(rename = "0")]
	AssignmentAccepted,
	/// Assignment Rejected
	#[serde(rename = "1")]
	AssignmentRejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamAsgnRejReason {
	/// Unknown client
	#[serde(rename = "0")]
	UnknownClient,
	/// Exceeds maximum size
	#[serde(rename = "1")]
	ExceedsMaximumSize,
	/// Unknown or Invalid currency pair
	#[serde(rename = "2")]
	UnknownOrInvalidCurrencyPair,
	/// No available stream
	#[serde(rename = "3")]
	NoAvailableStream,
	/// Other
	#[serde(rename = "99")]
	Other,
}
