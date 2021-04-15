
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Logon {
	/// MsgType = A
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'A'>,
	/// (Always unencrypted)
	#[serde(rename = "98")]
	pub encrypt_method: EncryptMethod,
	/// HeartBtInt
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "108")]
	pub heart_bt_int: i32,
	/// Required for some authentication methods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "95")]
	pub raw_data_length: Option<i32>,
	/// Required for some authentication methods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "96")]
	pub raw_data: Option<String>,
	/// Indicates both sides of a FIX session should reset sequence numbers
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "141")]
	pub reset_seq_num_flag: Option<ResetSeqNumFlag>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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
	/// PGP/DES-MD5 (see app note on FIX web site)
	#[serde(rename = "5")]
	PgpDesMd5,
	/// PEM/DES-MD5 (see app note on FIX web site)
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
