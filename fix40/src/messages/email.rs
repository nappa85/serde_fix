
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Email {
	/// MsgType = C
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'C'>,
	/// EmailType
	#[serde(rename = "94")]
	pub email_type: EmailType,
	/// OrigTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42")]
	pub orig_time: Option<fix_common::UTCTimeOnly>,
	/// Can be repeated multiple times if message is related to multiple symbols.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "46")]
	pub relatd_sym: Option<char>,
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<char>,
	/// ClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<char>,
	/// LinesOfText
	#[serde(rename = "33")]
	pub lines_of_text: fix_common::RepeatingValues<LinesOfTex>,
	/// RawDataLength
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "95")]
	pub raw_data_length: Option<i32>,
	/// RawData
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "96")]
	pub raw_data: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LinesOfTex {
	/// Repeating field, number of instances defined in <a href="tag_33_LinesOfText.html" target="bottom">LinesOfText&nbsp;(33)</a>
	#[serde(rename = "58")]
	pub text: char,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum EmailType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Reply
	#[serde(rename = "1")]
	Reply,
	/// Admin Reply
	#[serde(rename = "2")]
	AdminReply,
}

impl Default for EmailType {
	fn default() -> Self {
		EmailType::New
	}
}
