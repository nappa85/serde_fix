
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SequenceReset {
	/// MsgType = 4
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'4', ' '>,
	/// GapFillFlag
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "123")]
	pub gap_fill_flag: Option<GapFillFlag>,
	/// NewSeqNo
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "36")]
	pub new_seq_no: usize,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum GapFillFlag {
	/// Gap Fill message, <a href="tag_34_MsgSeqNum.html" target="bottom">MsgSeqNum&nbsp;(34)</a> field valid
	#[serde(rename = "Y")]
	GapFillMessageMsgSeqNumFieldValid,
	/// Sequence Reset, ignore <a href="tag_34_MsgSeqNum.html" target="bottom">MsgSeqNum&nbsp;(34)</a>
	#[serde(rename = "N")]
	SequenceResetIgnoreMsgSeqNum,
}

impl Default for GapFillFlag {
	fn default() -> Self {
		GapFillFlag::GapFillMessageMsgSeqNumFieldValid
	}
}
