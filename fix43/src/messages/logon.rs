
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Logon {
	/// MsgType = A
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// (Always unencrypted)
	#[serde(rename = "98")]
	pub encrypt_method: EncryptMethod,
	/// Note same value used by both sides
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "108")]
	pub heart_bt_int: i32,
	/// Required for some authentication methods
	#[serde(rename = "95")]
	/// Required for some authentication methods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "96")]
	pub raw_data: Option<fix_common::EncodedText<96>>,
	/// Indicates both sides of a FIX session should reset sequence numbers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "141")]
	pub reset_seq_num_flag: Option<ResetSeqNumFlag>,
	/// Can be used to specify the maximum number of bytes supported for messages received
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "383")]
	pub max_message_size: Option<usize>,
	/// Specifies the number of repeating MsgTypes specified
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "384")]
	pub msg_types: Option<fix_common::RepeatingValues<MsgType>>,
	/// Can be used to specify that this FIX session will be sending and receiving "test" vs. "production" messages.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "464")]
	pub test_message_indicator: Option<TestMessageIndicator>,
	/// Username
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "553")]
	pub username: Option<String>,
	/// Note: minimal security exists without transport-level encryption.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "554")]
	pub password: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MsgType {
	/// Specifies a specific, supported <a href="tag_35_MsgType.html" target="bottom">MsgType&nbsp;(35)</a> . Required if <a href="tag_384_NoMsgTypes.html" target="bottom">NoMsgTypes&nbsp;(384)</a> is &gt; 0. Should be specified from the point of view of the sender of the <a href="message_Logon_A.html" target="main">Logon&nbsp;(A)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "372")]
	pub ref_msg_type: Option<String>,
	/// Indicates direction (send vs. receive) of a supported <a href="tag_35_MsgType.html" target="bottom">MsgType&nbsp;(35)</a> . Required if <a href="tag_384_NoMsgTypes.html" target="bottom">NoMsgTypes&nbsp;(384)</a> is &gt; 0. Should be specified from the point of view of the sender of the <a href="message_Logon_A.html" target="main">Logon&nbsp;(A)</a> message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "385")]
	pub msg_direction: Option<MsgDirection>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum EncryptMethod {
	/// None / other
	#[serde(rename = "0")]
	NoneOther,
	/// PKCS (proprietary)
	#[serde(rename = "1")]
	Pkcs,
	/// DES (ECB mode)
	#[serde(rename = "2")]
	Des,
	/// PKCS/DES (proprietary)
	#[serde(rename = "3")]
	PkcsDes,
	/// PGP/DES (defunct)
	#[serde(rename = "4")]
	PgpDes,
	/// PGP/DES-MD5 (see app note on <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="https://www.fixtrading.org/" target="_blank">FIX web site</a> )
	#[serde(rename = "5")]
	PgpDesMd5,
	/// PEM/DES-MD5 (see app note on <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="https://www.fixtrading.org/" target="_blank">FIX web site</a> )
	#[serde(rename = "6")]
	PemDesMd5,
}

impl Default for EncryptMethod {
	fn default() -> Self {
		EncryptMethod::NoneOther
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ResetSeqNumFlag {
	/// Yes, reset sequence numbers
	#[serde(rename = "Y")]
	YesResetSequenceNumbers,
	/// No
	#[serde(rename = "N")]
	No,
}

impl Default for ResetSeqNumFlag {
	fn default() -> Self {
		ResetSeqNumFlag::YesResetSequenceNumbers
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TestMessageIndicator {
	/// True (Test)
	#[serde(rename = "Y")]
	True,
	/// False (Production)
	#[serde(rename = "N")]
	False,
}

impl Default for TestMessageIndicator {
	fn default() -> Self {
		TestMessageIndicator::True
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MsgDirection {
	/// Send
	#[serde(rename = "S")]
	Send,
	/// Receive
	#[serde(rename = "R")]
	Receive,
}

impl Default for MsgDirection {
	fn default() -> Self {
		MsgDirection::Send
	}
}
