
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDStatisticParameters {
	/// MDStatisticType
	#[serde(rename = "2456")]
	pub md_statistic_type: MDStatisticType,
	/// MDStatisticScope
	#[serde(rename = "2457")]
	pub md_statistic_scope: MDStatisticScope,
	/// MDStatisticSubScope
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2458")]
	pub md_statistic_sub_scope: Option<MDStatisticSubScope>,
	/// MDStatisticScopeType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2459")]
	pub md_statistic_scope_type: Option<MDStatisticScopeType>,
	/// MDStatisticName
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2454")]
	pub md_statistic_name: Option<String>,
	/// MDStatisticDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2455")]
	pub md_statistic_desc: Option<String>,
	/// Must be set if <a href="tag_2482_EncodedMDStatisticDesc.html" target="bottom">EncodedMDStatisticDesc(2482)&nbsp;(2482)</a> field is specified and must immediately precede it.
	#[serde(rename = "2481")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_2455_MDStatisticDesc.html" target="bottom">MDStatisticDesc(2455)&nbsp;(2455)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding(347)&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2482")]
	pub encoded_md_statistic_desc: Option<fix_common::EncodedText<2482>>,
	/// <p>May be used to specify the market depth up to specified level</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "264")]
	pub market_depth: Option<u32>,
	/// <p>Conditionally required when MDStatisticFrequencyPeriod(2461) is specified. Omission represents a one-time dissemination</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2460")]
	pub md_statistic_frequency_period: Option<i32>,
	/// <p>Conditionally required when MDStatisticFrequencyPeriod(2460) is specified</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2461")]
	pub md_statistic_frequency_unit: Option<MDStatisticFrequencyUnit>,
	/// <p>Conditionally required when MDStatisticDelayUnit(2463) is specified.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2462")]
	pub md_statistic_delay_period: Option<i32>,
	/// <p>Conditionally required when MDStatisticDelayPeriod(2462) is specified.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2463")]
	pub md_statistic_delay_unit: Option<MDStatisticDelayUnit>,
	/// MDStatisticIntervalType
	#[serde(rename = "2464")]
	pub md_statistic_interval_type: MDStatisticIntervalType,
	/// <p>Conditionally required for MDStatisticIntervalType (2464)= 1 (Sliding winndow) or 2 (Sliding window peak).</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2465")]
	pub md_statistic_interval_type_unit: Option<MDStatisticIntervalTypeUnit>,
	/// <p>Conditionally required when MDStatisticIntervalUnit(2467) is specified.</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2466")]
	pub md_statistic_interval_period: Option<i32>,
	/// <p>Conditionally required when MDStatisticIntervalPeriod(2466) is specified and MDStatisticIntervalType(2464) = 5(Current time
	/// unit), 6(Previous time unit) or 8(Maximum range up to previous time unit)
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2467")]
	pub md_statistic_interval_unit: Option<MDStatisticIntervalUnit>,
	/// <p>Can be used to define a date range for a sliding window peak other than the current day. Omission represents a date range
	/// starting with the first available day.
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2468")]
	pub md_statistic_start_date: Option<fix_common::UTCTimestamp>,
	/// <p>Can be used to define a date range for a sliding window peak other than the current day. Omission represents a date range
	/// including the current day
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2469")]
	pub md_statistic_end_date: Option<fix_common::UTCTimestamp>,
	/// <p>Can be used to define a date range for a sliding window peak other than the complete day. Omission represents a date range
	/// starting with the first available day.
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2470")]
	pub md_statistic_start_time: Option<fix_common::UTCTimeOnly>,
	/// <p>Can be used to define a date range for a sliding window peak other than the complete day. Omission represents a date range
	/// including the current day
	/// </p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2471")]
	pub md_statistic_end_time: Option<fix_common::UTCTimeOnly>,
	/// <p>Conditionally required when MDStatisticType(2456) = 5=(Ratio).</p>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2472")]
	pub md_statistic_ratio_type: Option<MDStatisticRatioType>,
	/// TradingCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1815")]
	pub trading_capacity: Option<TradingCapacity>,
	/// OrdType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40")]
	pub ord_type: Option<OrdType>,
	/// TimeInForce
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "59")]
	pub time_in_force: Option<TimeInForce>,
	/// QuoteCondition
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "276")]
	pub quote_condition: Option<fix_common::SeparatedValues<QuoteCondition>>,
	/// TradeCondition
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "277")]
	pub trade_condition: Option<fix_common::SeparatedValues<TradeCondition>>,
	/// Side
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "54")]
	pub side: Option<Side>,
	/// TradeInputSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "578")]
	pub trade_input_source: Option<String>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// MDOriginType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1024")]
	pub md_origin_type: Option<MDOriginType>,
	/// MDValueTier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2711")]
	pub md_value_tier: Option<MDValueTier>,
	/// TradSesMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "338")]
	pub trad_ses_method: Option<TradSesMethod>,
	/// MDFeedType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1022")]
	pub md_feed_type: Option<String>,
	/// ExposureDuration
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1629")]
	pub exposure_duration: Option<i32>,
	/// Conditionally required when ExposureDuration(1629) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1916")]
	pub exposure_duration_unit: Option<ExposureDurationUnit>,
	/// AggressorIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1057")]
	pub aggressor_indicator: Option<AggressorIndicator>,
	/// NestedParties
	#[serde(flatten)]
	pub nested_parties: Option<super::nested_parties::NestedParties>,
	/// AnnualTradingBusinessDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2584")]
	pub annual_trading_business_days: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticType {
	/// Count (Simple count of entities or events, e.g. orders transactions during a period of time)
	#[serde(rename = "1")]
	Count,
	/// Average volume (Average quantity of entities, e.g. average size of incoming quotes or average trade size)
	#[serde(rename = "2")]
	AverageVolume,
	/// Total volume (Aggregated volume of entities across events, e.g. total trade volume during a period of time)
	#[serde(rename = "3")]
	TotalVolume,
	/// Distribution (Distribution of entities across entity types, e.g. percentage of limit orders amongst all order types)
	#[serde(rename = "4")]
	Distribution,
	/// Ratio (Pre-defined ratio between entities, e.g. ratio of trades triggered by buy orders)
	#[serde(rename = "5")]
	Ratio,
	/// Liquidity (Measurement of liquidity of an instrument, e.g. by providing the spread between bid and offer or the trade volume
	/// needed to move the price)
	#[serde(rename = "6")]
	Liquidity,
	/// Volume weighted average price(VWAP)WAP (Benchmark price)
	#[serde(rename = "7")]
	VolumeWeightedAveragePriceWap,
	/// Volatility (Volatility of entities, e.g. price movements of incoming orders)
	#[serde(rename = "8")]
	Volatility,
	/// Duration (Time period of events, e.g. resting period of passive orders)
	#[serde(rename = "9")]
	Duration,
	/// Tick (Price movement of an instrument in number of ticks)
	#[serde(rename = "10")]
	Tick,
	/// Average value
	#[serde(rename = "11")]
	AverageValue,
	/// Total value
	#[serde(rename = "12")]
	TotalValue,
	/// High (Highest price)
	#[serde(rename = "13")]
	High,
	/// Low (Lowest price)
	#[serde(rename = "14")]
	Low,
	/// Midpoint (Midpoint price between bid and offer)
	#[serde(rename = "15")]
	Midpoint,
	/// First (First price or initial value)
	#[serde(rename = "16")]
	First,
	/// Last (Most recent price or value)
	#[serde(rename = "17")]
	Last,
	/// Final (Final price or confirmed value)
	#[serde(rename = "18")]
	Final,
	/// Exchange best (Best price of a single venue regardless of volume)
	#[serde(rename = "19")]
	ExchangeBest,
	/// Exchange best with volume (Best price of a single venue with volume at or above a pre-defined threshold)
	#[serde(rename = "20")]
	ExchangeBestWithVolume,
	/// Consolidated best (Best price across multiple venues regardless of volume)
	#[serde(rename = "21")]
	ConsolidatedBest,
	/// Consolidated best with volume (Best price across multiple venues with volume at or above a pre-defined threshold)
	#[serde(rename = "22")]
	ConsolidatedBestWithVolume,
	/// Time weighted average price (TWAP)
	#[serde(rename = "23")]
	TimeWeightedAveragePrice,
	/// Average duration
	#[serde(rename = "24")]
	AverageDuration,
	/// Average price
	#[serde(rename = "25")]
	AveragePrice,
	/// Total fees
	#[serde(rename = "26")]
	TotalFees,
	/// Total benefits
	#[serde(rename = "27")]
	TotalBenefits,
	/// Median value
	#[serde(rename = "28")]
	MedianValue,
	/// Average liquidity
	#[serde(rename = "29")]
	AverageLiquidity,
	/// Median duration
	#[serde(rename = "30")]
	MedianDuration,
}

impl Default for MDStatisticType {
	fn default() -> Self {
		MDStatisticType::Count
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticScope {
	/// Bid prices
	#[serde(rename = "1")]
	BidPrices,
	/// Offer prices
	#[serde(rename = "2")]
	OfferPrices,
	/// Bid depth
	#[serde(rename = "3")]
	BidDepth,
	/// Offer depth
	#[serde(rename = "4")]
	OfferDepth,
	/// Orders
	#[serde(rename = "5")]
	Orders,
	/// Quotes
	#[serde(rename = "6")]
	Quotes,
	/// Orders and Quotes
	#[serde(rename = "7")]
	OrdersAndQuotes,
	/// Trades
	#[serde(rename = "8")]
	Trades,
	/// Trade prices
	#[serde(rename = "9")]
	TradePrices,
	/// Auction prices
	#[serde(rename = "10")]
	AuctionPrices,
	/// Opening prices
	#[serde(rename = "11")]
	OpeningPrices,
	/// Closing prices
	#[serde(rename = "12")]
	ClosingPrices,
	/// Settlement prices
	#[serde(rename = "13")]
	SettlementPrices,
	/// Underlying prices
	#[serde(rename = "14")]
	UnderlyingPrices,
	/// Open interest
	#[serde(rename = "15")]
	OpenInterest,
	/// Index values
	#[serde(rename = "16")]
	IndexValues,
	/// Margin rates
	#[serde(rename = "17")]
	MarginRates,
	/// Outages
	#[serde(rename = "18")]
	Outages,
	/// Scheduled auctions
	#[serde(rename = "19")]
	ScheduledAuctions,
	/// Reference prices
	#[serde(rename = "20")]
	ReferencePrices,
	/// Trade value
	#[serde(rename = "21")]
	TradeValue,
	/// Market data fee items
	#[serde(rename = "22")]
	MarketDataFeeItems,
	/// Rebates
	#[serde(rename = "23")]
	Rebates,
	/// Discounts
	#[serde(rename = "24")]
	Discounts,
	/// Payments
	#[serde(rename = "25")]
	Payments,
	/// Taxes
	#[serde(rename = "26")]
	Taxes,
	/// Levies
	#[serde(rename = "27")]
	Levies,
	/// Benefits
	#[serde(rename = "28")]
	Benefits,
	/// Fees
	#[serde(rename = "29")]
	Fees,
	/// Orders and RFQs (Request for quotes)
	#[serde(rename = "30")]
	OrdersAndRfQs,
	/// Market makers
	#[serde(rename = "31")]
	MarketMakers,
	/// Trading interruptions
	#[serde(rename = "32")]
	TradingInterruptions,
	/// Trading suspensions
	#[serde(rename = "33")]
	TradingSuspensions,
	/// No quotes
	#[serde(rename = "34")]
	NoQuotes,
	/// Request for quotes
	#[serde(rename = "35")]
	RequestForQuotes,
	/// Trade volume
	#[serde(rename = "36")]
	TradeVolume,
}

impl Default for MDStatisticScope {
	fn default() -> Self {
		MDStatisticScope::BidPrices
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticSubScope {
	/// Visible (Only includes visible orders and/or quotes)
	#[serde(rename = "1")]
	Visible,
	/// Hidden (Only includes hidden orders and/or quotes)
	#[serde(rename = "2")]
	Hidden,
	/// Indicative (Only includes IOIs and nontradable quotes)
	#[serde(rename = "3")]
	Indicative,
	/// Tradeable (Excludes IOIs and indicative quotes)
	#[serde(rename = "4")]
	Tradeable,
	/// Passive ( Only includes resting orders and tradeable quotes)
	#[serde(rename = "5")]
	Passive,
	/// Market consensus (Only includes entities, e.g. trades, conforming to minimum requirements. Details to be defined out of band)
	#[serde(rename = "6")]
	MarketConsensus,
	/// Power
	#[serde(rename = "7")]
	Power,
	/// Hardware error
	#[serde(rename = "8")]
	HardwareError,
	/// Software error
	#[serde(rename = "9")]
	SoftwareError,
	/// Network error
	#[serde(rename = "10")]
	NetworkError,
	/// Failed
	#[serde(rename = "11")]
	Failed,
	/// Executed
	#[serde(rename = "12")]
	Executed,
	/// Entered
	#[serde(rename = "13")]
	Entered,
	/// Modified
	#[serde(rename = "14")]
	Modified,
	/// Cancelled
	#[serde(rename = "15")]
	Cancelled,
	/// Market data access
	#[serde(rename = "16")]
	MarketDataAccess,
	/// Terminal access
	#[serde(rename = "17")]
	TerminalAccess,
	/// Volume
	#[serde(rename = "18")]
	Volume,
	/// Cleared
	#[serde(rename = "19")]
	Cleared,
	/// Settled
	#[serde(rename = "20")]
	Settled,
	/// Other
	#[serde(rename = "21")]
	Other,
	/// Monetary
	#[serde(rename = "22")]
	Monetary,
	/// Non-monetary
	#[serde(rename = "23")]
	NonMonetary,
	/// Gross
	#[serde(rename = "24")]
	Gross,
	/// Large in scale
	#[serde(rename = "25")]
	LargeInScale,
	/// Neither hidden nor large in scale
	#[serde(rename = "26")]
	NeitherHiddenNorLargeInScale,
	/// Corporate action
	#[serde(rename = "27")]
	CorporateAction,
	/// Venue decision
	#[serde(rename = "28")]
	VenueDecision,
	/// Minimum time period
	#[serde(rename = "29")]
	MinimumTimePeriod,
	/// Open
	#[serde(rename = "30")]
	Open,
	/// Not executed
	#[serde(rename = "31")]
	NotExecuted,
	/// Aggressive
	#[serde(rename = "32")]
	Aggressive,
	/// Directed
	#[serde(rename = "33")]
	Directed,
}

impl Default for MDStatisticSubScope {
	fn default() -> Self {
		MDStatisticSubScope::Visible
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticScopeType {
	/// Entry rate
	#[serde(rename = "1")]
	EntryRate,
	/// Modification rate
	#[serde(rename = "2")]
	ModificationRate,
	/// Cancel rate
	#[serde(rename = "3")]
	CancelRate,
	/// Downward move
	#[serde(rename = "4")]
	DownwardMove,
	/// Upward move
	#[serde(rename = "5")]
	UpwardMove,
}

impl Default for MDStatisticScopeType {
	fn default() -> Self {
		MDStatisticScopeType::EntryRate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticFrequencyUnit {
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

impl Default for MDStatisticFrequencyUnit {
	fn default() -> Self {
		MDStatisticFrequencyUnit::Seconds
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticDelayUnit {
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

impl Default for MDStatisticDelayUnit {
	fn default() -> Self {
		MDStatisticDelayUnit::Seconds
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticIntervalType {
	/// Sliding window (Window is defined as an interval period up to the current time of dissemination, see MDStatisticIntervalPeriod
	/// (2466))
	#[serde(rename = "1")]
	SlidingWindow,
	/// Sliding window peak (Highest value of all sliding windows across date and/or time range. Omission of date/time range represents
	/// current day)
	#[serde(rename = "2")]
	SlidingWindowPeak,
	/// Fixed date range (Interval may be open ended on either side, see MDStatisticStartDate MDStatStartDate(2468) and MDStatisticEndDate(2469).
	/// Starting/ending time of date fields only apply to the first/last day of the date range. Additional time range may be defined
	/// with MDStatisticStartTime(2470) and MDStatisticEndTime(2471) and applies to every business day within date range, i.e. to
	/// define an identical time slice across days.)
	#[serde(rename = "3")]
	FixedDateRangeAndMdStatisticEndDateStartingEndingTimeOfDateFieldsOnlyApplyToTheFirstLastDayOfTheDateRangeAdditionalTimeRangeMayBeDefinedWithMdStatisticStartTimeAndMdStatisticEndTimeAndAppliesToEveryBusinessDayWithinDateRangeIEToDefineAnIdenticalTimeSliceAcrossDays,
	/// Fixed time range (Interval may be open ended on either side, see MDStatisticStartTime(2470) and MDStatisticEndTime(2471).)
	#[serde(rename = "4")]
	FixedTimeRangeAndMdStatisticEndTime,
	/// Current time unit (Relative time unit which has not ended yet, e.g. current day. Interval ends with the time of dissemination
	/// of the statistic. Requires the definition of an actual unit, see MDStatisticIntervalTypeUnit(2465))
	#[serde(rename = "5")]
	CurrentTimeUnit,
	/// Previous time unit (Relative time unit which has ended in the past. Requires the definition of an actual unit, see MDStatisticIntervalTypeUnit(2465))
	#[serde(rename = "6")]
	PreviousTimeUnit,
	/// Maximum range (Use to convey record values over the lifetime of the system or venue.)
	#[serde(rename = "7")]
	MaximumRange,
	/// Maximum range up to previous time unit (Use to convey record values over the lifetime of the system or venue but does not
	/// include the most recent time unit as it has not completed yet. Requires the definition of an actual unit, see MDStatisticIntervalTypeUnit(2465))
	#[serde(rename = "8")]
	MaximumRangeUpToPreviousTimeUnit,
}

impl Default for MDStatisticIntervalType {
	fn default() -> Self {
		MDStatisticIntervalType::SlidingWindow
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticIntervalTypeUnit {
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

impl Default for MDStatisticIntervalTypeUnit {
	fn default() -> Self {
		MDStatisticIntervalTypeUnit::Seconds
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticIntervalUnit {
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

impl Default for MDStatisticIntervalUnit {
	fn default() -> Self {
		MDStatisticIntervalUnit::Seconds
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDStatisticRatioType {
	/// Buyers to sellers
	#[serde(rename = "1")]
	BuyersToSellers,
	/// Upticks to downticks (Can also be used with a scope of multiple instruments representing an index)
	#[serde(rename = "2")]
	UpticksToDownticks,
	/// Market maker to non-market maker (Use to identify share of market making activity)
	#[serde(rename = "3")]
	MarketMakerToNonMarketMaker,
	/// Automated to non-automated (Use to identify ratio of orders and quotes resulting from automated trading)
	#[serde(rename = "4")]
	AutomatedToNonAutomated,
	/// Orders to trades (Use with scope of trades)
	#[serde(rename = "5")]
	OrdersToTrades,
	/// Quotes to trades (Use with scope of trades)
	#[serde(rename = "6")]
	QuotesToTrades,
	/// Orders and quotes to trades (Use with scope of trades)
	#[serde(rename = "7")]
	OrdersAndQuotesToTrades,
	/// Failed to total traded value
	#[serde(rename = "8")]
	FailedToTotalTradedValue,
	/// Benefits to total traded value
	#[serde(rename = "9")]
	BenefitsToTotalTradedValue,
	/// Fees to total traded value
	#[serde(rename = "10")]
	FeesToTotalTradedValue,
	/// Trade volume to total traded volume
	#[serde(rename = "11")]
	TradeVolumeToTotalTradedVolume,
	/// Orders to total number of orders
	#[serde(rename = "12")]
	OrdersToTotalNumberOfOrders,
}

impl Default for MDStatisticRatioType {
	fn default() -> Self {
		MDStatisticRatioType::BuyersToSellers
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for TradingCapacity {
	fn default() -> Self {
		TradingCapacity::Customer
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
pub enum QuoteCondition {
	/// Open / Active
	#[serde(rename = "A")]
	OpenActive,
	/// Closed / Inactive
	#[serde(rename = "B")]
	ClosedInactive,
	/// Exchange Best
	#[serde(rename = "C")]
	ExchangeBest,
	/// Consolidated Best
	#[serde(rename = "D")]
	ConsolidatedBest,
	/// Locked
	#[serde(rename = "E")]
	Locked,
	/// Crossed
	#[serde(rename = "F")]
	Crossed,
	/// Depth
	#[serde(rename = "G")]
	Depth,
	/// Fast Trading
	#[serde(rename = "H")]
	FastTrading,
	/// Non-Firm
	#[serde(rename = "I")]
	NonFirm,
	/// Manual/Slow Quote
	#[serde(rename = "L")]
	ManualSlowQuote,
	/// Outright Price
	#[serde(rename = "J")]
	OutrightPrice,
	/// Implied Price
	#[serde(rename = "K")]
	ImpliedPrice,
	/// Depth on Offer
	#[serde(rename = "M")]
	DepthOnOffer,
	/// Depth on Bid
	#[serde(rename = "N")]
	DepthOnBid,
	/// Closing
	#[serde(rename = "O")]
	Closing,
	/// News Dissemination
	#[serde(rename = "P")]
	NewsDissemination,
	/// Trading Range
	#[serde(rename = "Q")]
	TradingRange,
	/// Order Influx
	#[serde(rename = "R")]
	OrderInflux,
	/// Due to Related
	#[serde(rename = "S")]
	DueToRelated,
	/// News Pending
	#[serde(rename = "T")]
	NewsPending,
	/// Additional Info
	#[serde(rename = "U")]
	AdditionalInfo,
	/// Additional Info due to related
	#[serde(rename = "V")]
	AdditionalInfoDueToRelated,
	/// Resume
	#[serde(rename = "W")]
	Resume,
	/// View of Common
	#[serde(rename = "X")]
	ViewOfCommon,
	/// Volume Alert
	#[serde(rename = "Y")]
	VolumeAlert,
	/// Order Imbalance
	#[serde(rename = "Z")]
	OrderImbalance,
	/// Equipment Changeover
	#[serde(rename = "a")]
	EquipmentChangeover,
	/// No Open / No Resume
	#[serde(rename = "b")]
	NoOpenNoResume,
	/// Regular ETH
	#[serde(rename = "c")]
	RegularEth,
	/// Automatic Execution
	#[serde(rename = "d")]
	AutomaticExecution,
	/// Automatic Execution ETH
	#[serde(rename = "e")]
	AutomaticExecutionEth,
	/// Fast Market ETH
	#[serde(rename = "f")]
	FastMarketEth,
	/// Inactive ETH
	#[serde(rename = "g")]
	InactiveEth,
	/// Rotation
	#[serde(rename = "h")]
	Rotation,
	/// Rotation ETH
	#[serde(rename = "i")]
	RotationEth,
	/// Halt
	#[serde(rename = "j")]
	Halt,
	/// Halt ETH
	#[serde(rename = "k")]
	HaltEth,
	/// Due to News Dissemination
	#[serde(rename = "l")]
	DueToNewsDissemination,
	/// Due to News Pending
	#[serde(rename = "m")]
	DueToNewsPending,
	/// Trading Resume
	#[serde(rename = "n")]
	TradingResume,
	/// Out of Sequence
	#[serde(rename = "o")]
	OutOfSequence,
	/// Bid Specialist
	#[serde(rename = "p")]
	BidSpecialist,
	/// Offer Specialist
	#[serde(rename = "q")]
	OfferSpecialist,
	/// Bid Offer Specialist
	#[serde(rename = "r")]
	BidOfferSpecialist,
	/// End of Day SAM
	#[serde(rename = "s")]
	EndOfDaySam,
	/// Forbidden SAM
	#[serde(rename = "t")]
	ForbiddenSam,
	/// Frozen SAM
	#[serde(rename = "u")]
	FrozenSam,
	/// PreOpening SAM
	#[serde(rename = "v")]
	PreOpeningSam,
	/// Opening SAM
	#[serde(rename = "w")]
	OpeningSam,
	/// Open SAM
	#[serde(rename = "x")]
	OpenSam,
	/// Surveillance SAM
	#[serde(rename = "y")]
	SurveillanceSam,
	/// Suspended SAM
	#[serde(rename = "z")]
	SuspendedSam,
	/// Reserved SAM
	#[serde(rename = "0")]
	ReservedSam,
	/// No Active SAM
	#[serde(rename = "1")]
	NoActiveSam,
	/// Restricted
	#[serde(rename = "2")]
	Restricted,
	/// Rest of Book VWAP
	#[serde(rename = "3")]
	RestOfBookVwap,
	/// Better Prices in Conditional Orders
	#[serde(rename = "4")]
	BetterPricesInConditionalOrders,
	/// Median Price
	#[serde(rename = "5")]
	MedianPrice,
	/// Full Curve
	#[serde(rename = "6")]
	FullCurve,
	/// Flat Curve
	#[serde(rename = "7")]
	FlatCurve,
}

impl Default for QuoteCondition {
	fn default() -> Self {
		QuoteCondition::OpenActive
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TradeCondition {
	/// Implied Trade
	#[serde(rename = "1")]
	N1,
	/// Marketplace entered trade
	#[serde(rename = "2")]
	N2,
	/// Multi-asset class multileg trade
	#[serde(rename = "3")]
	N3,
	/// Multileg-to-Multileg Trade
	#[serde(rename = "4")]
	N4,
	/// Short Sale Minimum Price
	#[serde(rename = "5")]
	N5,
	/// Benchmark (Elaboration: Market Model Typology terminology: The "benchmark" price depends on a benchmark which has no current
	/// price but derived from a time series such as a VWAP)
	#[serde(rename = "6")]
	N6,
	/// Cash (only) Market
	#[serde(rename = "A")]
	A,
	/// Average Price Trade
	#[serde(rename = "B")]
	B,
	/// Cash Trade (same day clearing)
	#[serde(rename = "C")]
	C,
	/// Next Day (only) Market
	#[serde(rename = "D")]
	D,
	/// Opening / Reopening Trade Detail
	#[serde(rename = "E")]
	E,
	/// Intraday Trade Detail
	#[serde(rename = "F")]
	F,
	/// Rule 127 Trade (NYSE)
	#[serde(rename = "G")]
	G,
	/// Rule 155 Trade (Amex)
	#[serde(rename = "H")]
	H,
	/// Sold Last (late reporting)
	#[serde(rename = "I")]
	I,
	/// Next Day Trade (next day clearing)
	#[serde(rename = "J")]
	J,
	/// Opened (late report of opened trade)
	#[serde(rename = "K")]
	K,
	/// Seller
	#[serde(rename = "L")]
	L,
	/// Sold (out of sequence)
	#[serde(rename = "M")]
	M,
	/// Stopped Stock (guarantee of price but does not execute the order)
	#[serde(rename = "N")]
	N,
	/// Imbalance More Buyers (Cannot be used in combination with Q)
	#[serde(rename = "P")]
	P,
	/// Imbalance More Sellers (Cannot be used in combination with P)
	#[serde(rename = "Q")]
	Q,
	/// Opening Price
	#[serde(rename = "R")]
	R,
	/// Bargain Condition (LSE)
	#[serde(rename = "S")]
	S,
	/// Converted Price Indicator
	#[serde(rename = "T")]
	T,
	/// Exchange Last
	#[serde(rename = "U")]
	U,
	/// Final Price of Session
	#[serde(rename = "V")]
	V,
	/// Ex-pit
	#[serde(rename = "W")]
	W,
	/// Crossed
	#[serde(rename = "X")]
	X,
	/// Trades resulting from manual/slow quote
	#[serde(rename = "Y")]
	Y,
	/// Trades resulting from intermarket sweep
	#[serde(rename = "Z")]
	Z,
	/// Volume Only
	#[serde(rename = "a")]
	_A,
	/// Direct Plus
	#[serde(rename = "b")]
	_B,
	/// Acquisition
	#[serde(rename = "c")]
	_C,
	/// Bunched
	#[serde(rename = "d")]
	_D,
	/// Distribution
	#[serde(rename = "e")]
	_E,
	/// Bunched Sale
	#[serde(rename = "f")]
	_F,
	/// Split Trade
	#[serde(rename = "g")]
	_G,
	/// Cancel Stopped
	#[serde(rename = "h")]
	_H,
	/// Cancel ETH
	#[serde(rename = "i")]
	_I,
	/// Cancel Stopped ETH
	#[serde(rename = "j")]
	_J,
	/// Out of Sequence ETH
	#[serde(rename = "k")]
	_K,
	/// Cancel Last ETH
	#[serde(rename = "l")]
	_L,
	/// Sold Last Sale ETH
	#[serde(rename = "m")]
	_M,
	/// Cancel Last
	#[serde(rename = "n")]
	_N,
	/// Sold Last Sale
	#[serde(rename = "o")]
	_O,
	/// Cancel Open
	#[serde(rename = "p")]
	_P,
	/// Cancel Open ETH
	#[serde(rename = "q")]
	_Q,
	/// Opened Sale ETH
	#[serde(rename = "r")]
	_R,
	/// Cancel Only
	#[serde(rename = "s")]
	_S,
	/// Cancel Only ETH
	#[serde(rename = "t")]
	_T,
	/// Late Open ETH
	#[serde(rename = "u")]
	_U,
	/// Auto Execution ETH
	#[serde(rename = "v")]
	_V,
	/// Reopen
	#[serde(rename = "w")]
	_W,
	/// Reopen ETH
	#[serde(rename = "x")]
	_X,
	/// Adjusted
	#[serde(rename = "y")]
	_Y,
	/// Adjusted ETH
	#[serde(rename = "z")]
	_Z,
	/// Spread
	#[serde(rename = "AA")]
	Aa,
	/// Spread ETH
	#[serde(rename = "AB")]
	Ab,
	/// Straddle
	#[serde(rename = "AC")]
	Ac,
	/// Straddle ETH
	#[serde(rename = "AD")]
	Ad,
	/// Stopped
	#[serde(rename = "AE")]
	Ae,
	/// Stopped ETH
	#[serde(rename = "AF")]
	Af,
	/// Regular ETH
	#[serde(rename = "AG")]
	Ag,
	/// Combo
	#[serde(rename = "AH")]
	Ah,
	/// Combo ETH
	#[serde(rename = "AI")]
	Ai,
	/// Official Closing Price
	#[serde(rename = "AJ")]
	Aj,
	/// Prior Reference Price
	#[serde(rename = "AK")]
	Ak,
	/// Cancel
	#[serde(rename = "0")]
	N0,
	/// Stopped Sold Last
	#[serde(rename = "AL")]
	Al,
	/// Stopped Out of Sequence
	#[serde(rename = "AM")]
	Am,
	/// Offical Closing Price
	#[serde(rename = "AN")]
	An,
	/// Crossed
	#[serde(rename = "AO")]
	Ao,
	/// Fast Market
	#[serde(rename = "AP")]
	Ap,
	/// Automatic Execution
	#[serde(rename = "AQ")]
	Aq,
	/// Form T
	#[serde(rename = "AR")]
	Ar,
	/// Basket Index
	#[serde(rename = "AS")]
	As,
	/// Burst Basket
	#[serde(rename = "AT")]
	At,
	/// Outside Spread
	#[serde(rename = "AV")]
	Av,
	/// Trade through exempt ( Trade ignored prices on away markets)
	#[serde(rename = "AU")]
	Au,
	/// Last auction price (Trade represents outcome of last auction)
	#[serde(rename = "AW")]
	Aw,
	/// High price (Trade establishes new high price for the session)
	#[serde(rename = "AX")]
	Ax,
	/// Low price (Trade establishes new low price for the session)
	#[serde(rename = "AY")]
	Ay,
	/// Systematic Internaliser (SI)
	#[serde(rename = "AZ")]
	Az,
	/// Away market (Trade conducted on away market)
	#[serde(rename = "BA")]
	Ba,
	/// Mid-point price (Trade represents current midpoint price)
	#[serde(rename = "BB")]
	Bb,
	/// Traded before issue date (Trade conducted during subscription phase of new issue)
	#[serde(rename = "BC")]
	Bc,
	/// Previous closing price (Trade represents closing price of previous business day)
	#[serde(rename = "BD")]
	Bd,
	/// National Best Bid and Offer (Trade price within National Best Bid and Offer (NBBO))
	#[serde(rename = "BE")]
	Be,
}

impl Default for TradeCondition {
	fn default() -> Self {
		TradeCondition::N1
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
pub enum MDOriginType {
	/// Book
	#[serde(rename = "0")]
	Book,
	/// Off-Book
	#[serde(rename = "1")]
	OffBook,
	/// Cross
	#[serde(rename = "2")]
	Cross,
	/// Quote Driven Market (Elaboration: Examples for quote driven markets are market maker or specialist market models)
	#[serde(rename = "3")]
	QuoteDrivenMarket,
	/// Dark Order Book
	#[serde(rename = "4")]
	DarkOrderBook,
	/// Auction driven market
	#[serde(rename = "5")]
	AuctionDrivenMarket,
	/// Quote negotiation
	#[serde(rename = "6")]
	QuoteNegotiation,
	/// Voice negotiation
	#[serde(rename = "7")]
	VoiceNegotiation,
	/// Hybrid market
	#[serde(rename = "8")]
	HybridMarket,
}

impl Default for MDOriginType {
	fn default() -> Self {
		MDOriginType::Book
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDValueTier {
	/// Range 1
	#[serde(rename = "1")]
	Range1,
	/// Range 2
	#[serde(rename = "2")]
	Range2,
	/// Range 3
	#[serde(rename = "3")]
	Range3,
}

impl Default for MDValueTier {
	fn default() -> Self {
		MDValueTier::Range1
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
	/// Voice
	#[serde(rename = "4")]
	Voice,
}

impl Default for TradSesMethod {
	fn default() -> Self {
		TradSesMethod::Electronic
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
pub enum AggressorIndicator {
	/// Order initiator is aggressor
	#[serde(rename = "Y")]
	OrderInitiatorIsAggressor,
	/// Order initiator is passive
	#[serde(rename = "N")]
	OrderInitiatorIsPassive,
}

impl Default for AggressorIndicator {
	fn default() -> Self {
		AggressorIndicator::OrderInitiatorIsAggressor
	}
}
