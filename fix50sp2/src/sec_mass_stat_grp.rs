
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecMassStatGrp {
	/// Number of exceptions with a trading status different from SecurityMassTradingStatus (1679)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "146")]
	pub related_sym: Option<crate::entities::RepeatingValues<RelatedSy>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedSy {
	/// SecurityTradingStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "326")]
	pub security_trading_status: Option<SecurityTradingStatus>,
	/// SecurityTradingEvent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1174")]
	pub security_trading_event: Option<SecurityTradingEvent>,
	/// HaltReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "327")]
	pub halt_reason: Option<HaltReason>,
	/// FinancialStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "291")]
	pub financial_status: Option<crate::entities::SeparatedValues<FinancialStatus>>,
	/// CorporateAction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "292")]
	pub corporate_action: Option<crate::entities::SeparatedValues<CorporateAction>>,
	/// Comment, instructions, or other identifying information.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedText field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityTradingStatus {
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
	/// NoCancel
	#[serde(rename = "27")]
	NoCancel,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityTradingEvent {
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
	/// Corporate action
	#[serde(rename = "9")]
	CorporateAction,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HaltReason {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
