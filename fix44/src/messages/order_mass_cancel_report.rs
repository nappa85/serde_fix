
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderMassCancelReport {
	/// MsgType = r
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'r', ' '>,
	/// ClOrdID provided on the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// Unique Identifier for the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> assigned by the recipient of the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a>
	#[serde(rename = "37")]
	pub order_id: String,
	/// Secondary Order ID assigned by the recipient of the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// Order Mass Cancel Request Type accepted by the system.
	#[serde(rename = "530")]
	pub mass_cancel_request_type: MassCancelRequestType,
	/// Indicates the action taken by the counterparty order handling system as a result of the Cancel Request. 0 - Indicates <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> was rejected.
	#[serde(rename = "531")]
	pub mass_cancel_response: MassCancelResponse,
	/// Indicates why <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> was rejected. Required if <a href="tag_531_MassCancelResponse.html" target="bottom">MassCancelResponse&nbsp;(531)</a> = 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "532")]
	pub mass_cancel_reject_reason: Option<MassCancelRejectReason>,
	/// Optional field used to indicate the total number of orders affected by the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "533")]
	pub total_affected_orders: Option<i32>,
	/// Optional field used to indicate the number of order identifiers for orders affected by the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> . Must be followed with <a href="tag_41_OrigClOrdID.html" target="bottom">OrigClOrdID&nbsp;(41)</a> as the next field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "534")]
	pub affected_orders: Option<fix_common::RepeatingValues<AffectedOrder>>,
	/// Trading Session in which orders are to be canceled.
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
	/// Side of the market specified on the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Time this report was initiated/released by the sells-side (broker, exchange, ECN) or sell-side executing system.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AffectedOrder {
	/// Required if <a href="tag_534_NoAffectedOrders.html" target="bottom">NoAffectedOrders&nbsp;(534)</a> &gt; 0. Indicates the client order id of an order affected by the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41")]
	pub orig_cl_ord_id: Option<String>,
	/// Contains the <a href="tag_37_OrderID.html" target="bottom">OrderID&nbsp;(37)</a> assigned by the counterparty of an affected order. Not required as part of the repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "535")]
	pub affected_order_id: Option<String>,
	/// Contains the <a href="tag_198_SecondaryOrderID.html" target="bottom">SecondaryOrderID&nbsp;(198)</a> assigned by the counterparty of an affected order. Not required as part of the repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "536")]
	pub affected_secondary_order_id: Option<String>,
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
pub enum MassCancelResponse {
	/// Cancel Request Rejected - See <a href="tag_532_MassCancelRejectReason.html" target="bottom">MassCancelRejectReason&nbsp;(532)</a>
	#[serde(rename = "0")]
	CancelRequestRejectedSeeAHrefTag532MassCancelRejectReasonHtmlTargetBottomMassCancelRejectReasonNbspA,
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

impl Default for MassCancelResponse {
	fn default() -> Self {
		MassCancelResponse::CancelRequestRejectedSeeAHrefTag532MassCancelRejectReasonHtmlTargetBottomMassCancelRejectReasonNbspA
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MassCancelRejectReason {
	/// Mass Cancel Not Supported
	#[serde(rename = "0")]
	MassCancelNotSupported,
	/// Invalid or unknown Security
	#[serde(rename = "1")]
	InvalidOrUnknownSecurity,
	/// Invalid or unknown Underlying
	#[serde(rename = "2")]
	InvalidOrUnknownUnderlying,
	/// Invalid or unknown Product
	#[serde(rename = "3")]
	InvalidOrUnknownProduct,
	/// Invalid or unknown CFICode
	#[serde(rename = "4")]
	InvalidOrUnknownCfiCode,
	/// Invalid or unknown Security Type
	#[serde(rename = "5")]
	InvalidOrUnknownSecurityType,
	/// Invalid or unknown trading session
	#[serde(rename = "6")]
	InvalidOrUnknownTradingSession,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for MassCancelRejectReason {
	fn default() -> Self {
		MassCancelRejectReason::MassCancelNotSupported
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
