
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResendRequest {
	/// MsgType = 2
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'2'>,
	/// BeginSeqNo
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "7")]
	pub begin_seq_no: i32,
	/// EndSeqNo
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "16")]
	pub end_seq_no: i32,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}
