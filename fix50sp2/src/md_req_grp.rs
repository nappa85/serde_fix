
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDReqGrp {
	/// Number of MDEntryType fields requested.
	#[serde(rename = "267")]
	pub md_entry_types: crate::entities::RepeatingValues<MDEntryType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDEntryType {
	/// Must be the first field in this repeating group. This is a list of all the types of Market Data Entries that the firm requesting
	/// the Market Data is interested in receiving.
	#[serde(rename = "269")]
	pub md_entry_type_item: MDEntryTypeItem,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDEntryTypeItem {
	/// Bid
	#[serde(rename = "0")]
	Bid,
	/// Offer
	#[serde(rename = "1")]
	Offer,
	/// Trade
	#[serde(rename = "2")]
	Trade,
	/// Index Value
	#[serde(rename = "3")]
	IndexValue,
	/// Opening Price
	#[serde(rename = "4")]
	OpeningPrice,
	/// Closing Price
	#[serde(rename = "5")]
	ClosingPrice,
	/// Settlement Price
	#[serde(rename = "6")]
	SettlementPrice,
	/// Trading Session High Price
	#[serde(rename = "7")]
	TradingSessionHighPrice,
	/// Trading Session Low Price
	#[serde(rename = "8")]
	TradingSessionLowPrice,
	/// Trading Session VWAP Price
	#[serde(rename = "9")]
	TradingSessionVwapPrice,
	/// Imbalance
	#[serde(rename = "A")]
	Imbalance,
	/// Trade Volume
	#[serde(rename = "B")]
	TradeVolume,
	/// Open Interest
	#[serde(rename = "C")]
	OpenInterest,
	/// Composite Underlying Price
	#[serde(rename = "D")]
	CompositeUnderlyingPrice,
	/// Simulated Sell Price
	#[serde(rename = "E")]
	SimulatedSellPrice,
	/// Simulated Buy Price
	#[serde(rename = "F")]
	SimulatedBuyPrice,
	/// Margin Rate
	#[serde(rename = "G")]
	MarginRate,
	/// Mid Price
	#[serde(rename = "H")]
	MidPrice,
	/// Empty Book
	#[serde(rename = "J")]
	EmptyBook,
	/// Settle High Price
	#[serde(rename = "K")]
	SettleHighPrice,
	/// Settle Low Price
	#[serde(rename = "L")]
	SettleLowPrice,
	/// Prior Settle Price
	#[serde(rename = "M")]
	PriorSettlePrice,
	/// Session High Bid
	#[serde(rename = "N")]
	SessionHighBid,
	/// Session Low Offer
	#[serde(rename = "O")]
	SessionLowOffer,
	/// Early Prices
	#[serde(rename = "P")]
	EarlyPrices,
	/// Auction Clearing Price
	#[serde(rename = "Q")]
	AuctionClearingPrice,
	/// Swap Value Factor (SVF) for swaps cleared through a central counterparty (CCP)
	#[serde(rename = "S")]
	SwapValueFactorForSwapsClearedThroughACentralCounterparty,
	/// Value adjustment for long positions
	#[serde(rename = "R")]
	ValueAdjustmentForLongPositions,
	/// Cumulative Value Adjustment for long positions
	#[serde(rename = "T")]
	CumulativeValueAdjustmentForLongPositions,
	/// Daily Value Adjustment for Short Positions
	#[serde(rename = "U")]
	DailyValueAdjustmentForShortPositions,
	/// Cumulative Value Adjustment for Short Positions
	#[serde(rename = "V")]
	CumulativeValueAdjustmentForShortPositions,
	/// Fixing Price
	#[serde(rename = "W")]
	FixingPrice,
	/// Cash Rate
	#[serde(rename = "X")]
	CashRate,
	/// Recovery Rate
	#[serde(rename = "Y")]
	RecoveryRate,
	/// Recovery Rate for Long
	#[serde(rename = "Z")]
	RecoveryRateForLong,
	/// Recovery Rate for Short
	#[serde(rename = "a")]
	RecoveryRateForShort,
	/// Market Bid
	#[serde(rename = "b")]
	MarketBid,
	/// Market Offer
	#[serde(rename = "c")]
	MarketOffer,
	/// Short Sale Minimum Price
	#[serde(rename = "d")]
	ShortSaleMinimumPrice,
	/// Previous closing price
	#[serde(rename = "e")]
	PreviousClosingPrice,
}
