
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ListExecute {
	/// MsgType = L
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Must be unique, by customer, for the day
	#[serde(rename = "66")]
	pub list_id: String,
	/// Used with <a href="tag_394_BidType.html" target="bottom">BidType&nbsp;(394)</a> =Disclosed to provide the sell side the ability to determine the direction of the trade to execute.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "391")]
	pub client_bid_id: Option<String>,
	/// BidID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "390")]
	pub bid_id: Option<String>,
	/// Time this order request was initiated/released by the trader or trading system.
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
	/// Text
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
