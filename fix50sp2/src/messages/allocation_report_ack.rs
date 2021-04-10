
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Allocation {
	/// MsgType = AT
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// AllocReportID
	#[serde(rename = "755")]
	pub alloc_report_id: String,
	/// AllocID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "70")]
	pub alloc_id: Option<String>,
	/// May be used to link to a previously submitted AllocationInstructionAlertRequest(35=DU) message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2758")]
	pub alloc_request_id: Option<String>,
	/// Indicates Clearing Business Date for which transaction will be settled.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<crate::entities::LocalMktDate>,
	/// Indicates if an allocation is to be average priced. Is also used to indicate if average price allocation group is complete
	/// or incomplete.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "819")]
	pub avg_px_indicator: Option<AvgPxIndicator>,
	/// Quantity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "53")]
	pub quantity: Option<f64>,
	/// AllocTransType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "71")]
	pub alloc_trans_type: Option<AllocTransType>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Optional second identifier for the allocation report being acknowledged (need not be unique)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "793")]
	pub secondary_alloc_id: Option<String>,
	/// TradeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<crate::entities::LocalMktDate>,
	/// Date/Time Allocation Report Ack generated
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<crate::entities::UTCTimestamp>,
	/// Denotes the status of the allocation report; received (but not yet processed), rejected (at block or account level) or accepted
	/// (and processed). AllocStatus will be conditionally required in a 2-party model when used by a counterparty to convey a change
	/// in status. It will be optional in a 3-party model in which only the central counterparty may issue the status of an allocation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "87")]
	pub alloc_status: Option<AllocStatus>,
	/// Required for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> = 1 ( block level reject) and for <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus&nbsp;(87)</a> 2 (account level reject) if the individual accounts and reject reasons are not provided in this message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "88")]
	pub alloc_rej_code: Option<AllocRejCode>,
	/// AllocReportType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "794")]
	pub alloc_report_type: Option<AllocReportType>,
	/// Required if <a href="tag_794_AllocReportType.html" target="bottom">AllocReportType&nbsp;(794)</a> = 8 (Request to Intermediary) Indicates status that is requested to be transmitted to counterparty by the intermediary (i.e.
	/// clearing house)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "808")]
	pub alloc_intermed_req_type: Option<AllocIntermedReqType>,
	/// Denotes whether the financial details provided on the Allocation Instruction were successfully matched.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "573")]
	pub match_status: Option<MatchStatus>,
	/// Can include explanation for <a href="tag_88_AllocRejCode.html" target="bottom">AllocRejCode&nbsp;(88)</a> = 7 (other)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// This repeating group is optionally used for messages with <a href="tag_87_AllocStatus.html" target="bottom">AllocStatus (87)&nbsp;(87)</a> = 2 (account level reject) to provide details of the individual accounts that caused the rejection, together with reject reasons.
	/// This group should not be populated when AllocStatus has any other value. Indicates number of allocation groups to follow
	#[serde(flatten)]
	pub alloc_ack_grp: Option<super::super::alloc_ack_grp::AllocAckGrp>,
	/// <p>Firm assigned entity identifier for the allocation</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1728")]
	pub firm_group_id: Option<String>,
	/// <p>Group identifier assigned by the clearing house</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1730")]
	pub alloc_group_id: Option<String>,
	/// <p>Firm designated group identifier for average pricing</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1731")]
	pub avg_px_group_id: Option<String>,
	/// CustOrderHandlingInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1031")]
	pub cust_order_handling_inst: Option<crate::entities::SeparatedValues<CustOrderHandlingInst>>,
	/// OrderHandlingInstSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1032")]
	pub order_handling_inst_source: Option<OrderHandlingInstSource>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if <a href="tag_1665_EncodedRejectText.html" target="bottom">EncodedRejectTextLen(1665)&nbsp;(1665)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1664")]
	pub encoded_reject_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_1328_RejectText.html" target="bottom">RejectedText(1328)&nbsp;(1328)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1665")]
	pub encoded_reject_text: Option<String>,
	/// RegulatoryTradeIDGrp
	#[serde(flatten)]
	pub regulatory_trade_id_grp: Option<super::super::regulatory_trade_id_grp::RegulatoryTradeIDGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AvgPxIndicator {
	/// No Average Pricing
	#[serde(rename = "0")]
	NoAveragePricing,
	/// Trade is part of an average price group identified by the AvgPxGroupID(1731)
	#[serde(rename = "1")]
	TradeIsPartOfAnAveragePriceGroupIdentifiedByTheAvgPxGroupId,
	/// Last trade of the average price group identified by the AvgPxGroupID(1731)
	#[serde(rename = "2")]
	LastTradeOfTheAveragePriceGroupIdentifiedByTheAvgPxGroupId,
	/// Trade is part of a notional value average price group
	#[serde(rename = "3")]
	TradeIsPartOfANotionalValueAveragePriceGroup,
	/// Trade is average priced
	#[serde(rename = "4")]
	TradeIsAveragePriced,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocTransType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Replace
	#[serde(rename = "1")]
	Replace,
	/// Cancel
	#[serde(rename = "2")]
	Cancel,
	/// Preliminary (without MiscFees and NetMoney) (Removed/Replaced)
	#[serde(rename = "3")]
	Preliminary,
	/// Calculated (includes MiscFees and NetMoney) (Removed/Replaced)
	#[serde(rename = "4")]
	Calculated,
	/// Calculated without Preliminary (sent unsolicited by broker, includes MiscFees and NetMoney) (Removed/Replaced)
	#[serde(rename = "5")]
	CalculatedWithoutPreliminary,
	/// Reversal
	#[serde(rename = "6")]
	Reversal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocStatus {
	/// accepted (successfully processed)
	#[serde(rename = "0")]
	Accepted,
	/// block level reject
	#[serde(rename = "1")]
	BlockLevelReject,
	/// account level reject
	#[serde(rename = "2")]
	AccountLevelReject,
	/// received (received, not yet processed)
	#[serde(rename = "3")]
	Received,
	/// incomplete
	#[serde(rename = "4")]
	Incomplete,
	/// rejected by intermediary
	#[serde(rename = "5")]
	RejectedByIntermediary,
	/// allocation pending
	#[serde(rename = "6")]
	AllocationPending,
	/// reversed
	#[serde(rename = "7")]
	Reversed,
	/// Cancelled by intermediary
	#[serde(rename = "8")]
	CancelledByIntermediary,
	/// Claimed
	#[serde(rename = "9")]
	Claimed,
	/// Refused
	#[serde(rename = "10")]
	Refused,
	/// Pending give-up approval
	#[serde(rename = "11")]
	PendingGiveUpApproval,
	/// Cancelled
	#[serde(rename = "12")]
	Cancelled,
	/// Pending take-up approval
	#[serde(rename = "13")]
	PendingTakeUpApproval,
	/// Reversal pending
	#[serde(rename = "14")]
	ReversalPending,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocRejCode {
	/// Unknown account(s)
	#[serde(rename = "0")]
	N0,
	/// Incorrect quantity
	#[serde(rename = "1")]
	N1,
	/// Incorrect averageg price
	#[serde(rename = "2")]
	N2,
	/// Unknown executing broker mnemonic
	#[serde(rename = "3")]
	N3,
	/// Commission difference
	#[serde(rename = "4")]
	N4,
	/// Unknown OrderID (37)
	#[serde(rename = "5")]
	N5,
	/// Unknown ListID (66)
	#[serde(rename = "6")]
	N6,
	/// Other (further in Text (58))
	#[serde(rename = "7")]
	N7,
	/// Incorrect allocated quantity
	#[serde(rename = "8")]
	N8,
	/// Calculation difference
	#[serde(rename = "9")]
	N9,
	/// Unknown or stale ExecID
	#[serde(rename = "10")]
	N10,
	/// Mismatched data
	#[serde(rename = "11")]
	N11,
	/// Unknown ClOrdID
	#[serde(rename = "12")]
	N12,
	/// Warehouse request rejected
	#[serde(rename = "13")]
	N13,
	/// Other
	#[serde(rename = "99")]
	N99,
	/// Duplicate or missing IndividualAllocId(467)
	#[serde(rename = "14")]
	N14,
	/// Trade not recognized
	#[serde(rename = "15")]
	N15,
	/// Trade previously allocated
	#[serde(rename = "16")]
	N16,
	/// Incorrect or missing instrument
	#[serde(rename = "17")]
	N17,
	/// Incorrect or missing settlement date
	#[serde(rename = "18")]
	N18,
	/// Incorrect or missing fund ID or fund name
	#[serde(rename = "19")]
	N19,
	/// Incorrect or missing settlement instructions
	#[serde(rename = "20")]
	N20,
	/// Incorrect or missing fees
	#[serde(rename = "21")]
	N21,
	/// Incorrect or missing tax
	#[serde(rename = "22")]
	N22,
	/// Unknown or missing party
	#[serde(rename = "23")]
	N23,
	/// Incorrect or missing side
	#[serde(rename = "24")]
	N24,
	/// Incorrect or missing net-money
	#[serde(rename = "25")]
	N25,
	/// Incorrect or missing trade date
	#[serde(rename = "26")]
	N26,
	/// Incorrect or missing settlement currency instructions
	#[serde(rename = "27")]
	N27,
	/// Incorrect or missing ProcessCode
	#[serde(rename = "28")]
	N28,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocReportType {
	/// Preliminary Request to Intermediary
	#[serde(rename = "2")]
	PreliminaryRequestToIntermediary,
	/// Sellside Calculated Using Preliminary (includes MiscFees and NetMoney)
	#[serde(rename = "3")]
	SellsideCalculatedUsingPreliminary,
	/// Sellside Calculated Without Preliminary (sent unsolicited by sellside, includes MiscFees and NetMoney)
	#[serde(rename = "4")]
	SellsideCalculatedWithoutPreliminary,
	/// Warehouse Recap
	#[serde(rename = "5")]
	WarehouseRecap,
	/// Request to Intermediary
	#[serde(rename = "8")]
	RequestToIntermediary,
	/// Accept
	#[serde(rename = "9")]
	Accept,
	/// Reject
	#[serde(rename = "10")]
	Reject,
	/// Accept Pending
	#[serde(rename = "11")]
	AcceptPending,
	/// Complete
	#[serde(rename = "12")]
	Complete,
	/// Reverse Pending
	#[serde(rename = "14")]
	ReversePending,
	/// Give-up
	#[serde(rename = "15")]
	GiveUp,
	/// Take-up
	#[serde(rename = "16")]
	TakeUp,
	/// Reversal
	#[serde(rename = "17")]
	Reversal,
	/// Alleged reversal
	#[serde(rename = "18")]
	AllegedReversal,
	/// Sub-allocation giveup
	#[serde(rename = "19")]
	SubAllocationGiveup,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AllocIntermedReqType {
	/// Pending Accept
	#[serde(rename = "1")]
	PendingAccept,
	/// Pending Release
	#[serde(rename = "2")]
	PendingRelease,
	/// Pending Reversal
	#[serde(rename = "3")]
	PendingReversal,
	/// Accept
	#[serde(rename = "4")]
	Accept,
	/// Block Level Reject
	#[serde(rename = "5")]
	BlockLevelReject,
	/// Account Level Reject
	#[serde(rename = "6")]
	AccountLevelReject,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MatchStatus {
	/// Compared, matched or affirmed
	#[serde(rename = "0")]
	ComparedMatchedOrAffirmed,
	/// Uncompared, unmatched, or unaffirmed
	#[serde(rename = "1")]
	UncomparedUnmatchedOrUnaffirmed,
	/// Advisory or alert
	#[serde(rename = "2")]
	AdvisoryOrAlert,
	/// Mismatched (Indicates that data points from the AllocationInstruction(35=J) and Confirmation(35=AK) are matched but there
	/// are variances. MatchExceptionGrp component may be used to detail on the mis-matched data fields)
	#[serde(rename = "3")]
	MismatchedAndConfirmationAreMatchedButThereAreVariancesMatchExceptionGrpComponentMayBeUsedToDetailOnTheMisMatchedDataFields,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CustOrderHandlingInst {
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
	/// Phone simple
	#[serde(rename = "A")]
	PhoneSimple,
	/// Phone complex
	#[serde(rename = "B")]
	PhoneComplex,
	/// FCM provided screen
	#[serde(rename = "C")]
	FcmProvidedScreen,
	/// Other provided screen
	#[serde(rename = "D")]
	OtherProvidedScreen,
	/// Client provided platform controlled by FCM
	#[serde(rename = "E")]
	ClientProvidedPlatformControlledByFcm,
	/// Client provided platform direct to exchange
	#[serde(rename = "F")]
	ClientProvidedPlatformDirectToExchange,
	/// FCM API or FIX
	#[serde(rename = "G")]
	FcmApiOrFix,
	/// Algo engine
	#[serde(rename = "H")]
	AlgoEngine,
	/// Price at execution(price added at initial order entry, trading, middle office or time of give - up)
	#[serde(rename = "J")]
	PriceAtExecution,
	/// Desk - electronic
	#[serde(rename = "W")]
	DeskElectronic,
	/// Desk - pit
	#[serde(rename = "X")]
	DeskPit,
	/// Client - electronic
	#[serde(rename = "Y")]
	ClientElectronic,
	/// Client - pit
	#[serde(rename = "Z")]
	ClientPit,
	/// Conditional order
	#[serde(rename = "CND")]
	ConditionalOrder,
	/// Deliver instructions-cash
	#[serde(rename = "CSH")]
	DeliverInstructionsCash,
	/// Discretionary limit order
	#[serde(rename = "DLO")]
	DiscretionaryLimitOrder,
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
pub enum OrderHandlingInstSource {
	/// NASD OATS
	#[serde(rename = "1")]
	NasdOats,
	/// FIA Execution Source Code
	#[serde(rename = "2")]
	FiaExecutionSourceCode,
}
