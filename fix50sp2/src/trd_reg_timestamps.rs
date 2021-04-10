
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdRegTimestamps {
	/// NoTrdRegTimestamps
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "768")]
	pub trd_reg_timestamps: Option<fix_common::RepeatingValues<TrdRegTimestamp>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdRegTimestamp {
	/// Required if <a href="tag_768_NoTrdRegTimestamps.html" target="bottom">NoTrdRegTimestamps&nbsp;(768)</a> &gt; 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "769")]
	pub trd_reg_timestamp: Option<fix_common::UTCTimestamp>,
	/// Required if <a href="tag_768_NoTrdRegTimestamps.html" target="bottom">NoTrdRegTimestamps&nbsp;(768)</a> &gt; 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "770")]
	pub trd_reg_timestamp_type: Option<TrdRegTimestampType>,
	/// TrdRegTimestampOrigin
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "771")]
	pub trd_reg_timestamp_origin: Option<String>,
	/// Type of Trading desk
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1033")]
	pub desk_type: Option<DeskType>,
	/// DeskTypeSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1034")]
	pub desk_type_source: Option<DeskTypeSource>,
	/// DeskOrderHandlingInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1035")]
	pub desk_order_handling_inst: Option<fix_common::SeparatedValues<DeskOrderHandlingInst>>,
	/// DeskType is required for OATS Reporting if InformationBarrierID(1727) is specified
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1727")]
	pub information_barrier_id: Option<String>,
	/// TrdRegTimestampManualIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2839")]
	pub trd_reg_timestamp_manual_indicator: Option<TrdRegTimestampManualIndicator>,
	/// May be used with TrdRegTimestampType(770)=34 (Reference time for NBBO).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2831")]
	pub nbbo_entry_type: Option<NBBOEntryType>,
	/// May be used with TrdRegTimestampType(770)=34 (Reference time for NBBO).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2832")]
	pub nbbo_price: Option<f64>,
	/// May be used with TrdRegTimestampType(770)=34 (Reference time for NBBO).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2833")]
	pub nbbo_qty: Option<f64>,
	/// May be used with TrdRegTimestampType(770)=34 (Reference time for NBBO).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2834")]
	pub nbbo_source: Option<NBBOSource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrdRegTimestampType {
	/// Execution Time
	#[serde(rename = "1")]
	ExecutionTime,
	/// Time In
	#[serde(rename = "2")]
	TimeIn,
	/// Time Out
	#[serde(rename = "3")]
	TimeOut,
	/// Broker Receipt
	#[serde(rename = "4")]
	BrokerReceipt,
	/// Broker Execution
	#[serde(rename = "5")]
	BrokerExecution,
	/// Desk Receipt
	#[serde(rename = "6")]
	DeskReceipt,
	/// Submission to clearing
	#[serde(rename = "7")]
	SubmissionToClearing,
	/// Time Priority
	#[serde(rename = "8")]
	TimePriority,
	/// Orderbook entry time
	#[serde(rename = "9")]
	OrderbookEntryTime,
	/// Order submission time
	#[serde(rename = "10")]
	OrderSubmissionTime,
	/// Publicly reported
	#[serde(rename = "11")]
	PubliclyReported,
	/// Public report updated
	#[serde(rename = "12")]
	PublicReportUpdated,
	/// Non-publicly reported
	#[serde(rename = "13")]
	NonPubliclyReported,
	/// Non-public report updated
	#[serde(rename = "14")]
	NonPublicReportUpdated,
	/// Submitted for confirmation
	#[serde(rename = "15")]
	SubmittedForConfirmation,
	/// Updated for confirmation
	#[serde(rename = "16")]
	UpdatedForConfirmation,
	/// Confirmed
	#[serde(rename = "17")]
	Confirmed,
	/// Updated for clearing
	#[serde(rename = "18")]
	UpdatedForClearing,
	/// Cleared
	#[serde(rename = "19")]
	Cleared,
	/// Allocations submitted
	#[serde(rename = "20")]
	AllocationsSubmitted,
	/// Allocations updated
	#[serde(rename = "21")]
	AllocationsUpdated,
	/// Allocations completed
	#[serde(rename = "22")]
	AllocationsCompleted,
	/// Submitted to repository
	#[serde(rename = "23")]
	SubmittedToRepository,
	/// Post-trade continuation event
	#[serde(rename = "24")]
	PostTradeContinuationEvent,
	/// Post-trade valuation
	#[serde(rename = "25")]
	PostTradeValuation,
	/// Previous time priority
	#[serde(rename = "26")]
	PreviousTimePriority,
	/// Identifier assigned
	#[serde(rename = "27")]
	IdentifierAssigned,
	/// Previous identifier assigned
	#[serde(rename = "28")]
	PreviousIdentifierAssigned,
	/// Order cancellation time
	#[serde(rename = "29")]
	OrderCancellationTime,
	/// Order modification time
	#[serde(rename = "30")]
	OrderModificationTime,
	/// Order routing time
	#[serde(rename = "31")]
	OrderRoutingTime,
	/// Trade cancellation time
	#[serde(rename = "32")]
	TradeCancellationTime,
	/// Trade modification time
	#[serde(rename = "33")]
	TradeModificationTime,
	/// Reference time for NBBO
	#[serde(rename = "34")]
	ReferenceTimeForNbbo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeskType {
	/// Agency
	#[serde(rename = "A")]
	Agency,
	/// Arbitrage
	#[serde(rename = "AR")]
	Arbitrage,
	/// Derivatives
	#[serde(rename = "D")]
	Derivatives,
	/// International
	#[serde(rename = "IN")]
	International,
	/// Institutional
	#[serde(rename = "IS")]
	Institutional,
	/// Other
	#[serde(rename = "O")]
	Other,
	/// Preferred Trading
	#[serde(rename = "PF")]
	PreferredTrading,
	/// Proprietary
	#[serde(rename = "PR")]
	Proprietary,
	/// Program Trading
	#[serde(rename = "PT")]
	ProgramTrading,
	/// Sales
	#[serde(rename = "S")]
	Sales,
	/// Trading
	#[serde(rename = "T")]
	Trading,
	/// Block trading
	#[serde(rename = "B")]
	BlockTrading,
	/// Convertible desk
	#[serde(rename = "C")]
	ConvertibleDesk,
	/// Central risk books
	#[serde(rename = "CR")]
	CentralRiskBooks,
	/// Equity capital markets
	#[serde(rename = "EC")]
	EquityCapitalMarkets,
	/// Swaps
	#[serde(rename = "SW")]
	Swaps,
	/// Treasury
	#[serde(rename = "TR")]
	Treasury,
	/// Floor Broker
	#[serde(rename = "FB")]
	FloorBroker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeskTypeSource {
	/// NASD OATS
	#[serde(rename = "1")]
	NasdOats,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeskOrderHandlingInst {
	/// Add-on Order
	#[serde(rename = "ADD")]
	AddOnOrder,
	/// All or None
	#[serde(rename = "AON")]
	AllOrNone,
	/// Cash Not Held
	#[serde(rename = "CNH")]
	CashNotHeld,
	/// Directed Order
	#[serde(rename = "DIR")]
	DirectedOrder,
	/// Exchange for Physical Transaction
	#[serde(rename = "E.W")]
	ExchangeForPhysicalTransaction,
	/// Fill or Kill
	#[serde(rename = "FOK")]
	FillOrKill,
	/// Imbalance Only
	#[serde(rename = "IO")]
	ImbalanceOnly,
	/// Immediate or Cancel
	#[serde(rename = "IOC")]
	ImmediateOrCancel,
	/// Limit On Open
	#[serde(rename = "LOO")]
	LimitOnOpen,
	/// Limit on Close
	#[serde(rename = "LOC")]
	LimitOnClose,
	/// Market at Open
	#[serde(rename = "MAO")]
	MarketAtOpen,
	/// Market at Close
	#[serde(rename = "MAC")]
	MarketAtClose,
	/// Market on Open
	#[serde(rename = "MOO")]
	MarketOnOpen,
	/// Market On Close
	#[serde(rename = "MOC")]
	MarketOnClose,
	/// Minimum Quantity
	#[serde(rename = "MQT")]
	MinimumQuantity,
	/// Not Held
	#[serde(rename = "NH")]
	NotHeld,
	/// Over the Day
	#[serde(rename = "OVD")]
	OverTheDay,
	/// Pegged
	#[serde(rename = "PEG")]
	Pegged,
	/// Reserve Size Order
	#[serde(rename = "RSV")]
	ReserveSizeOrder,
	/// Stop Stock Transaction
	#[serde(rename = "S.W")]
	StopStockTransaction,
	/// Scale
	#[serde(rename = "SCL")]
	Scale,
	/// Time Order
	#[serde(rename = "TMO")]
	TimeOrder,
	/// Trailing Stop
	#[serde(rename = "TS")]
	TrailingStop,
	/// Work
	#[serde(rename = "WRK")]
	Work,
	/// Conditional order
	#[serde(rename = "CND")]
	ConditionalOrder,
	/// Deliver instructions-cash
	#[serde(rename = "CSH")]
	DeliverInstructionsCash,
	/// Discretionary limit order
	#[serde(rename = "DLO")]
	DiscretionaryLimitOrder,
	/// G Order
	#[serde(rename = "G")]
	GOrder,
	/// Intra-day cross
	#[serde(rename = "IDX")]
	IntraDayCross,
	/// Intermarket sweep order
	#[serde(rename = "ISO")]
	IntermarketSweepOrder,
	/// Merger related transfer position
	#[serde(rename = "MPT")]
	MergerRelatedTransferPosition,
	/// Market to limit
	#[serde(rename = "MTL")]
	MarketToLimit,
	/// Delivery instructions - next day
	#[serde(rename = "ND")]
	DeliveryInstructionsNextDay,
	/// Options related transaction
	#[serde(rename = "OPT")]
	OptionsRelatedTransaction,
	/// Delivery instructions - seller's option
	#[serde(rename = "SLR")]
	DeliveryInstructionsSellerSOption,
	/// Stay on offerside
	#[serde(rename = "F0")]
	StayOnOfferside,
	/// Go along
	#[serde(rename = "F3")]
	GoAlong,
	/// Participate do not initiate
	#[serde(rename = "F6")]
	ParticipateDoNotInitiate,
	/// Strict scale
	#[serde(rename = "F7")]
	StrictScale,
	/// Try to scale
	#[serde(rename = "F8")]
	TryToScale,
	/// Stay on bidside
	#[serde(rename = "F9")]
	StayOnBidside,
	/// No Cross
	#[serde(rename = "FA")]
	NoCross,
	/// OK to Cross
	#[serde(rename = "FB")]
	OkToCross,
	/// Call first
	#[serde(rename = "FC")]
	CallFirst,
	/// Percent of volume
	#[serde(rename = "FD")]
	PercentOfVolume,
	/// Reinstate on system failure
	#[serde(rename = "FH")]
	ReinstateOnSystemFailure,
	/// Institution only
	#[serde(rename = "FI")]
	InstitutionOnly,
	/// Reinstate on trading halt
	#[serde(rename = "FJ")]
	ReinstateOnTradingHalt,
	/// Cancel on trading halt
	#[serde(rename = "FK")]
	CancelOnTradingHalt,
	/// Last peg
	#[serde(rename = "FL")]
	LastPeg,
	/// Mid-price peg
	#[serde(rename = "FM")]
	MidPricePeg,
	/// Non - negotiable
	#[serde(rename = "FN")]
	NonNegotiable,
	/// Opening peg
	#[serde(rename = "FO")]
	OpeningPeg,
	/// Market peg
	#[serde(rename = "FP")]
	MarketPeg,
	/// Cancel on system failure
	#[serde(rename = "FQ")]
	CancelOnSystemFailure,
	/// Primary peg
	#[serde(rename = "FR")]
	PrimaryPeg,
	/// Suspend
	#[serde(rename = "FS")]
	Suspend,
	/// Fixed peg to local best bid or offer at time of order
	#[serde(rename = "FT")]
	FixedPegToLocalBestBidOrOfferAtTimeOfOrder,
	/// Peg to VWAP
	#[serde(rename = "FW")]
	PegToVwap,
	/// Trade along
	#[serde(rename = "FX")]
	TradeAlong,
	/// Try to stop
	#[serde(rename = "FY")]
	TryToStop,
	/// Cancel if not best
	#[serde(rename = "FZ")]
	CancelIfNotBest,
	/// Strict limit
	#[serde(rename = "Fb")]
	StrictLimit,
	/// Ignore price validity checks
	#[serde(rename = "Fc")]
	IgnorePriceValidityChecks,
	/// Peg to limit price
	#[serde(rename = "Fd")]
	PegToLimitPrice,
	/// Work to target strategy
	#[serde(rename = "Fe")]
	WorkToTargetStrategy,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrdRegTimestampManualIndicator {
	/// Not manually captured
	#[serde(rename = "N")]
	NotManuallyCaptured,
	/// Manually captured
	#[serde(rename = "Y")]
	ManuallyCaptured,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NBBOEntryType {
	/// Bid
	#[serde(rename = "0")]
	Bid,
	/// Offer
	#[serde(rename = "1")]
	Offer,
	/// Mid-price
	#[serde(rename = "2")]
	MidPrice,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NBBOSource {
	/// Not applicable
	#[serde(rename = "0")]
	NotApplicable,
	/// Direct
	#[serde(rename = "1")]
	Direct,
	/// Securities Information Processor
	#[serde(rename = "2")]
	SecuritiesInformationProcessor,
	/// Hybrid
	#[serde(rename = "3")]
	Hybrid,
}
