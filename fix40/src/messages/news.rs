
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct News {
	/// MsgType = B
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'B'>,
	/// OrigTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42")]
	pub orig_time: Option<fix_common::UTCTimeOnly>,
	/// Urgency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "61")]
	pub urgency: Option<Urgency>,
	/// Can be repeated multiple times if message is related to multiple symbols.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "46")]
	pub relatd_sym: Option<String>,
	/// LinesOfText
	#[serde(rename = "33")]
	pub lines_of_text: fix_common::RepeatingValues<LinesOfTex>,
	/// RawDataLength
	#[serde(rename = "95")]
	/// RawData
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "96")]
	pub raw_data: Option<fix_common::EncodedText<96>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LinesOfTex {
	/// Repeating field, number of instances defined in <a href="tag_33_LinesOfText.html" target="bottom">LinesOfText&nbsp;(33)</a>
	#[serde(rename = "58")]
	pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Urgency {
	/// Normal
	#[serde(rename = "0")]
	Normal,
	/// Flash
	#[serde(rename = "1")]
	Flash,
	/// Background
	#[serde(rename = "2")]
	Background,
}

impl Default for Urgency {
	fn default() -> Self {
		Urgency::Normal
	}
}
