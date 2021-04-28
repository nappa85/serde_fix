
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Reject {
	/// MsgType = 3
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'3', ' '>,
	/// <a href="tag_34_MsgSeqNum.html" target="bottom">MsgSeqNum&nbsp;(34)</a> of rejected message
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "45")]
	pub ref_seq_num: u32,
	/// The tag number of the FIX field being referenced.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "371")]
	pub ref_tag_id: Option<u16>,
	/// The <a href="tag_35_MsgType.html" target="bottom">MsgType&nbsp;(35)</a> of the FIX message being referenced.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "372")]
	pub ref_msg_type: Option<super::super::standard_message_header::MsgType<'3', ' '>>,
	/// Code to identify reason for a session-level <a href="message_Reject_3.html" target="main">Reject&nbsp;(3)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "373")]
	pub session_reject_reason: Option<SessionRejectReason>,
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SessionRejectReason {
	/// Invalid tag number
	#[serde(rename = "0")]
	InvalidTagNumber,
	/// Required tag missing
	#[serde(rename = "1")]
	RequiredTagMissing,
	/// Tag not defined for this message type
	#[serde(rename = "2")]
	TagNotDefinedForThisMessageType,
	/// Undefined Tag
	#[serde(rename = "3")]
	UndefinedTag,
	/// Tag specified without a value
	#[serde(rename = "4")]
	TagSpecifiedWithoutAValue,
	/// Value is incorrect (out of range) for this tag
	#[serde(rename = "5")]
	ValueIsIncorrectForThisTag,
	/// Incorrect data format for value
	#[serde(rename = "6")]
	IncorrectDataFormatForValue,
	/// Decryption problem
	#[serde(rename = "7")]
	DecryptionProblem,
	/// Signature problem
	#[serde(rename = "8")]
	SignatureProblem,
	/// CompID problem
	#[serde(rename = "9")]
	CompIdProblem,
	/// <a href="tag_52_SendingTime.html" target="bottom">SendingTime&nbsp;(52)</a> accuracy problem
	#[serde(rename = "10")]
	SendingTimeAccuracyProblem,
	/// Invalid <a href="tag_35_MsgType.html" target="bottom">MsgType&nbsp;(35)</a>
	#[serde(rename = "11")]
	InvalidMsgType,
	/// XML Validation error
	#[serde(rename = "12")]
	XmlValidationError,
	/// Tag appears more than once
	#[serde(rename = "13")]
	TagAppearsMoreThanOnce,
	/// Tag specified out of required order
	#[serde(rename = "14")]
	TagSpecifiedOutOfRequiredOrder,
	/// Repeating group fields out of order
	#[serde(rename = "15")]
	RepeatingGroupFieldsOutOfOrder,
	/// Incorrect NumInGroup count for repeating group
	#[serde(rename = "16")]
	IncorrectNumInGroupCountForRepeatingGroup,
	/// Non "data" value includes field delimiter (SOH character)
	#[serde(rename = "17")]
	NonDataValueIncludesFieldDelimiter,
}

impl Default for SessionRejectReason {
	fn default() -> Self {
		SessionRejectReason::InvalidTagNumber
	}
}
