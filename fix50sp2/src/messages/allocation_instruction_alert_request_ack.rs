
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AllocationInstructionAlertRequestAck {
	/// MsgType = DV
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for this message. If used, other allocation messages may link to the request through this field.
	#[serde(rename = "2758")]
	pub alloc_request_id: String,
	/// AllocRequestStatus
	#[serde(rename = "2768")]
	pub alloc_request_status: AllocRequestStatus,
	/// May be used to further describe rejection reasons when AllocRequestStatus(2768)=1 (Rejected)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if EncodedRejectText(1665) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1664")]
	pub encoded_reject_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the RejectText(1328) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1665")]
	pub encoded_reject_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocRequestStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Rejected
	#[serde(rename = "1")]
	Rejected,
}
