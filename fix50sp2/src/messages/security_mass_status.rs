
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityMassStatus {
	/// MsgType = CO
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'C', 'O'>,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Required when mass status is in response to a SecurityMassStatusRequest (35=CN) messag
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "324")]
	pub security_status_req_id: Option<String>,
	/// Identifies all securities for a security list identifier.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1465")]
	pub security_list_id: Option<String>,
	/// Identifies all securities for a market.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// Identifies all securities for a market segment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Identifies all securities for a trading session.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// Identifies all securities for a trading subsession.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// InstrumentScope
	#[serde(flatten)]
	pub instrument_scope: Option<super::super::instrument_scope::InstrumentScope>,
	/// Set to 'Y' if message is sent as a result of a subscription request not a snapshot request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "325")]
	pub unsolicited_indicator: Option<UnsolicitedIndicator>,
	/// SecurityMassTradingStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1679")]
	pub security_mass_trading_status: Option<SecurityMassTradingStatus>,
	/// SecurityMassTradingEvent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1680")]
	pub security_mass_trading_event: Option<SecurityMassTradingEvent>,
	/// MassHaltReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1681")]
	pub mass_halt_reason: Option<MassHaltReason>,
	/// Used to relay changes in the book type.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1021")]
	pub md_book_type: Option<MDBookType>,
	/// Used to relay changes in Market Depth.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "264")]
	pub market_depth: Option<u32>,
	/// Time of state change for security list.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Adjustment
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "334")]
	pub adjustment: Option<Adjustment>,
	/// SecMassStatGrp
	#[serde(flatten)]
	pub sec_mass_stat_grp: Option<super::super::sec_mass_stat_grp::SecMassStatGrp>,
	/// Business day that the state change applies to.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// FastMarketIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2447")]
	pub fast_market_indicator: Option<fix_common::Boolean>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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
	/// Holiday
	#[serde(rename = "7")]
	Holiday,
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
pub enum SecurityMassTradingStatus {
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

impl Default for SecurityMassTradingStatus {
	fn default() -> Self {
		SecurityMassTradingStatus::OpeningDelay
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityMassTradingEvent {
	/// Order imbalance, auction is extended
	#[serde(rename = "1")]
	OrderImbalanceAuctionIsExtended,
	/// Trading resumes (after Halt)
	#[serde(rename = "2")]
	TradingResumes,
	/// Price Volatility Interruption
	#[serde(rename = "3")]
	PriceVolatilityInterruption,
	/// Change of Trading Session
	#[serde(rename = "4")]
	ChangeOfTradingSession,
	/// Change of Trading Subsession
	#[serde(rename = "5")]
	ChangeOfTradingSubsession,
	/// Change of Security Trading Status
	#[serde(rename = "6")]
	ChangeOfSecurityTradingStatus,
	/// Change of Book Type
	#[serde(rename = "7")]
	ChangeOfBookType,
	/// Change of Market Depth
	#[serde(rename = "8")]
	ChangeOfMarketDepth,
}

impl Default for SecurityMassTradingEvent {
	fn default() -> Self {
		SecurityMassTradingEvent::OrderImbalanceAuctionIsExtended
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MassHaltReason {
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

impl Default for MassHaltReason {
	fn default() -> Self {
		MassHaltReason::NewsDissemination
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Adjustment {
	/// Cancel
	#[serde(rename = "1")]
	Cancel,
	/// Error
	#[serde(rename = "2")]
	Error,
	/// Correction
	#[serde(rename = "3")]
	Correction,
}

impl Default for Adjustment {
	fn default() -> Self {
		Adjustment::Cancel
	}
}
