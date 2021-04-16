
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DontKnowTrade {
	/// MsgType = Q
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'Q'>,
	/// Broker Order Id as identified on problem execution
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<char>,
	/// Execution Id of problem execution
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "17")]
	pub exec_id: Option<i32>,
	/// DKReason
	#[serde(rename = "127")]
	pub dk_reason: DKReason,
	/// Symbol
	#[serde(rename = "55")]
	pub symbol: char,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// OrderQty
	#[serde(rename = "38")]
	pub order_qty: OrderQty,
	/// LastShares
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "32")]
	pub last_shares: i32,
	/// LastPx
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "31")]
	pub last_px: f64,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<char>,
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
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderQty {
}
