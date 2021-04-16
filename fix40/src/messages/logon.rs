
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
	#[serde(rename = "95")]
	/// Required for some authentication methods
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "96")]
	pub raw_data: Option<fix_common::EncodedText<96>>,
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
