
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderMassCancelRequest {
	/// MsgType = q
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'q', ' '>,
	/// Unique ID of <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> as assigned by the institution.
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// Specifies the type of cancellation requested.
	#[serde(rename = "530")]
	pub mass_cancel_request_type: MassCancelRequestType,
	/// Trading Session in which orders are to be canceled
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// Underlying Instrument
	#[serde(flatten)]
	pub underlying_instrument: Option<super::super::underlying_instrument::UnderlyingInstrument>,
	/// Side of the market specified on the Order Mass Cancel Request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Time this report was initiated/released by the sells-side (broker, exchange, ECN) or sell-side executing system.
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MassCancelRequestType {
	/// Cancel orders for a security
	#[serde(rename = "1")]
	CancelOrdersForASecurity,
	/// Cancel orders for an Underlying security
	#[serde(rename = "2")]
	CancelOrdersForAnUnderlyingSecurity,
	/// Cancel orders for a Product
	#[serde(rename = "3")]
	CancelOrdersForAProduct,
	/// Cancel orders for a CFICode
	#[serde(rename = "4")]
	CancelOrdersForACfiCode,
	/// Cancel orders for a SecurityType
	#[serde(rename = "5")]
	CancelOrdersForASecurityType,
	/// Cancel orders for a trading session
	#[serde(rename = "6")]
	CancelOrdersForATradingSession,
	/// Cancel all orders
	#[serde(rename = "7")]
	CancelAllOrders,
}

impl Default for MassCancelRequestType {
	fn default() -> Self {
		MassCancelRequestType::CancelOrdersForASecurity
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
	/// Subscribe (e.g. CIV)
	#[serde(rename = "D")]
	Subscribe,
	/// Redeem (e.g. CIV)
	#[serde(rename = "E")]
	Redeem,
	/// Lend (FINANCING - identifies direction of collateral)
	#[serde(rename = "F")]
	Lend,
	/// Borrow (FINANCING - identifies direction of collateral)
	#[serde(rename = "G")]
	Borrow,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}
