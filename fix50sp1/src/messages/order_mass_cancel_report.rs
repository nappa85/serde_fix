
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderMassCancelReport {
	/// MsgType = r
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'r', ' '>,
	/// ClOrdID provided on the Order Mass Cancel Request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// (Deprecated in FIX 5.0 SP1).Unique Identifier for the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> assigned by the recipient of the Order Mass Cancel Request
	#[serde(rename = "37")]
	pub order_id: String,
	/// Unique Identifier for the Order Mass Cancel Report assigned by the recipient of the Order Mass Cancel Request
	#[serde(rename = "1369")]
	pub mass_action_report_id: String,
	/// (Deprecated in FIX 5.0 SP1).Secondary Order ID assigned by the recipient of the Order Mass Cancel Request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// Order Mass Cancel Request Type accepted by the system
	#[serde(rename = "530")]
	pub mass_cancel_request_type: MassCancelRequestType,
	/// Indicates the action taken by the counterparty order handling system as a result of the Cancel Request 0 - Indicates <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> was rejected.
	#[serde(rename = "531")]
	pub mass_cancel_response: MassCancelResponse,
	/// Indicates why <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> was rejected Required if <a href="tag_531_MassCancelResponse.html" target="bottom">MassCancelResponse&nbsp;(531)</a> = 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "532")]
	pub mass_cancel_reject_reason: Option<MassCancelRejectReason>,
	/// Optional field used to indicate the total number of orders affected by the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "533")]
	pub total_affected_orders: Option<i32>,
	/// Optional field used to indicate the number of order identifiers for orders affected by the Order Mass Cancel Request. Must
	/// be followed with <a href="tag_41_OrigClOrdID.html" target="bottom">OrigClOrdID&nbsp;(41)</a> as the next field
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "534")]
	pub affected_orders: Option<fix_common::RepeatingValues<AffectedOrder>>,
	/// Optional field used to indicate the number of order identifiers for orders not affected by the request. Must be followed with
	/// NotAffOrigClOrdID(1372) as the next field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1370")]
	pub not_affected_orders: Option<fix_common::RepeatingValues<NotAffectedOrder>>,
	/// Trading Session in which orders are to be canceled
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// Insert here the set of "Parties" (firm identification) fields defined in "common components of application messages".
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// Insert here the set of "UnderlyingInstrument" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub underlying_instrument: Option<super::super::underlying_instrument::UnderlyingInstrument>,
	/// MarketID for which orders are to be affected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID for which orders are to be affected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Side of the market specified on the Order Mass Cancel Request
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
	/// Required if <a href="tag_534_NoAffectedOrders.html" target="bottom">NoAffectedOrders&nbsp;(534)</a> &gt; 0. Indicates the client order id of an order affected by the Order Mass Cancel Request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41")]
	pub orig_cl_ord_id: Option<String>,
	/// Contains the <a href="tag_37_OrderID.html" target="bottom">OrderID&nbsp;(37)</a> assigned by the counterparty of an affected order. Not required as part of the repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "535")]
	pub affected_order_id: Option<String>,
	/// Contains the <a href="tag_198_SecondaryOrderID.html" target="bottom">SecondaryOrderID&nbsp;(198)</a> assigned by the counterparty of an affected order. Not required as part of the repeating group
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "536")]
	pub affected_secondary_order_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NotAffectedOrder {
	/// Required if NoNotAffectedOrders(1370) &gt; 0 and must be the first repeating field in the group. Indicates the client order id
	/// of an order not affected by the request. If order(s) were manually delivered (or otherwise not delivered over FIX and not
	/// assigned a ClOrdID) this field should contain string "MANUAL".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1372")]
	pub not_aff_orig_cl_ord_id: Option<String>,
	/// Contains the OrderID assigned by the counterparty of an unaffected order. Not required as part of the repeating group if NotAffOrigClOrdID(1372)
	/// has a value other than "MANUAL".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1371")]
	pub not_affected_order_id: Option<String>,
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
	/// Cancel orders for a market
	#[serde(rename = "8")]
	CancelOrdersForAMarket,
	/// Cancel orders for a market segment
	#[serde(rename = "9")]
	CancelOrdersForAMarketSegment,
	/// Cancel orders for a security group
	#[serde(rename = "A")]
	CancelOrdersForASecurityGroup,
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
	CancelRequestRejectedSeeMassCancelRejectReason,
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
	/// Cancel orders for a market
	#[serde(rename = "8")]
	CancelOrdersForAMarket,
	/// Cancel orders for a market segment
	#[serde(rename = "9")]
	CancelOrdersForAMarketSegment,
	/// Cancel orders for a security group
	#[serde(rename = "A")]
	CancelOrdersForASecurityGroup,
}

impl Default for MassCancelResponse {
	fn default() -> Self {
		MassCancelResponse::CancelRequestRejectedSeeMassCancelRejectReason
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
	/// Invalid or unkown Underlying security
	#[serde(rename = "2")]
	InvalidOrUnkownUnderlyingSecurity,
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
	/// Invalid or unknown Market
	#[serde(rename = "7")]
	InvalidOrUnknownMarket,
	/// Invalid or unkown Market Segment
	#[serde(rename = "8")]
	InvalidOrUnkownMarketSegment,
	/// Invalid or unknown Security Group
	#[serde(rename = "9")]
	InvalidOrUnknownSecurityGroup,
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
pub enum TradingSessionID {
	/// Day
	#[serde(rename = "1")]
	Day,
	/// HalfDay
	#[serde(rename = "2")]
	HalfDay,
	/// Morning
	#[serde(rename = "3")]
	Morning,
	/// Afternoon
	#[serde(rename = "4")]
	Afternoon,
	/// Evening
	#[serde(rename = "5")]
	Evening,
	/// After-hours
	#[serde(rename = "6")]
	AfterHours,
}

impl Default for TradingSessionID {
	fn default() -> Self {
		TradingSessionID::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradingSessionSubID {
	/// Pre-Trading
	#[serde(rename = "1")]
	PreTrading,
	/// Opening or opening auction
	#[serde(rename = "2")]
	OpeningOrOpeningAuction,
	/// (Continuous) Trading
	#[serde(rename = "3")]
	Trading,
	/// Closing or closing auction
	#[serde(rename = "4")]
	ClosingOrClosingAuction,
	/// Post-Trading
	#[serde(rename = "5")]
	PostTrading,
	/// Intraday Auction
	#[serde(rename = "6")]
	IntradayAuction,
	/// Quiescent
	#[serde(rename = "7")]
	Quiescent,
}

impl Default for TradingSessionSubID {
	fn default() -> Self {
		TradingSessionSubID::PreTrading
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
