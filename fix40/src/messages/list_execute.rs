
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ListExecute {
	/// MsgType = L
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'L'>,
	/// Must be unique, by customer, for the day
	#[serde(rename = "66")]
	pub list_id: String,
	/// WaveNo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "105")]
	pub wave_no: Option<char>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<char>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}
