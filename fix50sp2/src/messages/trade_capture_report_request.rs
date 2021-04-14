
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradeCaptureReportRequest {
	/// MsgType = AD
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for the trade request
	#[serde(rename = "568")]
	pub trade_request_id: String,
	/// TradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1003")]
	pub trade_id: Option<String>,
	/// SecondaryTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1040")]
	pub secondary_trade_id: Option<String>,
	/// FirmTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1041")]
	pub firm_trade_id: Option<String>,
	/// SecondaryFirmTradeID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1042")]
	pub secondary_firm_trade_id: Option<String>,
	/// TradeRequestType
	#[serde(rename = "569")]
	pub trade_request_type: TradeRequestType,
	/// If the field is absent, SubscriptionRequestType(263)=0(Snapshot) will be the default
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// Can be used to request a specific trade report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "571")]
	pub trade_report_id: Option<String>,
	/// (Deprecated in FIX.5.0)To request a specific trade report
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "818")]
	pub secondary_trade_report_id: Option<String>,
	/// ExecID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "17")]
	pub exec_id: Option<String>,
	/// Can be used to requst all trades of a specific execution type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "150")]
	pub exec_type: Option<ExecType>,
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// ClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// MatchStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "573")]
	pub match_status: Option<MatchStatus>,
	/// Can be used to request all trades of a specific trade type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "828")]
	pub trd_type: Option<TrdType>,
	/// Can be used to request all trades of a specific trade sub type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "829")]
	pub trd_sub_type: Option<TrdSubType>,
	/// TradeHandlingInstr
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1123")]
	pub trade_handling_instr: Option<TradeHandlingInstr>,
	/// Can be used to request all trades for a specific transfer reason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "830")]
	pub transfer_reason: Option<String>,
	/// Can be used to request all trades of a specific secondary trade type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "855")]
	pub secondary_trd_type: Option<SecondaryTrdType>,
	/// Can be used to request all trades of a specific trade link identifier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "820")]
	pub trade_link_id: Option<String>,
	/// Can be used to request a trade matching a specific TrdMatchID (880)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "880")]
	pub trd_match_id: Option<String>,
	/// Used to specify the parties for the trades to be returned (clearing firm, execution broker, trader id, etc.): ExecutingBroker;
	/// ClearingFirm; ContraBroker; ContraClearingFirm; SettlementLocation - depository, CSD, or other settlement party; ExecutingTrader;
	/// InitiatingTrader; OrderOriginator.
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Instrument
	#[serde(flatten)]
	pub instrument: Option<super::super::instrument::Instrument>,
	/// InstrumentExtension
	#[serde(flatten)]
	pub instrument_extension: Option<super::super::instrument_extension::InstrumentExtension>,
	/// FinancingDetails
	#[serde(flatten)]
	pub financing_details: Option<super::super::financing_details::FinancingDetails>,
	/// UndInstrmtGrp
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// InstrmtLegGrp
	#[serde(flatten)]
	pub instrmt_leg_grp: Option<super::super::instrmt_leg_grp::InstrmtLegGrp>,
	/// Number of date ranges provided (must be 1 or 2 if specified)
	#[serde(flatten)]
	pub trd_cap_dt_grp: Option<super::super::trd_cap_dt_grp::TrdCapDtGrp>,
	/// Can be used to request trades for a specific clearing business date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// Can be used to request trades for a specific trading session.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// Can be used to request trades for a specific trading session.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// Can be used to request trades within a specific time bracket.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "943")]
	pub time_bracket: Option<String>,
	/// Can be used to request trades for a specific side of a trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// Used to indicate if trades are to be returned for the individual legs of a multileg instrument or for the overall instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "442")]
	pub multi_leg_reporting_type: Option<MultiLegReportingType>,
	/// Can be used to requests trades that were submitted from a specific trade input source.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "578")]
	pub trade_input_source: Option<String>,
	/// Can be used to request trades that were submitted from a specific trade input device.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "579")]
	pub trade_input_device: Option<String>,
	/// ResponseTransportType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "725")]
	pub response_transport_type: Option<ResponseTransportType>,
	/// ResponseDestination
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "726")]
	pub response_destination: Option<String>,
	/// Used to match specific values within <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> fields
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(rename = "354")]
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// MessageEventSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1011")]
	pub message_event_source: Option<String>,
	/// Allow requests based on secondary execution identifier.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "527")]
	pub secondary_exec_id: Option<String>,
	/// OffsetInstruction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1849")]
	pub offset_instruction: Option<OffsetInstruction>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeRequestType {
	/// All Trades
	#[serde(rename = "0")]
	AllTrades,
	/// Matched trades matching criteria provided on request (Parties, ExecID, TradeID, OrderID, Instrument, InputSource, etc.)
	#[serde(rename = "1")]
	MatchedTradesMatchingCriteriaProvidedOnRequest,
	/// Unmatched trades that match criteria
	#[serde(rename = "2")]
	UnmatchedTradesThatMatchCriteria,
	/// Unreported trades that match criteria
	#[serde(rename = "3")]
	UnreportedTradesThatMatchCriteria,
	/// Advisories that match criteria
	#[serde(rename = "4")]
	AdvisoriesThatMatchCriteria,
}

impl Default for TradeRequestType {
	fn default() -> Self {
		TradeRequestType::AllTrades
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SubscriptionRequestType {
	/// Snapshot
	#[serde(rename = "0")]
	Snapshot,
	/// Snapshot + Updates (Subscribe)
	#[serde(rename = "1")]
	SnapshotUpdates,
	/// Disable previous Snapshot + Update Request (Unsubscribe)
	#[serde(rename = "2")]
	DisablePreviousSnapshotUpdateRequest,
}

impl Default for SubscriptionRequestType {
	fn default() -> Self {
		SubscriptionRequestType::Snapshot
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecType {
	/// New
	#[serde(rename = "0")]
	New,
	/// Done for day
	#[serde(rename = "3")]
	DoneForDay,
	/// Canceled
	#[serde(rename = "4")]
	Canceled,
	/// Replaced
	#[serde(rename = "5")]
	Replaced,
	/// Pending Cancel (e.g. result of Order Cancel Request)
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
	/// Restated (Execution Report sent unsolicited by sellside, with <a href="tag_378_ExecRestatementReason.html" target="bottom">ExecRestatementReason&nbsp;(378)</a> set)
	#[serde(rename = "D")]
	RestatedASet,
	/// Pending Replace (e.g. result of <a href="message_Order_Cancel_Replace_Request_G.html" target="main">Order Cancel/Replace Request&nbsp;(G)</a> )
	#[serde(rename = "E")]
	PendingReplaceA,
	/// Trade (partial fill or fill)
	#[serde(rename = "F")]
	Trade,
	/// Trade Correct (formerly an ExecTransType)
	#[serde(rename = "G")]
	TradeCorrect,
	/// Trade Cancel (formerly an ExecTransType)
	#[serde(rename = "H")]
	TradeCancel,
	/// Order Status (formerly an ExecTransType)
	#[serde(rename = "I")]
	OrderStatus,
	/// Trade in a Clearing Hold
	#[serde(rename = "J")]
	TradeInAClearingHold,
	/// Trade has been released to Clearing
	#[serde(rename = "K")]
	TradeHasBeenReleasedToClearing,
	/// Triggered or Activated by System
	#[serde(rename = "L")]
	TriggeredOrActivatedBySystem,
	/// Locked
	#[serde(rename = "M")]
	Locked,
	/// Released
	#[serde(rename = "N")]
	Released,
}

impl Default for ExecType {
	fn default() -> Self {
		ExecType::New
	}
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

impl Default for MatchStatus {
	fn default() -> Self {
		MatchStatus::ComparedMatchedOrAffirmed
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrdType {
	/// Regular Trade
	#[serde(rename = "0")]
	N0,
	/// Block Trade
	#[serde(rename = "1")]
	N1,
	/// EFP (Exchange for physical)
	#[serde(rename = "2")]
	N2,
	/// Transfer
	#[serde(rename = "3")]
	N3,
	/// Late Trade
	#[serde(rename = "4")]
	N4,
	/// T Trade
	#[serde(rename = "5")]
	N5,
	/// Weighted Average Price Trade
	#[serde(rename = "6")]
	N6,
	/// Bunched Trade
	#[serde(rename = "7")]
	N7,
	/// Late Bunched Trade
	#[serde(rename = "8")]
	N8,
	/// Prior Reference Price Trade
	#[serde(rename = "9")]
	N9,
	/// After Hours Trade
	#[serde(rename = "10")]
	N10,
	/// Exchange for Risk (EFR)
	#[serde(rename = "11")]
	N11,
	/// Exchange for Swap (EFS )
	#[serde(rename = "12")]
	N12,
	/// Exchange of Futures for (in Market) Futures (EFM ) (e,g, full sized for mini)
	#[serde(rename = "13")]
	N13,
	/// Exchange of Options for Options (EOO)
	#[serde(rename = "14")]
	N14,
	/// Trading at Settlement
	#[serde(rename = "15")]
	N15,
	/// All or None
	#[serde(rename = "16")]
	N16,
	/// Futures Large Order Execution
	#[serde(rename = "17")]
	N17,
	/// Exchange of Futures for Futures (external market) (EFF)
	#[serde(rename = "18")]
	N18,
	/// Option Interim Trade
	#[serde(rename = "19")]
	N19,
	/// Option Cabinet Trade
	#[serde(rename = "20")]
	N20,
	/// Privately Negotiated Trades
	#[serde(rename = "22")]
	N22,
	/// Substitution of Futures for Forwards
	#[serde(rename = "23")]
	N23,
	/// Error trade
	#[serde(rename = "24")]
	N24,
	/// Special cum dividend (CD)
	#[serde(rename = "25")]
	N25,
	/// Special ex dividend (XD)
	#[serde(rename = "26")]
	N26,
	/// Special cum coupon (CC)
	#[serde(rename = "27")]
	N27,
	/// Special ex coupon (XC)
	#[serde(rename = "28")]
	N28,
	/// Cash settlement (CS)
	#[serde(rename = "29")]
	N29,
	/// Special price (usually net- or all-in price) (SP)
	#[serde(rename = "30")]
	N30,
	/// Guaranteed delivery (GD)
	#[serde(rename = "31")]
	N31,
	/// Special cum rights (CR)
	#[serde(rename = "32")]
	N32,
	/// Special ex rights (XR)
	#[serde(rename = "33")]
	N33,
	/// Special cum capital repayments (CP)
	#[serde(rename = "34")]
	N34,
	/// Special ex capital repayments (XP)
	#[serde(rename = "35")]
	N35,
	/// Special cum bonus (CB)
	#[serde(rename = "36")]
	N36,
	/// Special ex bonus (XB)
	#[serde(rename = "37")]
	N37,
	/// Block trade (same as large trade)
	#[serde(rename = "38")]
	N38,
	/// Worked principal trade (UK-specific)
	#[serde(rename = "39")]
	N39,
	/// Block Trades - after market
	#[serde(rename = "40")]
	N40,
	/// Name change
	#[serde(rename = "41")]
	N41,
	/// Portfolio transfer
	#[serde(rename = "42")]
	N42,
	/// Prorogation buy - Euronext Paris only. Is used to defer settlement under French SRD (deferred settlement system) . Trades
	/// must be reported as crosses at zero price
	#[serde(rename = "43")]
	N43,
	/// Prorogation sell - see prorogation buy
	#[serde(rename = "44")]
	N44,
	/// Option exercise
	#[serde(rename = "45")]
	N45,
	/// Delta neutral transaction
	#[serde(rename = "46")]
	N46,
	/// Financing transaction (includes repo and stock lending)
	#[serde(rename = "47")]
	N47,
	/// Non-standard settlement
	#[serde(rename = "48")]
	N48,
	/// Derivative related transaction
	#[serde(rename = "49")]
	N49,
	/// Portfolio trade
	#[serde(rename = "50")]
	N50,
	/// Volume weighted average trade
	#[serde(rename = "51")]
	N51,
	/// Exchange granted trade
	#[serde(rename = "52")]
	N52,
	/// Repurchase agreement
	#[serde(rename = "53")]
	N53,
	/// OTC
	#[serde(rename = "54")]
	N54,
	/// Exchange absis facility (EBF)
	#[serde(rename = "55")]
	N55,
	/// Opening Trade
	#[serde(rename = "56")]
	N56,
	/// Netted trade
	#[serde(rename = "57")]
	N57,
	/// Block swap trade or large notional off-facility swap
	#[serde(rename = "58")]
	N58,
	/// Give-up/Give-in trade
	#[serde(rename = "61")]
	N61,
	/// Dark trade (Elaboration: A Market Model Typology dark trade might also come from a lit/hybrid book, when an aggressive lit
	/// order hits a resting dark order.)
	#[serde(rename = "62")]
	N62,
	/// Technical trade
	#[serde(rename = "63")]
	N63,
	/// Benchmark
	#[serde(rename = "64")]
	N64,
	/// Credit event trade
	#[serde(rename = "59")]
	N59,
	/// Succession event trade
	#[serde(rename = "60")]
	N60,
	/// Package Trade (Identifies the pseudotrade of a stream or collection of trades to be cleared and be reported as an atomic unit.
	/// The subsequent actual trades reported should not have this value)
	#[serde(rename = "65")]
	N65,
	/// Roll trade
	#[serde(rename = "66")]
	N66,
}

impl Default for TrdType {
	fn default() -> Self {
		TrdType::N0
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TrdSubType {
	/// CMTA
	#[serde(rename = "0")]
	Cmta,
	/// Internal transfer or adjustment
	#[serde(rename = "1")]
	InternalTransferOrAdjustment,
	/// External transfer or transfer of account
	#[serde(rename = "2")]
	ExternalTransferOrTransferOfAccount,
	/// Reject for submitting side
	#[serde(rename = "3")]
	RejectForSubmittingSide,
	/// Advisory for contra side
	#[serde(rename = "4")]
	AdvisoryForContraSide,
	/// Offset due to an allocation
	#[serde(rename = "5")]
	OffsetDueToAnAllocation,
	/// Onset due to an allocation
	#[serde(rename = "6")]
	OnsetDueToAnAllocation,
	/// Differential Spread
	#[serde(rename = "7")]
	DifferentialSpread,
	/// Implied Spread leg executed against an outright
	#[serde(rename = "8")]
	ImpliedSpreadLegExecutedAgainstAnOutright,
	/// Transaction from exercise
	#[serde(rename = "9")]
	TransactionFromExercise,
	/// Transaction from assignment
	#[serde(rename = "10")]
	TransactionFromAssignment,
	/// ACATS
	#[serde(rename = "11")]
	Acats,
	/// AI (Automated input facility disabled in response to an exchange request.)
	#[serde(rename = "14")]
	Ai,
	/// B (Transaction between two member firms where neither member firm is registered as a market maker in the security in question
	/// and neither is a designated fund manager. Also used by broker dealers when dealing with another broker which is not a member
	/// firm. Non-order book securities only.)
	#[serde(rename = "15")]
	B,
	/// K (Transaction using block trade facility.)
	#[serde(rename = "16")]
	K,
	/// LC (Correction submitted more than three days after publication of the original trade report.)
	#[serde(rename = "17")]
	Lc,
	/// M (Transaction, other than a transaction resulting from a stock swap or stock switch, between two market makers registered
	/// in that security including IDB or a public display system trades. Non-order book securities only.)
	#[serde(rename = "18")]
	M,
	/// N (Non-protected portfolio transaction or a fully disclosed portfolio transaction)
	#[serde(rename = "19")]
	N,
	/// NM (i) transaction where Exchange has granted permission for non-publication (ii) IDB is reporting as seller (iii) submitting
	/// a transaction report to the Exchange, where the transaction report is not also a trade report.
	#[serde(rename = "20")]
	NmTransactionWhereExchangeHasGrantedPermissionForNonPublicationIdbIsReportingAsSellerSubmittingATransactionReportToTheExchangeWhereTheTransactionReportIsNotAlsoATradeReport,
	/// NR (Non-risk transaction in a SEATS security other than an AIM security)
	#[serde(rename = "21")]
	Nr,
	/// P (Protected portfolio transaction or a worked principal agreement to effect a portfolio transaction which includes order
	/// book securities)
	#[serde(rename = "22")]
	P,
	/// PA (Protected transaction notification)
	#[serde(rename = "23")]
	Pa,
	/// PC (Contra trade for transaction which took place on a previous day and which was automatically executed on the Exchange trading
	/// system)
	#[serde(rename = "24")]
	Pc,
	/// PN (Worked principal notification for a portfolio transaction which includes order book securities)
	#[serde(rename = "25")]
	Pn,
	/// R ((i) riskless principal transaction between non-members where the buying and selling transactions are executed at different
	/// prices or on different terms (requires a trade report with trade type indicator R for each transaction) (ii) market maker
	/// is reporting all the legs of a riskless principal transaction where the buying and selling transactions are executed at different
	/// prices (requires a trade report with trade type indicator R for each transaction)or (iii) market maker is reporting the onward
	/// leg of a riskless principal transaction where the legs are executed at different prices, and another market maker has submitted
	/// a trade report using trade type indicator M for the first leg (this requires a single trade report with trade type indicator
	/// R))
	#[serde(rename = "26")]
	RRisklessPrincipalTransactionBetweenNonMembersWhereTheBuyingAndSellingTransactionsAreExecutedAtDifferentPricesOrOnDifferentTermsMarketMakerIsReportingAllTheLegsOfARisklessPrincipalTransactionWhereTheBuyingAndSellingTransactionsAreExecutedAtDifferentPricesOrMarketMakerIsReportingTheOnwardLegOfARisklessPrincipalTransactionWhereTheLegsAreExecutedAtDifferentPricesAndAnotherMarketMakerHasSubmittedATradeReportUsingTradeTypeIndicatorMForTheFirstLeg,
	/// RO (Transaction which resulted from the exercise of a traditional option or a stock-settled covered warrant)
	#[serde(rename = "27")]
	Ro,
	/// RT (Risk transaction in a SEATS security, (excluding AIM security) reported by a market maker registered in that security)
	#[serde(rename = "28")]
	RtReportedByAMarketMakerRegisteredInThatSecurity,
	/// SW (Transactions resulting from stock swap or a stock switch (one report is required for each line of stock))
	#[serde(rename = "29")]
	Sw,
	/// T (If reporting a single protected transaction)
	#[serde(rename = "30")]
	T,
	/// WN (Worked principal notification for a single order book security)
	#[serde(rename = "31")]
	Wn,
	/// WT (Worked principal transaction (other than a portfolio transaction))
	#[serde(rename = "32")]
	Wt,
	/// Off Hours Trade
	#[serde(rename = "33")]
	OffHoursTrade,
	/// On Hours Trade
	#[serde(rename = "34")]
	OnHoursTrade,
	/// OTC Quote
	#[serde(rename = "35")]
	OtcQuote,
	/// Converted SWAP
	#[serde(rename = "36")]
	ConvertedSwap,
	/// Crossed Trade (X)
	#[serde(rename = "37")]
	CrossedTrade,
	/// Interim Protected Trade (I)
	#[serde(rename = "38")]
	InterimProtectedTrade,
	/// Large in Scale (L)
	#[serde(rename = "39")]
	LargeInScale,
	/// Wash Trade
	#[serde(rename = "40")]
	WashTrade,
	/// Trade at Settlement (TAS)
	#[serde(rename = "41")]
	TradeAtSettlement,
	/// Trade at Marker (TAM)
	#[serde(rename = "43")]
	TradeAtMarker,
	/// AuctionTrade
	#[serde(rename = "42")]
	AuctionTrade,
	/// Default (credit event)
	#[serde(rename = "44")]
	Default,
	/// Restructuring (credit event)
	#[serde(rename = "45")]
	Restructuring,
	/// Merger (succession event)
	#[serde(rename = "46")]
	Merger,
	/// Spin-off (succession event)
	#[serde(rename = "47")]
	SpinOff,
	/// Multilateral compression
	#[serde(rename = "48")]
	MultilateralCompression,
	/// Balancing
	#[serde(rename = "50")]
	Balancing,
	/// Basis Trade index Close (BTIC)
	#[serde(rename = "51")]
	BasisTradeIndexClose,
	/// Trade At Cash Open (TACO). The marketplace name given to trading futures based on an opening quote of the underlying cash
	/// market
	#[serde(rename = "52")]
	TradeAtCashOpenTheMarketplaceNameGivenToTradingFuturesBasedOnAnOpeningQuoteOfTheUnderlyingCashMarket,
}

impl Default for TrdSubType {
	fn default() -> Self {
		TrdSubType::Cmta
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradeHandlingInstr {
	/// Trade Confirmation
	#[serde(rename = "0")]
	TradeConfirmation,
	/// Two-Party Report
	#[serde(rename = "1")]
	TwoPartyReport,
	/// One-Party Report for Matching
	#[serde(rename = "2")]
	OnePartyReportForMatching,
	/// One-Party Report for Pass Through
	#[serde(rename = "3")]
	OnePartyReportForPassThrough,
	/// Automated Floor Order Routing
	#[serde(rename = "4")]
	AutomatedFloorOrderRouting,
	/// Two party report for claim
	#[serde(rename = "5")]
	TwoPartyReportForClaim,
	/// One-party report
	#[serde(rename = "6")]
	OnePartyReport,
	/// Third-party report for pass through
	#[serde(rename = "7")]
	ThirdPartyReportForPassThrough,
	/// One-party report for auto-match
	#[serde(rename = "8")]
	OnePartyReportForAutoMatch,
}

impl Default for TradeHandlingInstr {
	fn default() -> Self {
		TradeHandlingInstr::TradeConfirmation
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecondaryTrdType {
	/// Regular Trade
	#[serde(rename = "0")]
	N0,
	/// Block Trade
	#[serde(rename = "1")]
	N1,
	/// EFP (Exchange for physical)
	#[serde(rename = "2")]
	N2,
	/// Transfer
	#[serde(rename = "3")]
	N3,
	/// Late Trade
	#[serde(rename = "4")]
	N4,
	/// T Trade
	#[serde(rename = "5")]
	N5,
	/// Weighted Average Price Trade
	#[serde(rename = "6")]
	N6,
	/// Bunched Trade
	#[serde(rename = "7")]
	N7,
	/// Late Bunched Trade
	#[serde(rename = "8")]
	N8,
	/// Prior Reference Price Trade
	#[serde(rename = "9")]
	N9,
	/// After Hours Trade
	#[serde(rename = "10")]
	N10,
	/// Exchange for Risk (EFR)
	#[serde(rename = "11")]
	N11,
	/// Exchange for Swap (EFS )
	#[serde(rename = "12")]
	N12,
	/// Exchange of Futures for (in Market) Futures (EFM ) (e,g, full sized for mini)
	#[serde(rename = "13")]
	N13,
	/// Exchange of Options for Options (EOO)
	#[serde(rename = "14")]
	N14,
	/// Trading at Settlement
	#[serde(rename = "15")]
	N15,
	/// All or None
	#[serde(rename = "16")]
	N16,
	/// Futures Large Order Execution
	#[serde(rename = "17")]
	N17,
	/// Exchange of Futures for Futures (external market) (EFF)
	#[serde(rename = "18")]
	N18,
	/// Option Interim Trade
	#[serde(rename = "19")]
	N19,
	/// Option Cabinet Trade
	#[serde(rename = "20")]
	N20,
	/// Privately Negotiated Trades
	#[serde(rename = "22")]
	N22,
	/// Substitution of Futures for Forwards
	#[serde(rename = "23")]
	N23,
	/// Error trade
	#[serde(rename = "24")]
	N24,
	/// Special cum dividend (CD)
	#[serde(rename = "25")]
	N25,
	/// Special ex dividend (XD)
	#[serde(rename = "26")]
	N26,
	/// Special cum coupon (CC)
	#[serde(rename = "27")]
	N27,
	/// Special ex coupon (XC)
	#[serde(rename = "28")]
	N28,
	/// Cash settlement (CS)
	#[serde(rename = "29")]
	N29,
	/// Special price (usually net- or all-in price) (SP)
	#[serde(rename = "30")]
	N30,
	/// Guaranteed delivery (GD)
	#[serde(rename = "31")]
	N31,
	/// Special cum rights (CR)
	#[serde(rename = "32")]
	N32,
	/// Special ex rights (XR)
	#[serde(rename = "33")]
	N33,
	/// Special cum capital repayments (CP)
	#[serde(rename = "34")]
	N34,
	/// Special ex capital repayments (XP)
	#[serde(rename = "35")]
	N35,
	/// Special cum bonus (CB)
	#[serde(rename = "36")]
	N36,
	/// Special ex bonus (XB)
	#[serde(rename = "37")]
	N37,
	/// Block trade (same as large trade)
	#[serde(rename = "38")]
	N38,
	/// Worked principal trade (UK-specific)
	#[serde(rename = "39")]
	N39,
	/// Block Trades - after market
	#[serde(rename = "40")]
	N40,
	/// Name change
	#[serde(rename = "41")]
	N41,
	/// Portfolio transfer
	#[serde(rename = "42")]
	N42,
	/// Prorogation buy - Euronext Paris only. Is used to defer settlement under French SRD (deferred settlement system) . Trades
	/// must be reported as crosses at zero price
	#[serde(rename = "43")]
	N43,
	/// Prorogation sell - see prorogation buy
	#[serde(rename = "44")]
	N44,
	/// Option exercise
	#[serde(rename = "45")]
	N45,
	/// Delta neutral transaction
	#[serde(rename = "46")]
	N46,
	/// Financing transaction (includes repo and stock lending)
	#[serde(rename = "47")]
	N47,
	/// Non-standard settlement
	#[serde(rename = "48")]
	N48,
	/// Derivative related transaction
	#[serde(rename = "49")]
	N49,
	/// Portfolio trade
	#[serde(rename = "50")]
	N50,
	/// Volume weighted average trade
	#[serde(rename = "51")]
	N51,
	/// Exchange granted trade
	#[serde(rename = "52")]
	N52,
	/// Repurchase agreement
	#[serde(rename = "53")]
	N53,
	/// OTC
	#[serde(rename = "54")]
	N54,
	/// Exchange absis facility (EBF)
	#[serde(rename = "55")]
	N55,
	/// Opening Trade
	#[serde(rename = "56")]
	N56,
	/// Netted trade
	#[serde(rename = "57")]
	N57,
	/// Block swap trade or large notional off-facility swap
	#[serde(rename = "58")]
	N58,
	/// Give-up/Give-in trade
	#[serde(rename = "61")]
	N61,
	/// Dark trade (Elaboration: A Market Model Typology dark trade might also come from a lit/hybrid book, when an aggressive lit
	/// order hits a resting dark order.)
	#[serde(rename = "62")]
	N62,
	/// Technical trade
	#[serde(rename = "63")]
	N63,
	/// Benchmark
	#[serde(rename = "64")]
	N64,
	/// Credit event trade
	#[serde(rename = "59")]
	N59,
	/// Succession event trade
	#[serde(rename = "60")]
	N60,
	/// Package Trade (Identifies the pseudotrade of a stream or collection of trades to be cleared and be reported as an atomic unit.
	/// The subsequent actual trades reported should not have this value)
	#[serde(rename = "65")]
	N65,
	/// Roll trade
	#[serde(rename = "66")]
	N66,
}

impl Default for SecondaryTrdType {
	fn default() -> Self {
		SecondaryTrdType::N0
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
	/// Holiday
	#[serde(rename = "7")]
	Holiday,
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
	/// Sell undisclosed
	#[serde(rename = "H")]
	SellUndisclosed,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MultiLegReportingType {
	/// Single Security (default if not specified)
	#[serde(rename = "1")]
	SingleSecurity,
	/// Individual leg of a multi-leg security
	#[serde(rename = "2")]
	IndividualLegOfAMultiLegSecurity,
	/// Multi-leg security
	#[serde(rename = "3")]
	MultiLegSecurity,
}

impl Default for MultiLegReportingType {
	fn default() -> Self {
		MultiLegReportingType::SingleSecurity
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ResponseTransportType {
	/// In-band (default)
	#[serde(rename = "0")]
	InBand,
	/// Out of band
	#[serde(rename = "1")]
	OutOfBand,
}

impl Default for ResponseTransportType {
	fn default() -> Self {
		ResponseTransportType::InBand
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OffsetInstruction {
	/// Offset
	#[serde(rename = "0")]
	Offset,
	/// Onset
	#[serde(rename = "1")]
	Onset,
}

impl Default for OffsetInstruction {
	fn default() -> Self {
		OffsetInstruction::Offset
	}
}
