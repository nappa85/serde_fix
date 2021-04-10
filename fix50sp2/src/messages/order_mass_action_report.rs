
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
	/// Order Mass Action Request Type accepted by the system.
	#[serde(rename = "1373")]
	pub mass_action_type: MassActionType,
	/// Specifies the scope of the action.
	#[serde(rename = "1374")]
	pub mass_action_scope: MassActionScope,
	/// Indicates the action taken by the counterparty order handling system as a result of the Action Request.
	#[serde(rename = "1375")]
	pub mass_action_response: MassActionResponse,
	/// Indicates why Order Mass Action Request was rejected Required if MassActionResponse(1375) = 0 (Rejected).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1376")]
	pub mass_action_reject_reason: Option<MassActionRejectReason>,
	/// Optional field used to indicate the total number of orders affected by the <a href="message_Order_Mass_Cancel_Request_q.html" target="main">Order Mass Cancel Request&nbsp;(q)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "533")]
	pub total_affected_orders: Option<i32>,
	/// List of orders affected by the Order Mass Action Request
	#[serde(flatten)]
	pub affected_ord_grp: Option<super::super::affected_ord_grp::AffectedOrdGrp>,
	/// List of orders not affected by the Order Mass Action Request
	#[serde(flatten)]
	pub not_affected_ord_grp: Option<super::super::not_affected_ord_grp::NotAffectedOrdGrp>,
	/// MarketID for which orders are to be affected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID for which orders are to be affected. Mutually exclusive with "TargetMarketSegmentGrp"
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// TradingSessionID for which orders are to be affected.
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
	/// Should be populated with the values provided on the associated <a href="message_Order_Mass_Action_Request_CA.html" target="main">Order Mass Action Request(MsgType=CA)&nbsp;(CA)</a> .
	#[serde(flatten)]
	pub target_parties: Option<super::super::target_parties::TargetParties>,
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
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// List of market segments affected by the Order Mass Action Request. Should only be used when request uses "TargetMarketSegmentGrp"
	#[serde(flatten)]
	pub affected_market_segment_grp: Option<super::super::affected_market_segment_grp::AffectedMarketSegmentGrp>,
	/// List of market segments not affected by the Order Mass Action Request. Should only be used when request uses "TargetMarketSegmentGrp"
	#[serde(flatten)]
	pub not_affected_market_segment_grp: Option<super::super::not_affected_market_segment_grp::NotAffectedMarketSegmentGrp>,
	/// Mutually exclusive with MarketSegmentID(1300)
	#[serde(flatten)]
	pub target_market_segment_grp: Option<super::super::target_market_segment_grp::TargetMarketSegmentGrp>,
	/// ComplianceID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "376")]
	pub compliance_id: Option<String>,
	/// ComplianceText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2404")]
	pub compliance_text: Option<String>,
	/// Must be set if EncodedComplianceText(2352) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2351")]
	pub encoded_compliance_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the ComplianceText(2404) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2352")]
	pub encoded_compliance_text: Option<String>,
	/// Specifies the reason for the action taken.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2675")]
	pub mass_action_reason: Option<MassActionReason>,
	/// Optional field used to indicate the total number of orders within the scope but not affected by the OrderMassActionRequest(35=CA).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2678")]
	pub total_not_affected_orders: Option<i32>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// Price
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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
	/// Cancel for Security Issuer
	#[serde(rename = "11")]
	CancelForSecurityIssuer,
	/// Cancel for Issuer of Underlying Security
	#[serde(rename = "12")]
	CancelForIssuerOfUnderlyingSecurity,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MassActionResponse {
	/// Rejected - See <a href="tag_1376_MassActionRejectReason.html" target="bottom">MassActionRejectReason&nbsp;(1376)</a>
	#[serde(rename = "0")]
	RejectedSeeAHrefTag1376MassActionRejectReasonHtmlTargetBottomMassActionRejectReasonNbspA,
	/// Accepted
	#[serde(rename = "1")]
	Accepted,
	/// Completed
	#[serde(rename = "2")]
	Completed,
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
	/// Invalid or unknown Security Issuer
	#[serde(rename = "10")]
	InvalidOrUnknownSecurityIssuer,
	/// Invalid or unknown Issuer of Underlying Security
	#[serde(rename = "11")]
	InvalidOrUnknownIssuerOfUnderlyingSecurity,
	/// Other
	#[serde(rename = "99")]
	Other,
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
	/// Holiday
	#[serde(rename = "7")]
	Holiday,
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
	/// Any auction
	#[serde(rename = "8")]
	AnyAuction,
	/// Unscheduled intraday auction (Elaboration: An unscheduled intraday auction might be triggered by a circuit breaker)
	#[serde(rename = "9")]
	UnscheduledIntradayAuction,
	/// Out of main session trading (Elaboration: In the context of Market Model Typology "Out of main session trading" refers to
	/// both before and after session, neither auction nor continuous trading)
	#[serde(rename = "10")]
	OutOfMainSessionTrading,
	/// Private auction
	#[serde(rename = "11")]
	PrivateAuction,
	/// Public auction
	#[serde(rename = "12")]
	PublicAuction,
	/// Group auction
	#[serde(rename = "13")]
	GroupAuction,
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
	/// Sell undisclosed
	#[serde(rename = "H")]
	SellUndisclosed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MassActionReason {
	/// No special reason (default)
	#[serde(rename = "0")]
	NoSpecialReason,
	/// Trading risk control
	#[serde(rename = "1")]
	TradingRiskControl,
	/// Clearing risk control
	#[serde(rename = "2")]
	ClearingRiskControl,
	/// Market maker protection
	#[serde(rename = "3")]
	MarketMakerProtection,
	/// Stop trading
	#[serde(rename = "4")]
	StopTrading,
	/// Emergency action
	#[serde(rename = "5")]
	EmergencyAction,
	/// Session loss or logout
	#[serde(rename = "6")]
	SessionLossOrLogout,
	/// Duplicate login
	#[serde(rename = "7")]
	DuplicateLogin,
	/// Product not traded
	#[serde(rename = "8")]
	ProductNotTraded,
	/// Instrument not traded
	#[serde(rename = "9")]
	InstrumentNotTraded,
	/// Complex instrument deleted
	#[serde(rename = "10")]
	ComplexInstrumentDeleted,
	/// Circuit breaker activated
	#[serde(rename = "11")]
	CircuitBreakerActivated,
	/// Other
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}
