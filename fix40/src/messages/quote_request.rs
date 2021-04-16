
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteRequest {
	/// MsgType = R
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'R'>,
	/// QuoteReqID
	#[serde(rename = "131")]
	pub quote_req_id: char,
	/// Symbol
	#[serde(rename = "55")]
	pub symbol: char,
	/// SymbolSfx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "65")]
	pub symbol_sfx: Option<char>,
	/// SecurityID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "48")]
	pub security_id: Option<char>,
	/// IDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "22")]
	pub id_source: Option<IDSource>,
	/// Issuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "106")]
	pub issuer: Option<char>,
	/// SecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "107")]
	pub security_desc: Option<char>,
	/// Useful for verifying security identification
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "140")]
	pub prev_close_px: Option<f64>,
	/// Side
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// OrderQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "38")]
	pub order_qty: Option<OrderQty>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum IDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN number
	#[serde(rename = "4")]
	IsinNumber,
	/// RIC code
	#[serde(rename = "5")]
	RicCode,
}

impl Default for IDSource {
	fn default() -> Self {
		IDSource::Cusip
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
