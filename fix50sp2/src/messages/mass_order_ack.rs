
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct MassOrderAck {
	/// MsgType = DK
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// For use in drop copy applications. NOT FOR USE in transactional applications.
	#[serde(flatten)]
	pub application_sequence_control: super::super::application_sequence_control::ApplicationSequenceControl,
	/// Unique identifier of MassOrder(35=DJ) message as assigned by the submitter of the request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2423")]
	pub mass_order_request_id: Option<String>,
	/// Unique identifier of MassOrder(35=DJ) message as assigned by the receiver
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2424")]
	pub mass_order_report_id: Option<String>,
	/// Message level request status
	#[serde(rename = "2425")]
	pub mass_order_request_status: MassOrderRequestStatus,
	/// Message level request result
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2426")]
	pub mass_order_request_result: Option<MassOrderRequestResult>,
	/// Level of response requested from receiver of MassOrder (35=DJ) message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2427")]
	pub order_response_level: Option<OrderResponseLevel>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// Must be set if EncodedRejectText(1665) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1664")]
	pub encoded_reject_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the RejectText(1328) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1665")]
	pub encoded_reject_text: Option<String>,
	/// MarketID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// TradingCapacity
	#[serde(rename = "1815")]
	pub trading_capacity: TradingCapacity,
	/// ClearingAccountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1816")]
	pub clearing_account_type: Option<ClearingAccountType>,
	/// Account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// AcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "660")]
	pub acct_id_source: Option<AcctIDSource>,
	/// AccountType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "581")]
	pub account_type: Option<AccountType>,
	/// OrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "528")]
	pub order_capacity: Option<OrderCapacity>,
	/// OrderRestrictions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "529")]
	pub order_restrictions: Option<fix_common::SeparatedValues<OrderRestrictions>>,
	/// CustOrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "582")]
	pub cust_order_capacity: Option<CustOrderCapacity>,
	/// ManualOrderIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1028")]
	pub manual_order_indicator: Option<fix_common::Boolean>,
	/// CustOrderHandlingInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1031")]
	pub cust_order_handling_inst: Option<fix_common::SeparatedValues<CustOrderHandlingInst>>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedRejectText(355) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the RejectText(1328) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// CopyMsgIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "797")]
	pub copy_msg_indicator: Option<fix_common::Boolean>,
	/// Used to support fragmentation. Sum of NoOrderEntries(2428) within the OrderEntryAckGrp across all messages with the same MassOrderRequestID(2423).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2432")]
	pub tot_no_order_entries: Option<i32>,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// ThrottleResponse
	#[serde(flatten)]
	pub throttle_response: Option<super::super::throttle_response::ThrottleResponse>,
	/// <p>Omit when the entire MassOrder(35=DJ) message is rejected or when no order entries are being rejected or accepted with additional
	/// events. Presence of this component is contingent upon the OrderResponseLevel(2427) value set by the order in the MassOrder(35=DJ)
	/// message
	/// </p>
	#[serde(flatten)]
	pub order_entry_ack_grp: Option<super::super::order_entry_ack_grp::OrderEntryAckGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MassOrderRequestStatus {
	/// Accepted
	#[serde(rename = "1")]
	Accepted,
	/// Accepted with additional events
	#[serde(rename = "2")]
	AcceptedWithAdditionalEvents,
	/// Rejected
	#[serde(rename = "3")]
	Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MassOrderRequestResult {
	/// Successful
	#[serde(rename = "0")]
	Successful,
	/// Response level not supported
	#[serde(rename = "1")]
	ResponseLevelNotSupported,
	/// Invalid market
	#[serde(rename = "2")]
	InvalidMarket,
	/// Invalid market segment
	#[serde(rename = "3")]
	InvalidMarketSegment,
	/// Other
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum OrderResponseLevel {
	/// No acknowledgement (Responses are provided through one or more ExecutionReport(35=8) messages)
	#[serde(rename = "0")]
	NoAcknowledgementMessages,
	/// Minimum acknowledgement (The minimum is any information to explain why the requested transaction was refused or led to additional
	/// events, e.g. immediate execution of an order that was entered or modified)
	#[serde(rename = "1")]
	MinimumAcknowledgement,
	/// Acknowledge each order (The number of entries in the response is identical to the number of entries in the request)
	#[serde(rename = "2")]
	AcknowledgeEachOrder,
	/// Summary acknowledgement (Responses are provided through a single MassOrderAck(35=DK) without entries and one or more ExecutionReport(35=8)
	/// messages)
	#[serde(rename = "3")]
	SummaryAcknowledgementWithoutEntriesAndOneOrMoreExecutionReportMessages,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum TradingCapacity {
	/// Customer
	#[serde(rename = "1")]
	Customer,
	/// Customer professional
	#[serde(rename = "2")]
	CustomerProfessional,
	/// Broker-dealer
	#[serde(rename = "3")]
	BrokerDealer,
	/// Customer broker-dealer
	#[serde(rename = "4")]
	CustomerBrokerDealer,
	/// Principal
	#[serde(rename = "5")]
	Principal,
	/// Market maker
	#[serde(rename = "6")]
	MarketMaker,
	/// Away market maker
	#[serde(rename = "7")]
	AwayMarketMaker,
	/// Systematic internaliser
	#[serde(rename = "8")]
	SystematicInternaliser,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ClearingAccountType {
	/// Customer
	#[serde(rename = "1")]
	Customer,
	/// Firm
	#[serde(rename = "2")]
	Firm,
	/// Market maker
	#[serde(rename = "3")]
	MarketMaker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum AcctIDSource {
	/// BIC
	#[serde(rename = "1")]
	Bic,
	/// SID code
	#[serde(rename = "2")]
	SidCode,
	/// TFM (GSPTA)
	#[serde(rename = "3")]
	Tfm,
	/// OMGEO (AlertID)
	#[serde(rename = "4")]
	Omgeo,
	/// DTCC code
	#[serde(rename = "5")]
	DtccCode,
	/// Special Segregated Account ID
	#[serde(rename = "6")]
	SpecialSegregatedAccountId,
	/// Other (custom or proprietary)
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum AccountType {
	/// Account is carried on customer Side of Books
	#[serde(rename = "1")]
	AccountIsCarriedOnCustomerSideOfBooks,
	/// Account is carried on non-Customer Side of books
	#[serde(rename = "2")]
	AccountIsCarriedOnNonCustomerSideOfBooks,
	/// House Trader
	#[serde(rename = "3")]
	HouseTrader,
	/// Floor Trader
	#[serde(rename = "4")]
	FloorTrader,
	/// Account is carried on non-customer side of books and is cross margined
	#[serde(rename = "6")]
	AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined,
	/// Account is house trader and is cross margined
	#[serde(rename = "7")]
	AccountIsHouseTraderAndIsCrossMargined,
	/// Joint Backoffice Account (JBO)
	#[serde(rename = "8")]
	JointBackofficeAccount,
	/// Equities specialist
	#[serde(rename = "9")]
	EquitiesSpecialist,
	/// Options market maker
	#[serde(rename = "10")]
	OptionsMarketMaker,
	/// Options firm account
	#[serde(rename = "11")]
	OptionsFirmAccount,
	/// Account for customer and non-customer orders
	#[serde(rename = "12")]
	AccountForCustomerAndNonCustomerOrders,
	/// Account for orders from multiple customers
	#[serde(rename = "13")]
	AccountForOrdersFromMultipleCustomers,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum CustOrderCapacity {
	/// Member trading for their own account
	#[serde(rename = "1")]
	MemberTradingForTheirOwnAccount,
	/// Clearing firm trading for its proprietary account
	#[serde(rename = "2")]
	ClearingFirmTradingForItsProprietaryAccount,
	/// Member trading for another member
	#[serde(rename = "3")]
	MemberTradingForAnotherMember,
	/// All other
	#[serde(rename = "4")]
	AllOther,
	/// Retail customer
	#[serde(rename = "5")]
	RetailCustomer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}
