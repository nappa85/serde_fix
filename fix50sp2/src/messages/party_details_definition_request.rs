
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Party {
	/// MsgType = CX
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// PartyDetailsListRequestID
	#[serde(rename = "1505")]
	pub party_details_list_request_id: String,
	/// Can be used to identify the party making the request and their role.
	#[serde(flatten)]
	pub requesting_party_grp: Option<super::super::requesting_party_grp::RequestingPartyGrp>,
	/// Specifies the parties and relationships between parties to be defined, modified or deleted.
	#[serde(flatten)]
	pub party_details_update_grp: super::super::party_details_update_grp::PartyDetailsUpdateGrp,
	/// Text
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
