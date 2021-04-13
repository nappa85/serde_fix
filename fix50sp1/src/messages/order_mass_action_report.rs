
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Order {
	/// MsgType = BZ
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ClOrdID provided on the Order Mass Action Request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// Unique Identifier for the Order Mass Action Report.
	#[serde(rename = "1369")]
	pub mass_action_report_id: String,
	/// Specifies the type of action requested.
	#[serde(rename = "1373")]
	pub mass_action_type: MassActionType,
	/// Specifies the scope of the action.
	#[serde(rename = "1374")]
	pub mass_action_scope: MassActionScope,
	/// Indicates the action taken by the counterparty order handling system as a result of the Action Request. 0 - Indicates Order
	/// Mass Action Request was rejected.
	#[serde(rename = "1375")]
	pub mass_action_response: MassActionResponse,
	/// Indicates why Order Mass Action Request was rejected. Required if MassActionResponse = 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1376")]
	pub mass_action_reject_reason: Option<MassActionRejectReason>,
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
	/// OrigClOrdID(41) as the next field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1370")]
	pub not_affected_orders: Option<fix_common::RepeatingValues<NotAffectedOrder>>,
	/// MarketID for which orders are to be affected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID for which orders are to be affected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Trading Session in which orders are to be affected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID for which orders are to be affected
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// UnderlyingInstrument
	#[serde(flatten)]
	pub underlying_instrument: Option<super::super::underlying_instrument::UnderlyingInstrument>,
	/// Side of the market specified on the Order Mass Action Request
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
	/// Required if <a href="tag_534_NoAffectedOrders.html" target="bottom">NoAffectedOrders&nbsp;(534)</a> &gt; 0. Indicates the client order id of an order affected by the Order Mass Cancel Request. If order(s) were manually delivered
	/// (or otherwise not delivered over FIX and not assigned a ClOrdID) this field should contain string "MANUAL".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41")]
	pub orig_cl_ord_id: Option<String>,
	/// Contains the <a href="tag_37_OrderID.html" target="bottom">OrderID&nbsp;(37)</a> assigned by the counterparty of an affected order. Not required as part of the repeating group if OrigClOrdID(41) has a value
	/// other than "MANUAL".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "535")]
	pub affected_order_id: Option<String>,
	/// Contains the <a href="tag_198_SecondaryOrderID.html" target="bottom">SecondaryOrderID&nbsp;(198)</a> assigned by the counterparty of an affected order. Not required as part of the repeating group.
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MassActionType {
	/// TSuspend orders
	#[serde(rename = "0")]
	TSuspendOrders,
	/// Release orders from suspension
	#[serde(rename = "1")]
	ReleaseOrdersFromSuspension,
	/// Cancel orders
	#[serde(rename = "2")]
	CancelOrders,
}

impl Default for MassActionType {
	fn default() -> Self {
		MassActionType::TSuspendOrders
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MassActionScope {
	/// All orders for a security
	#[serde(rename = "1")]
	AllOrdersForASecurity,
	/// All orders for an underlying security
	#[serde(rename = "2")]
	AllOrdersForAnUnderlyingSecurity,
	/// All orders for a Product
	#[serde(rename = "3")]
	AllOrdersForAProduct,
	/// All orders for a CFICode
	#[serde(rename = "4")]
	AllOrdersForACfiCode,
	/// All orders for a SecurityType
	#[serde(rename = "5")]
	AllOrdersForASecurityType,
	/// All orders for a trading session
	#[serde(rename = "6")]
	AllOrdersForATradingSession,
	/// All orders
	#[serde(rename = "7")]
	AllOrders,
	/// All orders for a Market
	#[serde(rename = "8")]
	AllOrdersForAMarket,
	/// All orders for a Market Segment
	#[serde(rename = "9")]
	AllOrdersForAMarketSegment,
	/// All orders for a Security Group
	#[serde(rename = "10")]
	AllOrdersForASecurityGroup,
}

impl Default for MassActionScope {
	fn default() -> Self {
		MassActionScope::AllOrdersForASecurity
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MassActionResponse {
	/// Rejected - See <a href="tag_1376_MassActionRejectReason.html" target="bottom">MassActionRejectReason&nbsp;(1376)</a>
	#[serde(rename = "0")]
	RejectedSeeAHrefTag1376MassActionRejectReasonHtmlTargetBottomMassActionRejectReasonNbspA,
	/// Accepted
	#[serde(rename = "1")]
	Accepted,
}

impl Default for MassActionResponse {
	fn default() -> Self {
		MassActionResponse::RejectedSeeAHrefTag1376MassActionRejectReasonHtmlTargetBottomMassActionRejectReasonNbspA
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MassActionRejectReason {
	/// Mass Action Not Supported
	#[serde(rename = "0")]
	MassActionNotSupported,
	/// Invalid or unknown security
	#[serde(rename = "1")]
	InvalidOrUnknownSecurity,
	/// Invalid or unknown underlying security
	#[serde(rename = "2")]
	InvalidOrUnknownUnderlyingSecurity,
	/// Invalid or unknown Product
	#[serde(rename = "3")]
	InvalidOrUnknownProduct,
	/// Invalid or unknown CFICode
	#[serde(rename = "4")]
	InvalidOrUnknownCfiCode,
	/// Invalid or unknown SecurityType
	#[serde(rename = "5")]
	InvalidOrUnknownSecurityType,
	/// Invalid or unknown trading session
	#[serde(rename = "6")]
	InvalidOrUnknownTradingSession,
	/// Invalid or unknown Market
	#[serde(rename = "7")]
	InvalidOrUnknownMarket,
	/// Invalid or unknown Market Segment
	#[serde(rename = "8")]
	InvalidOrUnknownMarketSegment,
	/// Invalid or unknown Security Group
	#[serde(rename = "9")]
	InvalidOrUnknownSecurityGroup,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for MassActionRejectReason {
	fn default() -> Self {
		MassActionRejectReason::MassActionNotSupported
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
