
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DontKnowTrade {
	/// MsgType = Q
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Broker Order ID as identified on problem execution
	#[serde(rename = "37")]
	pub order_id: String,
	/// Execution ID of problem execution
	#[serde(rename = "17")]
	pub exec_id: String,
	/// DKReason
	#[serde(rename = "127")]
	pub dk_reason: DKReason,
	/// Instrument
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Order Qty Data
	#[serde(flatten)]
	pub order_qty_data: super::super::order_qty_data::OrderQtyData,
	/// Required if specified on the <a href="message_Execution_Report_8.html" target="main">ExecutionRpt&nbsp;(8)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "32")]
	pub last_qty: Option<f64>,
	/// Required if specified on the <a href="message_Execution_Report_8.html" target="main">ExecutionRpt&nbsp;(8)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum DKReason {
	/// Unknown symbol
	#[serde(rename = "A")]
	UnknownSymbol,
	/// Wrong side
	#[serde(rename = "B")]
	WrongSide,
	/// Quantity exceeds order
	#[serde(rename = "C")]
	QuantityExceedsOrder,
	/// No matching order
	#[serde(rename = "D")]
	NoMatchingOrder,
	/// Price exceeds limit
	#[serde(rename = "E")]
	PriceExceedsLimit,
	/// Other
	#[serde(rename = "Z")]
	Other,
}

impl Default for DKReason {
	fn default() -> Self {
		DKReason::UnknownSymbol
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Side {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
	/// Buy minus
	#[serde(rename = "3")]
	BuyMinus,
	/// Sell plus
	#[serde(rename = "4")]
	SellPlus,
	/// Sell short
	#[serde(rename = "5")]
	SellShort,
	/// Sell short exempt
	#[serde(rename = "6")]
	SellShortExempt,
	/// Undisclosed (valid for IOI and List Order messages only)
	#[serde(rename = "7")]
	Undisclosed,
	/// Cross (orders where counterparty is an exchange, valid for all messages except IOIs)
	#[serde(rename = "8")]
	Cross,
	/// Cross short
	#[serde(rename = "9")]
	CrossShort,
	/// Cross short exempt
	#[serde(rename = "A")]
	CrossShortExempt,
	/// "As Defined" (for use with multileg instruments)
	#[serde(rename = "B")]
	AsDefined,
	/// "Opposite" (for use with multileg instruments)
	#[serde(rename = "C")]
	Opposite,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}
