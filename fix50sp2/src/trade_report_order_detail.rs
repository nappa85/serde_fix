
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeReportOrderDetail {
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// SecondaryOrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// In the case of quotes can be mapped to QuoteMsgID(1166) of a single <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a> or <a href="tag_117_QuoteID.html" target="bottom">QuoteID&nbsp;(117)</a> of a <a href="message_Mass_Quote_i.html" target="main">MassQuote&nbsp;(i)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// In the case of quotes can be mapped to <a href="tag_117_QuoteID.html" target="bottom">QuoteID&nbsp;(117)</a> of a single <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a> or <a href="tag_299_QuoteEntryID.html" target="bottom">QuoteEntryID&nbsp;(299)</a> of a <a href="message_Mass_Quote_i.html" target="main">MassQuote&nbsp;(i)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// ListID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "66")]
	pub list_id: Option<String>,
	/// Some hosts assign an order a new order id under special circumstances. The RefOrdID field will connect the same underlying
	/// order across changing OrderIDs.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1080")]
	pub ref_order_id: Option<String>,
	/// RefOrderIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1081")]
	pub ref_order_id_source: Option<RefOrderIDSource>,
	/// The reason for updating the RefOrdID.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1431")]
	pub ref_ord_id_reason: Option<RefOrdIDReason>,
	/// Order type from the order associated with the trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40")]
	pub ord_type: Option<OrdType>,
	/// Order price at time of trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "44")]
	pub price: Option<f64>,
	/// Stop/Limit order price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "99")]
	pub stop_px: Option<f64>,
	/// Execution Instruction from the order associated with the trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "18")]
	pub exec_inst: Option<fix_common::SeparatedValues<ExecInst>>,
	/// Status of order as of this trade report.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "39")]
	pub ord_status: Option<OrdStatus>,
	/// Order quantity at time of trade.
	#[serde(flatten)]
	pub order_qty_data: Option<super::order_qty_data::OrderQtyData>,
	/// LeavesQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "151")]
	pub leaves_qty: Option<f64>,
	/// CumQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "14")]
	pub cum_qty: Option<f64>,
	/// TimeInForce
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "59")]
	pub time_in_force: Option<TimeInForce>,
	/// The order expiration date/time in UTC.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "126")]
	pub expire_time: Option<fix_common::UTCTimestamp>,
	/// DisplayInstruction
	#[serde(flatten)]
	pub display_instruction: Option<super::display_instruction::DisplayInstruction>,
	/// OrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "528")]
	pub order_capacity: Option<OrderCapacity>,
	/// OrderRestrictions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "529")]
	pub order_restrictions: Option<fix_common::SeparatedValues<OrderRestrictions>>,
	/// BookingType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "775")]
	pub booking_type: Option<BookingType>,
	/// OrigCustOrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1432")]
	pub orig_cust_order_capacity: Option<OrigCustOrderCapacity>,
	/// OrderInputDevice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "821")]
	pub order_input_device: Option<String>,
	/// LotType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1093")]
	pub lot_type: Option<LotType>,
	/// TransBkdTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "483")]
	pub trans_bkd_time: Option<fix_common::UTCTimestamp>,
	/// OrigOrdModTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "586")]
	pub orig_ord_mod_time: Option<fix_common::UTCTimestamp>,
	/// OrderPercentOfTotalVolume
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2766")]
	pub order_percent_of_total_volume: Option<f32>,
	/// MatchingInstructions
	#[serde(flatten)]
	pub matching_instructions: Option<super::matching_instructions::MatchingInstructions>,
	/// PreTradeAnonymity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1091")]
	pub pre_trade_anonymity: Option<fix_common::Boolean>,
	/// The(minimum or suggested) period of time a quoted price is to be tradable before it becomes indicative.(i.e. quoted price
	/// becomes off-the-wire).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1629")]
	pub exposure_duration: Option<i32>,
	/// ExposureDurationUnit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1916")]
	pub exposure_duration_unit: Option<ExposureDurationUnit>,
	/// May be used as an alternative to MatchingInstructions when the identifier does not appear in another field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2362")]
	pub self_match_prevention_id: Option<String>,
	/// OrderOrigination
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1724")]
	pub order_origination: Option<OrderOrigination>,
	/// OrderAttributeGrp
	#[serde(flatten)]
	pub order_attribute_grp: Option<super::order_attribute_grp::OrderAttributeGrp>,
	/// ExDestinationType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2704")]
	pub ex_destination_type: Option<ExDestinationType>,
	/// RelatedOrderGrp
	#[serde(flatten)]
	pub related_order_grp: Option<super::related_order_grp::RelatedOrderGrp>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RefOrderIDSource {
	/// Secondary order ID
	#[serde(rename = "0")]
	SecondaryOrderId,
	/// Order ID
	#[serde(rename = "1")]
	OrderId,
	/// Market data entry ID
	#[serde(rename = "2")]
	MarketDataEntryId,
	/// Quote entry ID
	#[serde(rename = "3")]
	QuoteEntryId,
	/// Original order ID
	#[serde(rename = "4")]
	OriginalOrderId,
	/// Quote ID
	#[serde(rename = "5")]
	QuoteId,
	/// Quote request ID
	#[serde(rename = "6")]
	QuoteRequestId,
	/// Previous order identifier
	#[serde(rename = "7")]
	PreviousOrderIdentifier,
	/// Previous quote identifier
	#[serde(rename = "8")]
	PreviousQuoteIdentifier,
	/// Parent order identifier
	#[serde(rename = "9")]
	ParentOrderIdentifier,
	/// Manual order identifier
	#[serde(rename = "10")]
	ManualOrderIdentifier,
}

impl Default for RefOrderIDSource {
	fn default() -> Self {
		RefOrderIDSource::SecondaryOrderId
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RefOrdIDReason {
	/// GTC from previous day
	#[serde(rename = "0")]
	GtcFromPreviousDay,
	/// Partial Fill Remaining
	#[serde(rename = "1")]
	PartialFillRemaining,
	/// Order Changed
	#[serde(rename = "2")]
	OrderChanged,
}

impl Default for RefOrdIDReason {
	fn default() -> Self {
		RefOrdIDReason::GtcFromPreviousDay
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
	/// Previous Fund Valuation Point (Historic pricing; for CIV)
	#[serde(rename = "L")]
	PreviousFundValuationPoint,
	/// Next Fund Valuation Point (Forward pricing; for CIV)
	#[serde(rename = "M")]
	NextFundValuationPoint,
	/// Pegged
	#[serde(rename = "P")]
	Pegged,
	/// Counter-order selection
	#[serde(rename = "Q")]
	CounterOrderSelection,
	/// Stop on Bid or Offer (A stop order that is triggered by a bid or offer price movement (quote) at which point the stopped order
	/// becomes a market order, also known as "stop on quote" in some markets (e.g. US markets). In the US equities market it is common
	/// to trigger a stop off the National Best Bid or Offer (NBBO))
	#[serde(rename = "R")]
	StopOnBidOrOfferAtWhichPointTheStoppedOrderBecomesAMarketOrderAlsoKnownAsStopOnQuoteInSomeMarketsInTheUsEquitiesMarketItIsCommonToTriggerAStopOffTheNationalBestBidOrOffer,
	/// Stop Limit on Bid or Offer (A stop order that is triggered by a bid or offer price movement (quote) at which ponit the stopped
	/// order becomes a limit order, also known as "stop limit on quote" in some markets (e.g. US markets). In the US equities market
	/// it is common to trigger a stop off the National Best Bid or Offer (NBBO)
	#[serde(rename = "S")]
	StopLimitOnBidOrOfferAtWhichPonitTheStoppedOrderBecomesALimitOrderAlsoKnownAsStopLimitOnQuoteInSomeMarketsInTheUsEquitiesMarketItIsCommonToTriggerAStopOffTheNationalBestBidOrOffer,
}

impl Default for OrdType {
	fn default() -> Self {
		OrdType::Market
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExecInst {
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
	/// Minimum Guaranteed Fill Eligible
	#[serde(rename = "u")]
	MinimumGuaranteedFillEligible,
	/// Bypass Non-Displayed Liquidity
	#[serde(rename = "v")]
	BypassNonDisplayedLiquidity,
	/// Lock (mutually exclusive with q)
	#[serde(rename = "w")]
	Lock,
	/// Ignore notional value checks
	#[serde(rename = "x")]
	IgnoreNotionalValueChecks,
	/// Trade at reference price
	#[serde(rename = "y")]
	TradeAtReferencePrice,
	/// Allow facilitation
	#[serde(rename = "z")]
	AllowFacilitation,
}

impl Default for ExecInst {
	fn default() -> Self {
		ExecInst::StayOnOfferside
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrdStatus {
	/// New
	#[serde(rename = "0")]
	New,
	/// Partially filled
	#[serde(rename = "1")]
	PartiallyFilled,
	/// Filled
	#[serde(rename = "2")]
	Filled,
	/// Done for day
	#[serde(rename = "3")]
	DoneForDay,
	/// Canceled
	#[serde(rename = "4")]
	Canceled,
	/// Replaced (No longer used)
	#[serde(rename = "5")]
	Replaced,
	/// Pending Cancel (i.e. result of Order Cancel Request)
	#[serde(rename = "6")]
	PendingCancel,
	/// Stopped
	#[serde(rename = "7")]
	Stopped,
	/// Rejected
	#[serde(rename = "8")]
	Rejected,
	/// Suspended
	#[serde(rename = "9")]
	Suspended,
	/// Pending New
	#[serde(rename = "A")]
	PendingNew,
	/// Calculated
	#[serde(rename = "B")]
	Calculated,
	/// Expired
	#[serde(rename = "C")]
	Expired,
	/// Accepted for Bidding
	#[serde(rename = "D")]
	AcceptedForBidding,
	/// Pending Replace (i.e. result of Order Cancel/Replace Request)
	#[serde(rename = "E")]
	PendingReplace,
}

impl Default for OrdStatus {
	fn default() -> Self {
		OrdStatus::New
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
	/// Good For Time
	#[serde(rename = "A")]
	GoodForTime,
	/// Good for auction (GFA)
	#[serde(rename = "B")]
	GoodForAuction,
	/// Good for this Month (GFM)
	#[serde(rename = "C")]
	GoodForThisMonth,
}

impl Default for TimeInForce {
	fn default() -> Self {
		TimeInForce::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderCapacity {
	/// Agency
	#[serde(rename = "A")]
	Agency,
	/// Proprietary
	#[serde(rename = "G")]
	Proprietary,
	/// Individual
	#[serde(rename = "I")]
	Individual,
	/// Principal
	#[serde(rename = "P")]
	Principal,
	/// Riskless Principal
	#[serde(rename = "R")]
	RisklessPrincipal,
	/// Agent for Other Member
	#[serde(rename = "W")]
	AgentForOtherMember,
	/// Mixed capacity
	#[serde(rename = "M")]
	MixedCapacity,
}

impl Default for OrderCapacity {
	fn default() -> Self {
		OrderCapacity::Agency
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderRestrictions {
	/// Program Trade
	#[serde(rename = "1")]
	ProgramTrade,
	/// Index Arbitrage
	#[serde(rename = "2")]
	IndexArbitrage,
	/// Non-Index Arbitrage
	#[serde(rename = "3")]
	NonIndexArbitrage,
	/// Competing Market Maker
	#[serde(rename = "4")]
	CompetingMarketMaker,
	/// Acting as Market Maker or Specialist in the security
	#[serde(rename = "5")]
	ActingAsMarketMakerOrSpecialistInTheSecurity,
	/// Acting as Market Maker or Specialist in the underlying security of a derivative security
	#[serde(rename = "6")]
	ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity,
	/// Foreign Entity (of foreign governmnet or regulatory jurisdiction)
	#[serde(rename = "7")]
	ForeignEntity,
	/// External Market Participant
	#[serde(rename = "8")]
	ExternalMarketParticipant,
	/// External Inter-connected Market Linkage
	#[serde(rename = "9")]
	ExternalInterConnectedMarketLinkage,
	/// Riskless Arbitrage
	#[serde(rename = "A")]
	RisklessArbitrage,
	/// Issuer Holding
	#[serde(rename = "B")]
	IssuerHolding,
	/// Issue Price Stabilization
	#[serde(rename = "C")]
	IssuePriceStabilization,
	/// Non-algorithmic
	#[serde(rename = "D")]
	NonAlgorithmic,
	/// Algorithmic
	#[serde(rename = "E")]
	Algorithmic,
	/// Cross
	#[serde(rename = "F")]
	Cross,
	/// Insider Account
	#[serde(rename = "G")]
	InsiderAccount,
	/// Significant Shareholder
	#[serde(rename = "H")]
	SignificantShareholder,
	/// Normal Course Issuer Bid (NCIB)
	#[serde(rename = "I")]
	NormalCourseIssuerBid,
}

impl Default for OrderRestrictions {
	fn default() -> Self {
		OrderRestrictions::ProgramTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum BookingType {
	/// Regular booking
	#[serde(rename = "0")]
	RegularBooking,
	/// CFD (Contract for difference)
	#[serde(rename = "1")]
	Cfd,
	/// Total Return Swap
	#[serde(rename = "2")]
	TotalReturnSwap,
}

impl Default for BookingType {
	fn default() -> Self {
		BookingType::RegularBooking
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrigCustOrderCapacity {
	/// Member trading for their own account
	#[serde(rename = "1")]
	MemberTradingForTheirOwnAccount,
	/// Clearing Firm trading for its proprietary account
	#[serde(rename = "2")]
	ClearingFirmTradingForItsProprietaryAccount,
	/// Member trading for another member
	#[serde(rename = "3")]
	MemberTradingForAnotherMember,
	/// All Other
	#[serde(rename = "4")]
	AllOther,
}

impl Default for OrigCustOrderCapacity {
	fn default() -> Self {
		OrigCustOrderCapacity::MemberTradingForTheirOwnAccount
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LotType {
	/// Odd Lot
	#[serde(rename = "1")]
	OddLot,
	/// Round Lot
	#[serde(rename = "2")]
	RoundLot,
	/// Block Lot
	#[serde(rename = "3")]
	BlockLot,
	/// Round lot based upon <a href="tag_996_UnitOfMeasure.html" target="bottom">UnitOfMeasure&nbsp;(996)</a>
	#[serde(rename = "4")]
	RoundLotBasedUponUnitOfMeasure,
}

impl Default for LotType {
	fn default() -> Self {
		LotType::OddLot
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExposureDurationUnit {
	/// Seconds (default if not specified)
	#[serde(rename = "0")]
	Seconds,
	/// Tenths of a second
	#[serde(rename = "1")]
	TenthsOfASecond,
	/// Hundredths of a second
	#[serde(rename = "2")]
	HundredthsOfASecond,
	/// milliseconds
	#[serde(rename = "3")]
	Milliseconds,
	/// microseconds
	#[serde(rename = "4")]
	Microseconds,
	/// nanoseconds
	#[serde(rename = "5")]
	Nanoseconds,
	/// minutes
	#[serde(rename = "10")]
	Minutes,
	/// hours
	#[serde(rename = "11")]
	Hours,
	/// days
	#[serde(rename = "12")]
	Days,
	/// weeks
	#[serde(rename = "13")]
	Weeks,
	/// months
	#[serde(rename = "14")]
	Months,
	/// years
	#[serde(rename = "15")]
	Years,
}

impl Default for ExposureDurationUnit {
	fn default() -> Self {
		ExposureDurationUnit::Seconds
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderOrigination {
	/// Order received from a customer
	#[serde(rename = "1")]
	OrderReceivedFromACustomer,
	/// Order received from within the firm
	#[serde(rename = "2")]
	OrderReceivedFromWithinTheFirm,
	/// Order received from another broker-dealer
	#[serde(rename = "3")]
	OrderReceivedFromAnotherBrokerDealer,
	/// Order received from a customer or originated from within the firm
	#[serde(rename = "4")]
	OrderReceivedFromACustomerOrOriginatedFromWithinTheFirm,
	/// Order received from a direct access or sponsored access customer
	#[serde(rename = "5")]
	OrderReceivedFromADirectAccessOrSponsoredAccessCustomer,
	/// Order received from a foreign dealer equivalent
	#[serde(rename = "6")]
	OrderReceivedFromAForeignDealerEquivalent,
	/// Order received from an execution-only service
	#[serde(rename = "7")]
	OrderReceivedFromAnExecutionOnlyService,
}

impl Default for OrderOrigination {
	fn default() -> Self {
		OrderOrigination::OrderReceivedFromACustomer
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExDestinationType {
	/// No restriction
	#[serde(rename = "0")]
	NoRestriction,
	/// Can be traded only on a trading venue
	#[serde(rename = "1")]
	CanBeTradedOnlyOnATradingVenue,
	/// Can be traded only on a Systematic Internaliser (SI)
	#[serde(rename = "2")]
	CanBeTradedOnlyOnASystematicInternaliser,
	/// Can be traded on a trading venue or Systematic internaliser (SI)
	#[serde(rename = "3")]
	CanBeTradedOnATradingVenueOrSystematicInternaliser,
}

impl Default for ExDestinationType {
	fn default() -> Self {
		ExDestinationType::NoRestriction
	}
}
