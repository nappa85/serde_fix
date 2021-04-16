
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradingSessionListUpdateReport {
	/// MsgType = BS
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'B', 'S'>,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Provided for a response to a specific Trading Session List Request message (snapshot).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "335")]
	pub trad_ses_req_id: Option<String>,
	/// Specifies the action taken
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1327")]
	pub trad_ses_update_action: Option<TradSesUpdateAction>,
	/// NoTradingSessions
	#[serde(rename = "386")]
	pub trading_sessions: fix_common::RepeatingValues<TradingSession>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradingSession {
	/// Identifier for Trading Session
	#[serde(rename = "336")]
	pub trading_session_id: TradingSessionID,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// SecurityExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "207")]
	pub security_exchange: Option<String>,
	/// Market for which Trading Session applies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Market Segment for which Trading Session applies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// TradingSessionDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1326")]
	pub trading_session_desc: Option<String>,
	/// Method of Trading
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "338")]
	pub trad_ses_method: Option<TradSesMethod>,
	/// Trading Session Mode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "339")]
	pub trad_ses_mode: Option<TradSesMode>,
	/// "Y" if message is sent unsolicited as a result of a previous subscription request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "325")]
	pub unsolicited_indicator: Option<UnsolicitedIndicator>,
	/// State of trading session.
	#[serde(rename = "340")]
	pub trad_ses_status: TradSesStatus,
	/// Used with <a href="tag_340_TradSesStatus.html" target="bottom">TradSesStatus&nbsp;(340)</a> = "Request Rejected".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "567")]
	pub trad_ses_status_rej_reason: Option<TradSesStatusRejReason>,
	/// Starting time of trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "341")]
	pub trad_ses_start_time: Option<fix_common::UTCTimestamp>,
	/// Time of the opening of the trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "342")]
	pub trad_ses_open_time: Option<fix_common::UTCTimestamp>,
	/// Time of pre-close of trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "343")]
	pub trad_ses_pre_close_time: Option<fix_common::UTCTimestamp>,
	/// Closing time of trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "344")]
	pub trad_ses_close_time: Option<fix_common::UTCTimestamp>,
	/// End time of trading session
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "345")]
	pub trad_ses_end_time: Option<fix_common::UTCTimestamp>,
	/// TotalVolumeTraded
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "387")]
	pub total_volume_traded: Option<f64>,
	/// Number of order types.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1237")]
	pub no_ord_type_rules: Option<usize>,
	/// Indicates order types that are valid for the specified market segment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40")]
	pub ord_type: Option<OrdType>,
	/// Number of time in force techniques.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1239")]
	pub no_time_in_force_rules: Option<usize>,
	/// Indicates time in force techniques that are valid for the specified market segment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "59")]
	pub time_in_force: Option<TimeInForce>,
	/// Number of execution instructions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1232")]
	pub no_exec_inst_rules: Option<usize>,
	/// Indicates execution instructions that are valid for the specified market segment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1308")]
	pub exec_inst_value: Option<ExecInstValue>,
	/// Number of match rules.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1235")]
	pub no_match_rules: Option<usize>,
	/// The type of algorithm used to match orders in a specific security on an electronic trading platform. Possible values are FIFO,
	/// Allocation, Pro-rata, Lead Market Maker, Currency Calendar
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1142")]
	pub match_algorithm: Option<String>,
	/// The point in the matching process at which this trade was matched.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "574")]
	pub match_type: Option<MatchType>,
	/// The number of feed types and corresponding book depths associated with a security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1141")]
	pub no_md_feed_types: Option<usize>,
	/// Describes a class of service for a given data feed.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1022")]
	pub md_feed_type: Option<String>,
	/// The depth of book associated with a particular feed type.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "264")]
	pub market_depth: Option<MarketDepth>,
	/// Describes the type of book for which the feed is intended. Can be used when multiple feeds are provided over the same connection.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1021")]
	pub md_book_type: Option<MDBookType>,
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
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesUpdateAction {
	/// Add
	#[serde(rename = "A")]
	Add,
	/// Delete
	#[serde(rename = "D")]
	Delete,
	/// Modify
	#[serde(rename = "M")]
	Modify,
}

impl Default for TradSesUpdateAction {
	fn default() -> Self {
		TradSesUpdateAction::Add
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
pub enum TradSesMethod {
	/// Electronic
	#[serde(rename = "1")]
	Electronic,
	/// Open Outcry
	#[serde(rename = "2")]
	OpenOutcry,
	/// Two Party
	#[serde(rename = "3")]
	TwoParty,
}

impl Default for TradSesMethod {
	fn default() -> Self {
		TradSesMethod::Electronic
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesMode {
	/// Testing
	#[serde(rename = "1")]
	Testing,
	/// Simulated
	#[serde(rename = "2")]
	Simulated,
	/// Production
	#[serde(rename = "3")]
	Production,
}

impl Default for TradSesMode {
	fn default() -> Self {
		TradSesMode::Testing
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnsolicitedIndicator {
	/// Message is being sent as a result of a prior request
	#[serde(rename = "N")]
	MessageIsBeingSentAsAResultOfAPriorRequest,
	/// Message is being sent unsolicited
	#[serde(rename = "Y")]
	MessageIsBeingSentUnsolicited,
}

impl Default for UnsolicitedIndicator {
	fn default() -> Self {
		UnsolicitedIndicator::MessageIsBeingSentAsAResultOfAPriorRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesStatus {
	/// Unknown
	#[serde(rename = "0")]
	Unknown,
	/// Halted
	#[serde(rename = "1")]
	Halted,
	/// Open
	#[serde(rename = "2")]
	Open,
	/// Closed
	#[serde(rename = "3")]
	Closed,
	/// Pre-Open
	#[serde(rename = "4")]
	PreOpen,
	/// Pre-Close
	#[serde(rename = "5")]
	PreClose,
	/// Request Rejected
	#[serde(rename = "6")]
	RequestRejected,
}

impl Default for TradSesStatus {
	fn default() -> Self {
		TradSesStatus::Unknown
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradSesStatusRejReason {
	/// Unknown or invalid TradingSessionID
	#[serde(rename = "1")]
	UnknownOrInvalidTradingSessionId,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for TradSesStatusRejReason {
	fn default() -> Self {
		TradSesStatusRejReason::UnknownOrInvalidTradingSessionId
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrdType {
	/// Market
	#[serde(rename = "1")]
	Market,
	/// Limit
	#[serde(rename = "2")]
	Limit,
	/// Stop / Stop Loss
	#[serde(rename = "3")]
	StopStopLoss,
	/// Stop Limit
	#[serde(rename = "4")]
	StopLimit,
	/// Market On Close (No longer used)
	#[serde(rename = "5")]
	MarketOnClose,
	/// With Or Without
	#[serde(rename = "6")]
	WithOrWithout,
	/// Limit Or Better
	#[serde(rename = "7")]
	LimitOrBetter,
	/// Limit With Or Without
	#[serde(rename = "8")]
	LimitWithOrWithout,
	/// On Basis
	#[serde(rename = "9")]
	OnBasis,
	/// On Close (No longer used)
	#[serde(rename = "A")]
	OnClose,
	/// Limit On Close (No longer used)
	#[serde(rename = "B")]
	LimitOnClose,
	/// Forex Market (No longer used)
	#[serde(rename = "C")]
	ForexMarket,
	/// Previously Quoted
	#[serde(rename = "D")]
	PreviouslyQuoted,
	/// Previously Indicated
	#[serde(rename = "E")]
	PreviouslyIndicated,
	/// Forex Limit (No longer used)
	#[serde(rename = "F")]
	ForexLimit,
	/// Forex Swap
	#[serde(rename = "G")]
	ForexSwap,
	/// Forex Previously Quoted (No longer used)
	#[serde(rename = "H")]
	ForexPreviouslyQuoted,
	/// Funari (Limit day order with unexecuted portion handles as Market On Close. E.g. Japan)
	#[serde(rename = "I")]
	Funari,
	/// Market If Touched (MIT)
	#[serde(rename = "J")]
	MarketIfTouched,
	/// Market With Left Over as Limit (market order with unexecuted quantity becoming limit order at last price)
	#[serde(rename = "K")]
	MarketWithLeftOverAsLimit,
	/// Previous Fund Valuation Point (Historic pricing; for CIV)"
	#[serde(rename = "L")]
	PreviousFundValuationPoint,
	/// Next Fund Valuation Point (Forward pricing; for CIV)"
	#[serde(rename = "M")]
	NextFundValuationPoint,
	/// Pegged
	#[serde(rename = "P")]
	Pegged,
	/// Counter-order selection
	#[serde(rename = "Q")]
	CounterOrderSelection,
}

impl Default for OrdType {
	fn default() -> Self {
		OrdType::Market
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TimeInForce {
	/// Day (or session)
	#[serde(rename = "0")]
	Day,
	/// Good Till Cancel (GTC)
	#[serde(rename = "1")]
	GoodTillCancel,
	/// At the Opening (OPG)
	#[serde(rename = "2")]
	AtTheOpening,
	/// Immediate Or Cancel (IOC)
	#[serde(rename = "3")]
	ImmediateOrCancel,
	/// Fill Or Kill (FOK)
	#[serde(rename = "4")]
	FillOrKill,
	/// Good Till Crossing (GTX)
	#[serde(rename = "5")]
	GoodTillCrossing,
	/// Good Till Date (GTD)
	#[serde(rename = "6")]
	GoodTillDate,
	/// At the Close
	#[serde(rename = "7")]
	AtTheClose,
	/// Good Through Crossing
	#[serde(rename = "8")]
	GoodThroughCrossing,
	/// At Crossing
	#[serde(rename = "9")]
	AtCrossing,
}

impl Default for TimeInForce {
	fn default() -> Self {
		TimeInForce::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecInstValue {
	/// Stay on offerside
	#[serde(rename = "0")]
	StayOnOfferside,
	/// Not held
	#[serde(rename = "1")]
	NotHeld,
	/// Work
	#[serde(rename = "2")]
	Work,
	/// Go along
	#[serde(rename = "3")]
	GoAlong,
	/// Over the day
	#[serde(rename = "4")]
	OverTheDay,
	/// Held
	#[serde(rename = "5")]
	Held,
	/// Participate don't initiate
	#[serde(rename = "6")]
	ParticipateDonTInitiate,
	/// Strict scale
	#[serde(rename = "7")]
	StrictScale,
	/// Try to scale
	#[serde(rename = "8")]
	TryToScale,
	/// Stay on bidside
	#[serde(rename = "9")]
	StayOnBidside,
	/// No cross (cross is forbidden)
	#[serde(rename = "A")]
	NoCross,
	/// OK to cross
	#[serde(rename = "B")]
	OkToCross,
	/// Call first
	#[serde(rename = "C")]
	CallFirst,
	/// Percent of volume (indicates that the sender does not want to be all of the volume on the floor vs. a specific percentage)
	#[serde(rename = "D")]
	PercentOfVolume,
	/// Do not increase - DNI
	#[serde(rename = "E")]
	DoNotIncreaseDni,
	/// Do not reduce - DNR
	#[serde(rename = "F")]
	DoNotReduceDnr,
	/// All or none - AON
	#[serde(rename = "G")]
	AllOrNoneAon,
	/// Reinstate on System Failure (mutually exclusive with Q)
	#[serde(rename = "H")]
	ReinstateOnSystemFailure,
	/// Institutions only
	#[serde(rename = "I")]
	InstitutionsOnly,
	/// Reinstate on Trading Halt (mutually exclusive with K)
	#[serde(rename = "J")]
	ReinstateOnTradingHalt,
	/// Cancel on Trading Halt (mutually exclusive with J)
	#[serde(rename = "K")]
	CancelOnTradingHalt,
	/// Last peg (last sale)
	#[serde(rename = "L")]
	LastPeg,
	/// Mid-price peg (midprice of inside quote)
	#[serde(rename = "M")]
	MidPricePeg,
	/// Non-negotiable
	#[serde(rename = "N")]
	NonNegotiable,
	/// Opening peg
	#[serde(rename = "O")]
	OpeningPeg,
	/// Market peg
	#[serde(rename = "P")]
	MarketPeg,
	/// Cancel on System Failure (mutually exclusive with H)
	#[serde(rename = "Q")]
	CancelOnSystemFailure,
	/// Primary peg (primary market - buy at bid/sell at offer)
	#[serde(rename = "R")]
	PrimaryPeg,
	/// Suspend
	#[serde(rename = "S")]
	Suspend,
	/// Fixed Peg to Local best bid or offer at time of order
	#[serde(rename = "T")]
	FixedPegToLocalBestBidOrOfferAtTimeOfOrder,
	/// Customer Display Instruction (Rule11Ac1-1/4)
	#[serde(rename = "U")]
	CustomerDisplayInstruction,
	/// Netting (for Forex)
	#[serde(rename = "V")]
	Netting,
	/// Peg to VWAP
	#[serde(rename = "W")]
	PegToVwap,
	/// Trade Along
	#[serde(rename = "X")]
	TradeAlong,
	/// Try to Stop
	#[serde(rename = "Y")]
	TryToStop,
	/// Cancel if Not Best
	#[serde(rename = "Z")]
	CancelIfNotBest,
	/// Trailing Stop Peg
	#[serde(rename = "a")]
	TrailingStopPeg,
	/// Strict Limit (No Price Improvement)
	#[serde(rename = "b")]
	StrictLimit,
	/// Ignore Price Validity Checks
	#[serde(rename = "c")]
	IgnorePriceValidityChecks,
	/// Peg to Limit Price
	#[serde(rename = "d")]
	PegToLimitPrice,
	/// Work to Target Strategy
	#[serde(rename = "e")]
	WorkToTargetStrategy,
	/// Intermarket Sweep
	#[serde(rename = "f")]
	IntermarketSweep,
	/// External Routing Allowed
	#[serde(rename = "g")]
	ExternalRoutingAllowed,
	/// External Routing Not Allowed
	#[serde(rename = "h")]
	ExternalRoutingNotAllowed,
	/// Imbalance Only
	#[serde(rename = "i")]
	ImbalanceOnly,
	/// Single execution requested for block trade
	#[serde(rename = "j")]
	SingleExecutionRequestedForBlockTrade,
	/// Best Execution
	#[serde(rename = "k")]
	BestExecution,
	/// Suspend on system failure (mutually exclusive with H and Q)
	#[serde(rename = "l")]
	SuspendOnSystemFailure,
	/// Suspend on Trading Halt (mutually exclusive with J and K)
	#[serde(rename = "m")]
	SuspendOnTradingHalt,
	/// Reinstate on connection loss (mutually exclusive with o and p)
	#[serde(rename = "n")]
	ReinstateOnConnectionLoss,
	/// Cancel on connection loss (mutually exclusive with n and p)
	#[serde(rename = "o")]
	CancelOnConnectionLoss,
	/// Suspend on connection loss (mutually exclusive with n and o)
	#[serde(rename = "p")]
	SuspendOnConnectionLoss,
	/// Release from suspension (mutually exclusive with S)
	#[serde(rename = "q")]
	ReleaseFromSuspension,
	/// Execute as delta neutral using volatility provided
	#[serde(rename = "r")]
	ExecuteAsDeltaNeutralUsingVolatilityProvided,
	/// Execute as duration neutral
	#[serde(rename = "s")]
	ExecuteAsDurationNeutral,
	/// Execute as FX neutral
	#[serde(rename = "t")]
	ExecuteAsFxNeutral,
}

impl Default for ExecInstValue {
	fn default() -> Self {
		ExecInstValue::StayOnOfferside
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MatchType {
	/// ACT Accepted Trade
	#[serde(rename = "M3")]
	ActAcceptedTrade,
	/// ACT Default Trade
	#[serde(rename = "M4")]
	ActDefaultTrade,
	/// ACT Default After M2
	#[serde(rename = "M5")]
	ActDefaultAfterM2,
	/// ACT M6 Match
	#[serde(rename = "M6")]
	ActM6Match,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus four badges and execution
	/// time (within two-minute window)
	#[serde(rename = "A1")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus four badges
	#[serde(rename = "A2")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges and execution
	/// time (within two-minute window)
	#[serde(rename = "A3")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges
	#[serde(rename = "A4")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadges,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus execution time (within
	/// two-minute window)
	#[serde(rename = "A5")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime,
	/// Compared records resulting from stamped advisories or specialist accepts/pair-offs
	#[serde(rename = "AQ")]
	ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAcceptsPairOffs,
	/// Summarized Match using A1 exact match criteria except quantity is summarized
	#[serde(rename = "S1")]
	SummarizedMatchUsingA1ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A2 exact match criteria except quantity is summarized
	#[serde(rename = "S2")]
	SummarizedMatchUsingA2ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A3 exact match criteria except quantity is summarized
	#[serde(rename = "S3")]
	SummarizedMatchUsingA3ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A4 exact match criteria except quantity is summarized
	#[serde(rename = "S4")]
	SummarizedMatchUsingA4ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A5 exact match criteria except quantity is summarized
	#[serde(rename = "S5")]
	SummarizedMatchUsingA5ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Exact Match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator minus badges and times
	#[serde(rename = "M1")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorMinusBadgesAndTimes,
	/// Summarized Match minus badges and times
	#[serde(rename = "M2")]
	SummarizedMatchMinusBadgesAndTimes,
	/// OCS Locked In
	#[serde(rename = "MT")]
	OcsLockedIn,
	/// One-Party Trade Report (privately negotiated trade)
	#[serde(rename = "1")]
	OnePartyTradeReport,
	/// Two-Party Trade Report (privately negotiated trade)
	#[serde(rename = "2")]
	TwoPartyTradeReport,
	/// Confirmed Trade Report (reporting from recognized markets)
	#[serde(rename = "3")]
	ConfirmedTradeReport,
	/// Auto-match
	#[serde(rename = "4")]
	AutoMatch,
	/// Cross Auction
	#[serde(rename = "5")]
	CrossAuction,
	/// Counter-Order Selection
	#[serde(rename = "6")]
	CounterOrderSelection,
	/// Call Auction
	#[serde(rename = "7")]
	CallAuction,
	/// Issuing/Buy Back Auction
	#[serde(rename = "8")]
	IssuingBuyBackAuction,
}

impl Default for MatchType {
	fn default() -> Self {
		MatchType::ActAcceptedTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MarketDepth {
	/// full book depth
	#[serde(rename = "0")]
	FullBookDepth,
	/// top of book
	#[serde(rename = "1")]
	TopOfBook,
}

impl Default for MarketDepth {
	fn default() -> Self {
		MarketDepth::FullBookDepth
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDBookType {
	/// Top of Book
	#[serde(rename = "1")]
	TopOfBook,
	/// Price Depth
	#[serde(rename = "2")]
	PriceDepth,
	/// Order Depth
	#[serde(rename = "3")]
	OrderDepth,
}

impl Default for MDBookType {
	fn default() -> Self {
		MDBookType::TopOfBook
	}
}
