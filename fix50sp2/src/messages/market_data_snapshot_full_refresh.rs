
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDataSnapshotFullRefresh {
	/// MsgType = W
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'W', ' '>,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Total number or reports returned in response to a request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "911")]
	pub tot_num_reports: Option<i32>,
	/// Unique identifier for the market data report.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "963")]
	pub md_report_id: Option<i32>,
	/// ClearingBusinessDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "715")]
	pub clearing_business_date: Option<fix_common::LocalMktDate>,
	/// Describes the type of book for which the feed is intended. Can be used when multiple feeds are provided over the same connection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1021")]
	pub md_book_type: Option<MDBookType>,
	/// Can be used to define a subordinate book.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1173")]
	pub md_sub_book_type: Option<i32>,
	/// Can be used to define the current depth of the book.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "264")]
	pub market_depth: Option<u32>,
	/// Describes a class of service for a given data feed, ie Regular and Market Maker
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1022")]
	pub md_feed_type: Option<String>,
	/// RefreshIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1187")]
	pub refresh_indicator: Option<RefreshIndicator>,
	/// Used to specify the trading date for which a set of market data applies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// Conditionally required if this message is in response to a MarketDataRequest(35=V).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "262")]
	pub md_req_id: Option<String>,
	/// MDStreamID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1500")]
	pub md_stream_id: Option<String>,
	/// Insert here the set of "Instrument" fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub instrument: super::super::instrument::Instrument,
	/// Number of underlyings
	#[serde(flatten)]
	pub und_instrmt_grp: Option<super::super::und_instrmt_grp::UndInstrmtGrp>,
	/// Required for multileg quotes
	#[serde(flatten)]
	pub instrmt_leg_grp: Option<super::super::instrmt_leg_grp::InstrmtLegGrp>,
	/// FinancialStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "291")]
	pub financial_status: Option<fix_common::SeparatedValues<FinancialStatus>>,
	/// CorporateAction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "292")]
	pub corporate_action: Option<fix_common::SeparatedValues<CorporateAction>>,
	/// NetChgPrevDay
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "451")]
	pub net_chg_prev_day: Option<f64>,
	/// Number of entries following.
	#[serde(flatten)]
	pub md_full_grp: super::super::md_full_grp::MDFullGrp,
	/// Depth of application messages queued for transmission as of delivery of this message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "813")]
	pub appl_queue_depth: Option<i32>,
	/// Action taken to resolve application queuing
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "814")]
	pub appl_queue_resolution: Option<ApplQueueResolution>,
	/// RoutingGrp
	#[serde(flatten)]
	pub routing_grp: Option<super::super::routing_grp::RoutingGrp>,
	/// MDSubFeedType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1683")]
	pub md_sub_feed_type: Option<String>,
	/// MarketID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// MDSecurityTradingStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1682")]
	pub md_security_trading_status: Option<MDSecurityTradingStatus>,
	/// MDHaltReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1684")]
	pub md_halt_reason: Option<MDHaltReason>,
	/// LastUpdateTime
	#[serde(rename = "779")]
	pub last_update_time: fix_common::UTCTimestamp,
	/// InstrumentExtension
	#[serde(flatten)]
	pub instrument_extension: Option<super::super::instrument_extension::InstrumentExtension>,
	/// FinancingDetails
	#[serde(flatten)]
	pub financing_details: Option<super::super::financing_details::FinancingDetails>,
	/// RelatedInstrumentGrp
	#[serde(flatten)]
	pub related_instrument_grp: Option<super::super::related_instrument_grp::RelatedInstrumentGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RefreshIndicator {
	/// Mandatory refresh by all participants
	#[serde(rename = "Y")]
	MandatoryRefreshByAllParticipants,
	/// Process as required
	#[serde(rename = "N")]
	ProcessAsRequired,
}

impl Default for RefreshIndicator {
	fn default() -> Self {
		RefreshIndicator::MandatoryRefreshByAllParticipants
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum FinancialStatus {
	/// Bankrupt
	#[serde(rename = "1")]
	Bankrupt,
	/// Pending delisting
	#[serde(rename = "2")]
	PendingDelisting,
	/// Restricted
	#[serde(rename = "3")]
	Restricted,
}

impl Default for FinancialStatus {
	fn default() -> Self {
		FinancialStatus::Bankrupt
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CorporateAction {
	/// Ex-Dividend
	#[serde(rename = "A")]
	ExDividend,
	/// Ex-Distribution
	#[serde(rename = "B")]
	ExDistribution,
	/// Ex-Rights
	#[serde(rename = "C")]
	ExRights,
	/// New
	#[serde(rename = "D")]
	New,
	/// Ex-Interest
	#[serde(rename = "E")]
	ExInterest,
	/// Cash Dividend
	#[serde(rename = "F")]
	CashDividend,
	/// Stock Dividend
	#[serde(rename = "G")]
	StockDividend,
	/// Non-Integer Stock Split
	#[serde(rename = "H")]
	NonIntegerStockSplit,
	/// Reverse Stock Split
	#[serde(rename = "I")]
	ReverseStockSplit,
	/// Standard-Integer Stock Split
	#[serde(rename = "J")]
	StandardIntegerStockSplit,
	/// Position Consolidation
	#[serde(rename = "K")]
	PositionConsolidation,
	/// Liquidation Reorganization
	#[serde(rename = "L")]
	LiquidationReorganization,
	/// Merger Reorganization
	#[serde(rename = "M")]
	MergerReorganization,
	/// Rights Offering
	#[serde(rename = "N")]
	RightsOffering,
	/// Shareholder Meeting
	#[serde(rename = "O")]
	ShareholderMeeting,
	/// Spinoff
	#[serde(rename = "P")]
	Spinoff,
	/// Tender Offer
	#[serde(rename = "Q")]
	TenderOffer,
	/// Warrant
	#[serde(rename = "R")]
	Warrant,
	/// Special Action
	#[serde(rename = "S")]
	SpecialAction,
	/// Symbol Conversion
	#[serde(rename = "T")]
	SymbolConversion,
	/// CUSIP / Name Change
	#[serde(rename = "U")]
	CusipNameChange,
	/// Leap Rollover
	#[serde(rename = "V")]
	LeapRollover,
	/// Succession Event
	#[serde(rename = "W")]
	SuccessionEvent,
}

impl Default for CorporateAction {
	fn default() -> Self {
		CorporateAction::ExDividend
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ApplQueueResolution {
	/// No action taken
	#[serde(rename = "0")]
	NoActionTaken,
	/// Queue Flushed
	#[serde(rename = "1")]
	QueueFlushed,
	/// Overlay Last
	#[serde(rename = "2")]
	OverlayLast,
	/// End Session
	#[serde(rename = "3")]
	EndSession,
}

impl Default for ApplQueueResolution {
	fn default() -> Self {
		ApplQueueResolution::NoActionTaken
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDSecurityTradingStatus {
	/// Opening delay
	#[serde(rename = "1")]
	OpeningDelay,
	/// Trading halt
	#[serde(rename = "2")]
	TradingHalt,
	/// Resume
	#[serde(rename = "3")]
	Resume,
	/// No Open / No Resume
	#[serde(rename = "4")]
	NoOpenNoResume,
	/// Price indication
	#[serde(rename = "5")]
	PriceIndication,
	/// Trading Range Indication
	#[serde(rename = "6")]
	TradingRangeIndication,
	/// Market Imbalance Buy
	#[serde(rename = "7")]
	MarketImbalanceBuy,
	/// Market Imbalance Sell
	#[serde(rename = "8")]
	MarketImbalanceSell,
	/// Market on Close Imbalance Buy
	#[serde(rename = "9")]
	MarketOnCloseImbalanceBuy,
	/// Market on Close Imbalance Sell
	#[serde(rename = "10")]
	MarketOnCloseImbalanceSell,
	/// (not assigned)
	#[serde(rename = "11")]
	NotAssigned,
	/// No Market Imbalance
	#[serde(rename = "12")]
	NoMarketImbalance,
	/// No Market On Close Imbalance
	#[serde(rename = "13")]
	NoMarketOnCloseImbalance,
	/// ITS Pre-Opening
	#[serde(rename = "14")]
	ItsPreOpening,
	/// New Price Indication
	#[serde(rename = "15")]
	NewPriceIndication,
	/// Trade Dissemination Time
	#[serde(rename = "16")]
	TradeDisseminationTime,
	/// Ready to trade (start of session)
	#[serde(rename = "17")]
	ReadyToTrade,
	/// Not available for trading (end of session)
	#[serde(rename = "18")]
	NotAvailableForTrading,
	/// Not traded on this market
	#[serde(rename = "19")]
	NotTradedOnThisMarket,
	/// Unknown or Invalid
	#[serde(rename = "20")]
	UnknownOrInvalid,
	/// Pre-open
	#[serde(rename = "21")]
	PreOpen,
	/// Opening Rotation
	#[serde(rename = "22")]
	OpeningRotation,
	/// Fast Market
	#[serde(rename = "23")]
	FastMarket,
	/// Pre-cross - system is in a pre-cross state allowing market to respond to either side of cross
	#[serde(rename = "24")]
	PreCrossSystemIsInAPreCrossStateAllowingMarketToRespondToEitherSideOfCross,
	/// Cross - system has crossed a percentage of the orders and allows market to rspond prior to crossing remaining portion
	#[serde(rename = "25")]
	CrossSystemHasCrossedAPercentageOfTheOrdersAndAllowsMarketToRspondPriorToCrossingRemainingPortion,
	/// Post-close
	#[serde(rename = "26")]
	PostClose,
}

impl Default for MDSecurityTradingStatus {
	fn default() -> Self {
		MDSecurityTradingStatus::OpeningDelay
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDHaltReason {
	/// News Dissemination
	#[serde(rename = "0")]
	NewsDissemination,
	/// Order Influx
	#[serde(rename = "1")]
	OrderInflux,
	/// Order Imbalance
	#[serde(rename = "2")]
	OrderImbalance,
	/// Additional Information
	#[serde(rename = "3")]
	AdditionalInformation,
	/// New Pending
	#[serde(rename = "4")]
	NewPending,
	/// Equipment Changeover
	#[serde(rename = "5")]
	EquipmentChangeover,
}

impl Default for MDHaltReason {
	fn default() -> Self {
		MDHaltReason::NewsDissemination
	}
}
