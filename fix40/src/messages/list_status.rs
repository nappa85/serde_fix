
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ListStatus {
	/// MsgType = N
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'N'>,
	/// ListID
	#[serde(rename = "66")]
	pub list_id: String,
	/// WaveNo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "105")]
	pub wave_no: Option<char>,
	/// Total number of messages required to status complete list.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "82")]
	pub no_rpts: i32,
	/// Sequence number of this report message.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "83")]
	pub rpt_seq: i32,
	/// Number of orders statused in this message, i.e. number of repeating groups to follow.
	#[serde(rename = "73")]
	pub orders: fix_common::RepeatingValues<Order>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Order {
	/// ClOrdID
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// CumQty
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "14")]
	pub cum_qty: u32,
	/// CxlQty
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "84")]
	pub cxl_qty: i32,
	/// AvgPx
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "6")]
	pub avg_px: f64,
}
