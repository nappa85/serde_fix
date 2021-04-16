
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Reject {
	/// MsgType = 3
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'3'>,
	/// <a href="tag_34_MsgSeqNum.html" target="bottom">MsgSeqNum&nbsp;(34)</a> of rejected message
	#[serde(rename = "45")]
	pub ref_seq_num: RefSeqNum,
	/// Where possible, message to explain reason for rejection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<char>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RefSeqNum {
}
