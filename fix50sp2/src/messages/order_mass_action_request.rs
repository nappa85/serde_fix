
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Order {
	/// MsgType = CA
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier of the order as assigned by institution or by the intermediary (CIV term, not a hub/service bureau) with
	/// closest association with the investor.
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// Specifies the type of action requested
	#[serde(rename = "1373")]
	pub mass_action_type: MassActionType,
	/// Specifies the scope of the action
	#[serde(rename = "1374")]
	pub mass_action_scope: MassActionScope,
	/// MarketID for which orders are to be affected
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID for which orders are to be affected. Mutually exclusive with "TargetMarketSegmentGrp"
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Trading Session in which orders are to be affected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// TargetParties
	#[serde(flatten)]
	pub target_parties: Option<super::super::target_parties::TargetParties>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// UnderlyingInstrument
	#[serde(flatten)]
	pub underlying_instrument: Option<super::super::underlying_instrument::UnderlyingInstrument>,
	/// Can be used to filter for orders of a single instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// TransactTime
	#[serde(rename = "60")]
	pub transact_time: fix_common::UTCTimestamp,
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
	/// List of market segments for which orders are to be affected. Mutually exclusive with MarketSegmentID(1300)
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
	/// Specifies the reason for the action requested.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2675")]
	pub mass_action_reason: Option<MassActionReason>,
	/// Can be used to filter for orders of a single instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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
