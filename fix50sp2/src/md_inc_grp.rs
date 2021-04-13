
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDIncGrp {
	/// Number of entries following.
	#[serde(rename = "268")]
	pub md_entries: fix_common::RepeatingValues<MDEntrie>,
	/// DiscountFactor
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1592")]
	pub discount_factor: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDEntrie {
	/// Must be first field in this repeating group.
	#[serde(rename = "279")]
	pub md_update_action: MDUpdateAction,
	/// (Deprecated in FIX.5.0)If MDUpdateAction = Delete(2), can be used to specify a reason for the deletion.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "285")]
	pub delete_reason: Option<DeleteReason>,
	/// Can be used to define a subordinate book.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1173")]
	pub md_sub_book_type: Option<i32>,
	/// Can be used to define the current depth of the book.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "264")]
	pub market_depth: Option<MarketDepth>,
	/// Conditionally required if <a href="tag_279_MDUpdateAction.html" target="bottom">MDUpdateAction&nbsp;(279)</a> = New(0). Cannot be changed.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "269")]
	pub md_entry_type: Option<MDEntryType>,
	/// If specified, must be unique among currently active entries if <a href="tag_279_MDUpdateAction.html" target="bottom">MDUpdateAction&nbsp;(279)</a> = New (0), must be the same as a previous <a href="tag_278_MDEntryID.html" target="bottom">MDEntryID&nbsp;(278)</a> if <a href="tag_279_MDUpdateAction.html" target="bottom">MDUpdateAction&nbsp;(279)</a> = Delete (2), and must be the same as a previous <a href="tag_278_MDEntryID.html" target="bottom">MDEntryID&nbsp;(278)</a> if <a href="tag_279_MDUpdateAction.html" target="bottom">MDUpdateAction&nbsp;(279)</a> = Change (1) and <a href="tag_280_MDEntryRefID.html" target="bottom">MDEntryRefID&nbsp;(280)</a> is not specified, or must be unique among currently active entries if <a href="tag_279_MDUpdateAction.html" target="bottom">MDUpdateAction&nbsp;(279)</a> = Change(1) and <a href="tag_280_MDEntryRefID.html" target="bottom">MDEntryRefID&nbsp;(280)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "278")]
	pub md_entry_id: Option<String>,
	/// If <a href="tag_279_MDUpdateAction.html" target="bottom">MDUpdateAction&nbsp;(279)</a> = New(0), for the first Market Data Entry in a message, either this field or a Symbol must be specified. If <a href="tag_279_MDUpdateAction.html" target="bottom">MDUpdateAction&nbsp;(279)</a> = Change(1), this must refer to a previous <a href="tag_278_MDEntryID.html" target="bottom">MDEntryID&nbsp;(278)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "280")]
	pub md_entry_ref_id: Option<String>,
	/// MDStreamID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1500")]
	pub md_stream_id: Option<String>,
	/// FinancialStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "291")]
	pub financial_status: Option<fix_common::SeparatedValues<FinancialStatus>>,
	/// CorporateAction
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "292")]
	pub corporate_action: Option<fix_common::SeparatedValues<CorporateAction>>,
	/// Conditionally required when <a href="tag_279_MDUpdateAction.html" target="bottom">MDUpdateAction&nbsp;(279)</a> = New(0) and <a href="tag_269_MDEntryType.html" target="bottom">MDEntryType&nbsp;(269)</a> is not Imbalance(A) ), Trade Volume (B), or Open Interest (C). Conditionally required when <a href="tag_269_MDEntryType.html" target="bottom">MDEntryType&nbsp;(269)</a> = "Auction Clearing Price"
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "270")]
	pub md_entry_px: Option<f64>,
	/// PriceType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "423")]
	pub price_type: Option<PriceType>,
	/// AvgPxIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "819")]
	pub avg_px_indicator: Option<AvgPxIndicator>,
	/// Used to support market mechanism type; limit order, market order, committed principal order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40")]
	pub ord_type: Option<OrdType>,
	/// Can be used to specify the currency of the quoted price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "15")]
	pub currency: Option<Currency>,
	/// Required for NDFs to specify the settlement currency (fixing currency).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "120")]
	pub settl_currency: Option<SettlCurrency>,
	/// Conditionally required when MDUpdateAction = New(0) and MDEntryType = Bid(0), Offer(1), Trade(2) , Trade Volume(B), or Open
	/// Interest(C).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "271")]
	pub md_entry_size: Option<f64>,
	/// Can be used to specify the lot type of the quoted size in order depth books.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1093")]
	pub lot_type: Option<LotType>,
	/// MDEntryDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "272")]
	pub md_entry_date: Option<fix_common::UTCDateOnly>,
	/// MDEntryTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "273")]
	pub md_entry_time: Option<fix_common::UTCTimeOnly>,
	/// TickDirection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "274")]
	pub tick_direction: Option<TickDirection>,
	/// (Deprecated in FIX.5.0)Market posting quote / trade. Valid values: See <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="../appendices/appendix_6-c.html" target="_blank">Volume 6: Appendix 6-C</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "275")]
	pub md_mkt: Option<String>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<TradingSessionID>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<TradingSessionSubID>,
	/// SecurityTradingStatus
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "326")]
	pub security_trading_status: Option<SecurityTradingStatus>,
	/// HaltReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "327")]
	pub halt_reason: Option<HaltReason>,
	/// Space-delimited list of conditions describing a quote.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "276")]
	pub quote_condition: Option<fix_common::SeparatedValues<QuoteCondition>>,
	/// Space-delimited list of conditions describing a trade
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "277")]
	pub trade_condition: Option<fix_common::SeparatedValues<TradeCondition>>,
	/// Use for MMT transaction category. Conditionally required in all MMT-supporting messages. For optional use in reporting Trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "828")]
	pub trd_type: Option<TrdType>,
	/// For optional use in reporting Trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "574")]
	pub match_type: Option<MatchType>,
	/// (Deprecated in FIX.5.0)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "282")]
	pub md_entry_originator: Option<String>,
	/// (Deprecated in FIX.5.0)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "283")]
	pub location_id: Option<String>,
	/// (Deprecated in FIX.5.0)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "284")]
	pub desk_id: Option<String>,
	/// Used if MDEntryType = Opening Price(4), Closing Price(5), or Settlement Price(6).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "286")]
	pub open_close_settl_flag: Option<fix_common::SeparatedValues<OpenCloseSettlFlag>>,
	/// For optional use when this Bid or Offer represents an order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "59")]
	pub time_in_force: Option<TimeInForce>,
	/// For optional use when this Bid or Offer represents an order. ExpireDate and ExpireTime cannot both be specified in one Market
	/// Data Entry.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "432")]
	pub expire_date: Option<fix_common::LocalMktDate>,
	/// For optional use when this Bid or Offer represents an order. ExpireDate and ExpireTime cannot both be specified in one Market
	/// Data Entry.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "126")]
	pub expire_time: Option<fix_common::UTCTimestamp>,
	/// For optional use when this Bid or Offer represents an order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "110")]
	pub min_qty: Option<f64>,
	/// Can contain multiple instructions, space delimited.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "18")]
	pub exec_inst: Option<fix_common::SeparatedValues<ExecInst>>,
	/// SellerDays
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "287")]
	pub seller_days: Option<i32>,
	/// For optional use when this Bid, Offer, or Trade represents an order
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<String>,
	/// For optional use to support Hit/Take (selecting a specific order from the feed) without disclosing a private order id.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// For optional use when this Bid, Offer, or Trade represents a quote
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "299")]
	pub quote_entry_id: Option<String>,
	/// For optional use in reporting Trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1003")]
	pub trade_id: Option<String>,
	/// For optional use in reporting Trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "288")]
	pub md_entry_buyer: Option<String>,
	/// For optional use in reporting Trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "289")]
	pub md_entry_seller: Option<String>,
	/// In an Aggregated Book, used to show how many individual orders make up an MDEntry
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "346")]
	pub number_of_orders: Option<i32>,
	/// Display position of a bid or offer, numbered from most competitive to least competitive, per market side, beginning with 1
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "290")]
	pub md_entry_position_no: Option<i32>,
	/// Scope
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "546")]
	pub scope: Option<fix_common::SeparatedValues<Scope>>,
	/// PriceDelta
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "811")]
	pub price_delta: Option<f64>,
	/// NetChgPrevDay
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "451")]
	pub net_chg_prev_day: Option<f64>,
	/// Text to describe the Market Data Entry. Part of repeating group.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if EncodedText field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
	/// MDPriceLevel
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1023")]
	pub md_price_level: Option<i32>,
	/// OrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "528")]
	pub order_capacity: Option<OrderCapacity>,
	/// MDOriginType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1024")]
	pub md_origin_type: Option<MDOriginType>,
	/// HighPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "332")]
	pub high_px: Option<f64>,
	/// LowPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "333")]
	pub low_px: Option<f64>,
	/// Indicates the last price of a trading session; can be a bid, ask, or a trade price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "31")]
	pub last_px: Option<f64>,
	/// TradeVolume
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1020")]
	pub trade_volume: Option<f64>,
	/// SettlType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "63")]
	pub settl_type: Option<SettlType>,
	/// Indicates date on which instrument will settle. For NDFs required for specifying the "value date".
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "64")]
	pub settl_date: Option<fix_common::LocalMktDate>,
	/// For optional use in reporting Trades. Used to specify the time of trade agreement for privately negotiated trades.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "483")]
	pub trans_bkd_time: Option<fix_common::UTCTimestamp>,
	/// For optional use in reporting Trades. Used to specify the time of matching.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// MDQuoteType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1070")]
	pub md_quote_type: Option<MDQuoteType>,
	/// Allows sequence number to be specified within a feed type
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "83")]
	pub rpt_seq: Option<i32>,
	/// DealingCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1048")]
	pub dealing_capacity: Option<DealingCapacity>,
	/// MDEntrySpotRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1026")]
	pub md_entry_spot_rate: Option<f64>,
	/// MDEntryForwardPoints
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1027")]
	pub md_entry_forward_points: Option<f64>,
	/// Conditionally required when TimeInForce(59) = 10 (Good for Time)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1629")]
	pub exposure_duration: Option<i32>,
	/// ExposureDurationUnit
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1916")]
	pub exposure_duration_unit: Option<ExposureDurationUnit>,
	/// Use for MMT crossing trade indicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "829")]
	pub trd_sub_type: Option<TrdSubType>,
	/// Use for MMT negotiated transaction indicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1115")]
	pub order_category: Option<OrderCategory>,
	/// Use for MMT publication mode
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1390")]
	pub trade_publish_indicator: Option<TradePublishIndicator>,
	/// ExecMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2405")]
	pub exec_method: Option<ExecMethod>,
	/// For optional use in reporting trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2449")]
	pub number_of_buy_orders: Option<i32>,
	/// For optional use in reporting trades
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2450")]
	pub number_of_sell_orders: Option<i32>,
	/// SettlPriceDeterminationMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2451")]
	pub settl_price_determination_method: Option<SettlPriceDeterminationMethod>,
	/// AggressorTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2445")]
	pub aggressor_time: Option<fix_common::UTCTimestamp>,
	/// AggressorSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2446")]
	pub aggressor_side: Option<AggressorSide>,
	/// FastMarketIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2447")]
	pub fast_market_indicator: Option<fix_common::Boolean>,
	/// For optional use in reporting Trades. May be used to link together trades that are reported separately but are part of the
	/// same overall trade, e.g. spread trade and their constituent trades.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1851")]
	pub strategy_link_id: Option<String>,
	/// AlgorithmicTradeIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2667")]
	pub algorithmic_trade_indicator: Option<AlgorithmicTradeIndicator>,
	/// Used only when reporting a trade (MDEntryType(269)=2 (Trade)) that is a regulatory trade report.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1934")]
	pub regulatory_report_type: Option<RegulatoryReportType>,
	/// PreviouslyReported
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "570")]
	pub previously_reported: Option<PreviouslyReported>,
	/// MarketCondition
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2705")]
	pub market_condition: Option<MarketCondition>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDUpdateAction {
	/// New
	#[serde(rename = "0")]
	New,
	/// Change
	#[serde(rename = "1")]
	Change,
	/// Delete
	#[serde(rename = "2")]
	Delete,
	/// Delete Thru
	#[serde(rename = "3")]
	DeleteThru,
	/// Delete From
	#[serde(rename = "4")]
	DeleteFrom,
	/// Overlay
	#[serde(rename = "5")]
	Overlay,
}

impl Default for MDUpdateAction {
	fn default() -> Self {
		MDUpdateAction::New
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeleteReason {
	/// Cancelation / Trade Bust
	#[serde(rename = "0")]
	CancelationTradeBust,
	/// Error
	#[serde(rename = "1")]
	Error,
}

impl Default for DeleteReason {
	fn default() -> Self {
		DeleteReason::CancelationTradeBust
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MarketDepth {
	/// full book depth
	#[serde(rename = "0")]
	FullBookDepth,
	/// top of book
	#[serde(rename = "1")]
	TopOfBook,
}

impl Default for MarketDepth {
	fn default() -> Self {
		MarketDepth::FullBookDepth
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDEntryType {
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

impl Default for MDEntryType {
	fn default() -> Self {
		MDEntryType::Bid
	}
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

impl Default for FinancialStatus {
	fn default() -> Self {
		FinancialStatus::Bankrupt
	}
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

impl Default for CorporateAction {
	fn default() -> Self {
		CorporateAction::ExDividend
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PriceType {
	/// Percentage (e.g. percent of par) (often called "dollar price" for fixed income)
	#[serde(rename = "1")]
	Percentage,
	/// Per unit (i.e. per share or contract)
	#[serde(rename = "2")]
	PerUnit,
	/// Fixed Amount (absolute value)
	#[serde(rename = "3")]
	FixedAmount,
	/// Discount - percentage points below par
	#[serde(rename = "4")]
	DiscountPercentagePointsBelowPar,
	/// Premium - percentage points over par
	#[serde(rename = "5")]
	PremiumPercentagePointsOverPar,
	/// Spread
	#[serde(rename = "6")]
	Spread,
	/// TED price
	#[serde(rename = "7")]
	TedPrice,
	/// TED yield
	#[serde(rename = "8")]
	TedYield,
	/// Yield
	#[serde(rename = "9")]
	Yield,
	/// Fixed cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "10")]
	FixedCabinetTradePrice,
	/// Variable cabinet trade price (primarily for listed futures and options)
	#[serde(rename = "11")]
	VariableCabinetTradePrice,
	/// Price spread
	#[serde(rename = "12")]
	PriceSpread,
	/// Product ticks in halves
	#[serde(rename = "13")]
	ProductTicksInHalves,
	/// Product ticks in fourths
	#[serde(rename = "14")]
	ProductTicksInFourths,
	/// Product ticks in eighths
	#[serde(rename = "15")]
	ProductTicksInEighths,
	/// Product ticks in sixteenths
	#[serde(rename = "16")]
	ProductTicksInSixteenths,
	/// Product ticks in thirty-seconds
	#[serde(rename = "17")]
	ProductTicksInThirtySeconds,
	/// Product ticks in sixty-fourths
	#[serde(rename = "18")]
	ProductTicksInSixtyFourths,
	/// Product ticks in one-twenty-eighths
	#[serde(rename = "19")]
	ProductTicksInOneTwentyEighths,
	/// Normal rate representation (e.g. FX rate) (represents the number of units of currency 2 that is required to purchase one unit
	/// of currency 1.)
	#[serde(rename = "20")]
	NormalRateRepresentation,
	/// Inverse rate representation (e.g. FX rate) (represents the number of units of 1 that is needed to purchase one unit of currency
	/// 2.)
	#[serde(rename = "21")]
	InverseRateRepresentation,
	/// Basis points (when the price is not spread based)
	#[serde(rename = "22")]
	BasisPoints,
	/// Upfront points (used specifically for CDS pricing)
	#[serde(rename = "23")]
	UpfrontPoints,
	/// Interest rate (Elaboration: When the price is an interest rate. For example, used with benchmark reference rate)
	#[serde(rename = "24")]
	InterestRate,
	/// Percentage of notional
	#[serde(rename = "25")]
	PercentageOfNotional,
}

impl Default for PriceType {
	fn default() -> Self {
		PriceType::Percentage
	}
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

impl Default for AvgPxIndicator {
	fn default() -> Self {
		AvgPxIndicator::NoAveragePricing
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Currency {
	/// Afghani
	#[serde(rename = "AFA")]
	Afa,
	/// Algerian Dinar
	#[serde(rename = "DZD")]
	Dzd,
	/// Andorran Peseta
	#[serde(rename = "ADP")]
	Adp,
	/// Argentine Peso
	#[serde(rename = "ARS")]
	Ars,
	/// Armenian Dram
	#[serde(rename = "AMD")]
	Amd,
	/// Aruban Guilder
	#[serde(rename = "AWG")]
	Awg,
	/// Australian Dollar
	#[serde(rename = "AUD")]
	Aud,
	/// Azerbaijanian Manat
	#[serde(rename = "AZM")]
	Azm,
	/// Bahamian Dollar
	#[serde(rename = "BSD")]
	Bsd,
	/// Bahraini Dinar
	#[serde(rename = "BHD")]
	Bhd,
	/// Baht
	#[serde(rename = "THB")]
	Thb,
	/// Balboa
	#[serde(rename = "PAB")]
	Pab,
	/// Barbados Dollar
	#[serde(rename = "BBD")]
	Bbd,
	/// Belarussian Ruble
	#[serde(rename = "BYB")]
	Byb,
	/// Belgian Franc
	#[serde(rename = "BEF")]
	Bef,
	/// Belize Dollar
	#[serde(rename = "BZD")]
	Bzd,
	/// Bermudian Dollar
	#[serde(rename = "BMD")]
	Bmd,
	/// Bolivar
	#[serde(rename = "VEB")]
	Veb,
	/// Boliviano
	#[serde(rename = "BOB")]
	Bob,
	/// Brazilian Real
	#[serde(rename = "BRL")]
	Brl,
	/// Brunei Dollar
	#[serde(rename = "BND")]
	Bnd,
	/// Burundi Franc
	#[serde(rename = "BIF")]
	Bif,
	/// CFA Franc BCEAO+
	#[serde(rename = "XOF")]
	Xof,
	/// CFA Franc BEAC#
	#[serde(rename = "XAF")]
	Xaf,
	/// CFP Franc
	#[serde(rename = "XPF")]
	Xpf,
	/// Canadian Dollar
	#[serde(rename = "CAD")]
	Cad,
	/// Cape Verde Escudo
	#[serde(rename = "CVE")]
	Cve,
	/// Cayman Islands Dollar
	#[serde(rename = "KYD")]
	Kyd,
	/// Cedi
	#[serde(rename = "GHC")]
	Ghc,
	/// Chilean Peso
	#[serde(rename = "CLP")]
	Clp,
	/// Colombian Peso
	#[serde(rename = "COP")]
	Cop,
	/// Comoro Franc
	#[serde(rename = "KMF")]
	Kmf,
	/// Convertible Marks
	#[serde(rename = "BAM")]
	Bam,
	/// Cordoba Oro
	#[serde(rename = "NIO")]
	Nio,
	/// Costa Rican Colon
	#[serde(rename = "CRC")]
	Crc,
	/// Cuban Peso
	#[serde(rename = "CUP")]
	Cup,
	/// Cyprus Pound
	#[serde(rename = "CYP")]
	Cyp,
	/// Czech Koruna
	#[serde(rename = "CZK")]
	Czk,
	/// Dalasi
	#[serde(rename = "GMD")]
	Gmd,
	/// Danish Krone
	#[serde(rename = "DKK")]
	Dkk,
	/// Denar
	#[serde(rename = "MKD")]
	Mkd,
	/// Deutsche Mark
	#[serde(rename = "DEM")]
	Dem,
	/// Djibouti Franc
	#[serde(rename = "DJF")]
	Djf,
	/// Dobra
	#[serde(rename = "STD")]
	Std,
	/// Dominican Peso
	#[serde(rename = "DOP")]
	Dop,
	/// Dong
	#[serde(rename = "VND")]
	Vnd,
	/// Drachma
	#[serde(rename = "GRD")]
	Grd,
	/// East Caribbean Dollar
	#[serde(rename = "XCD")]
	Xcd,
	/// Egyptian Pound
	#[serde(rename = "EGP")]
	Egp,
	/// El Salvador Colon
	#[serde(rename = "SVC")]
	Svc,
	/// Ethiopian Birr
	#[serde(rename = "ETB")]
	Etb,
	/// Euro
	#[serde(rename = "EUR")]
	Eur,
	/// Falkland Islands Pound
	#[serde(rename = "FKP")]
	Fkp,
	/// Fiji Dollar
	#[serde(rename = "FJD")]
	Fjd,
	/// Forint
	#[serde(rename = "HUF")]
	Huf,
	/// Franc Congolais
	#[serde(rename = "CDF")]
	Cdf,
	/// French Franc
	#[serde(rename = "FRF")]
	Frf,
	/// Gibraltar Pound
	#[serde(rename = "GIP")]
	Gip,
	/// Gourde
	#[serde(rename = "HTG")]
	Htg,
	/// Guarani
	#[serde(rename = "PYG")]
	Pyg,
	/// Guinea Franc
	#[serde(rename = "GNF")]
	Gnf,
	/// Guinea-Bissau Peso
	#[serde(rename = "GWP")]
	Gwp,
	/// Guyana Dollar
	#[serde(rename = "GYD")]
	Gyd,
	/// Hong Kong Dollar
	#[serde(rename = "HKD")]
	Hkd,
	/// Hryvnia
	#[serde(rename = "UAH")]
	Uah,
	/// Iceland Krona
	#[serde(rename = "ISK")]
	Isk,
	/// Indian Rupee
	#[serde(rename = "INR")]
	Inr,
	/// Iranian Rial
	#[serde(rename = "IRR")]
	Irr,
	/// Iraqi Dinar
	#[serde(rename = "IQD")]
	Iqd,
	/// Irish Pound
	#[serde(rename = "IEP")]
	Iep,
	/// Italian Lira
	#[serde(rename = "ITL")]
	Itl,
	/// Jamaican Dollar
	#[serde(rename = "JMD")]
	Jmd,
	/// Jordanian Dinar
	#[serde(rename = "JOD")]
	Jod,
	/// Kenyan Shilling
	#[serde(rename = "KES")]
	Kes,
	/// Kina
	#[serde(rename = "PGK")]
	Pgk,
	/// Kip
	#[serde(rename = "LAK")]
	Lak,
	/// Kroon
	#[serde(rename = "EEK")]
	Eek,
	/// Kuna
	#[serde(rename = "HRK")]
	Hrk,
	/// Kuwaiti Dinar
	#[serde(rename = "KWD")]
	Kwd,
	/// Kwacha
	#[serde(rename = "MWK")]
	Mwk,
	/// Kwacha
	#[serde(rename = "ZMK")]
	Zmk,
	/// Kwanza Reajustado
	#[serde(rename = "AOR")]
	Aor,
	/// Kyat
	#[serde(rename = "MMK")]
	Mmk,
	/// Lari
	#[serde(rename = "GEL")]
	Gel,
	/// Latvian Lats
	#[serde(rename = "LVL")]
	Lvl,
	/// Lebanese Pound
	#[serde(rename = "LBP")]
	Lbp,
	/// Lek
	#[serde(rename = "ALL")]
	All,
	/// Lempira
	#[serde(rename = "HNL")]
	Hnl,
	/// Leone
	#[serde(rename = "SLL")]
	Sll,
	/// Leu
	#[serde(rename = "ROL")]
	Rol,
	/// Lev
	#[serde(rename = "BGL")]
	Bgl,
	/// Liberian Dollar
	#[serde(rename = "LRD")]
	Lrd,
	/// Libyan Dinar
	#[serde(rename = "LYD")]
	Lyd,
	/// Lilangeni
	#[serde(rename = "SZL")]
	Szl,
	/// Lithuanian Litas
	#[serde(rename = "LTL")]
	Ltl,
	/// Loti
	#[serde(rename = "LSL")]
	Lsl,
	/// Luxembourg Franc
	#[serde(rename = "LUF")]
	Luf,
	/// Malagasy Franc
	#[serde(rename = "MGF")]
	Mgf,
	/// Malaysian Ringgit
	#[serde(rename = "MYR")]
	Myr,
	/// Maltese Lira
	#[serde(rename = "MTL")]
	Mtl,
	/// Manat
	#[serde(rename = "TMM")]
	Tmm,
	/// Markka
	#[serde(rename = "FIM")]
	Fim,
	/// Mauritius Rupee
	#[serde(rename = "MUR")]
	Mur,
	/// Metical
	#[serde(rename = "MZM")]
	Mzm,
	/// Mexican Peso
	#[serde(rename = "MXN")]
	Mxn,
	/// Mexican Unidad de Inversion (UDI)
	#[serde(rename = "MXV")]
	Mxv,
	/// Moldovan Leu
	#[serde(rename = "MDL")]
	Mdl,
	/// Moroccan Dirham
	#[serde(rename = "MAD")]
	Mad,
	/// Mvdol
	#[serde(rename = "BOV")]
	Bov,
	/// Naira
	#[serde(rename = "NGN")]
	Ngn,
	/// Nakfa
	#[serde(rename = "ERN")]
	Ern,
	/// Namibia Dollar
	#[serde(rename = "NAD")]
	Nad,
	/// Nepalese Rupee
	#[serde(rename = "NPR")]
	Npr,
	/// Netherlands Antillian Guilder
	#[serde(rename = "ANG")]
	Ang,
	/// Netherlands Guilder
	#[serde(rename = "NLG")]
	Nlg,
	/// New Dinar
	#[serde(rename = "YUM")]
	Yum,
	/// New Israeli Sheqel
	#[serde(rename = "ILS")]
	Ils,
	/// New Kwanza
	#[serde(rename = "AON")]
	Aon,
	/// New Taiwan Dollar
	#[serde(rename = "TWD")]
	Twd,
	/// New Zaire
	#[serde(rename = "ZRN")]
	Zrn,
	/// New Zealand Dollar
	#[serde(rename = "NZD")]
	Nzd,
	/// Next day
	#[serde(rename = "USN")]
	Usn,
	/// Ngultrum
	#[serde(rename = "BTN")]
	Btn,
	/// North Korean Won
	#[serde(rename = "KPW")]
	Kpw,
	/// Norwegian Krone
	#[serde(rename = "NOK")]
	Nok,
	/// Nuevo Sol
	#[serde(rename = "PEN")]
	Pen,
	/// Ouguiya
	#[serde(rename = "MRO")]
	Mro,
	/// Pa'anga
	#[serde(rename = "TOP")]
	Top,
	/// Pakistan Rupee
	#[serde(rename = "PKR")]
	Pkr,
	/// Pataca
	#[serde(rename = "MOP")]
	Mop,
	/// Peso Uruguayo
	#[serde(rename = "UYU")]
	Uyu,
	/// Philippine Peso
	#[serde(rename = "PHP")]
	Php,
	/// Portuguese Escudo
	#[serde(rename = "PTE")]
	Pte,
	/// Pound Sterling
	#[serde(rename = "GBP")]
	Gbp,
	/// Pula
	#[serde(rename = "BWP")]
	Bwp,
	/// Qatari Rial
	#[serde(rename = "QAR")]
	Qar,
	/// Quetzal
	#[serde(rename = "GTQ")]
	Gtq,
	/// Rand
	#[serde(rename = "ZAR")]
	Zar,
	/// Rial Omani
	#[serde(rename = "OMR")]
	Omr,
	/// Riel
	#[serde(rename = "KHR")]
	Khr,
	/// Rufiyaa
	#[serde(rename = "MVR")]
	Mvr,
	/// Rupiah
	#[serde(rename = "IDR")]
	Idr,
	/// Russian Ruble
	#[serde(rename = "RUB")]
	Rub,
	/// Russian Ruble
	#[serde(rename = "RUR")]
	Rur,
	/// Rwanda Franc
	#[serde(rename = "RWF")]
	Rwf,
	/// SDR
	#[serde(rename = "XDR")]
	Xdr,
	/// Same day
	#[serde(rename = "USS")]
	Uss,
	/// Saudi Riyal
	#[serde(rename = "SAR")]
	Sar,
	/// Schilling
	#[serde(rename = "ATS")]
	Ats,
	/// Seychelles Rupee
	#[serde(rename = "SCR")]
	Scr,
	/// Singapore Dollar
	#[serde(rename = "SGD")]
	Sgd,
	/// Slovak Koruna
	#[serde(rename = "SKK")]
	Skk,
	/// Solomon Islands Dollar
	#[serde(rename = "SBD")]
	Sbd,
	/// Som
	#[serde(rename = "KGS")]
	Kgs,
	/// Somali Shilling
	#[serde(rename = "SOS")]
	Sos,
	/// Spanish Peseta
	#[serde(rename = "ESP")]
	Esp,
	/// Sri Lanka Rupee
	#[serde(rename = "LKR")]
	Lkr,
	/// St Helena Pound
	#[serde(rename = "SHP")]
	Shp,
	/// Sucre
	#[serde(rename = "ECS")]
	Ecs,
	/// Sudanese Dinar
	#[serde(rename = "SDD")]
	Sdd,
	/// Surinam Guilder
	#[serde(rename = "SRG")]
	Srg,
	/// Swedish Krona
	#[serde(rename = "SEK")]
	Sek,
	/// Swiss Franc
	#[serde(rename = "CHF")]
	Chf,
	/// Syrian Pound
	#[serde(rename = "SYP")]
	Syp,
	/// Tajik Ruble
	#[serde(rename = "TJR")]
	Tjr,
	/// Taka
	#[serde(rename = "BDT")]
	Bdt,
	/// Tala
	#[serde(rename = "WST")]
	Wst,
	/// Tanzanian Shilling
	#[serde(rename = "TZS")]
	Tzs,
	/// Tenge
	#[serde(rename = "KZT")]
	Kzt,
	/// Timor Escudo
	#[serde(rename = "TPE")]
	Tpe,
	/// Tolar
	#[serde(rename = "SIT")]
	Sit,
	/// Trinidad and Tobago Dollar
	#[serde(rename = "TTD")]
	Ttd,
	/// Tugrik
	#[serde(rename = "MNT")]
	Mnt,
	/// Tunisian Dinar
	#[serde(rename = "TND")]
	Tnd,
	/// Turkish Lira
	#[serde(rename = "TRL")]
	Trl,
	/// UAE Dirham
	#[serde(rename = "AED")]
	Aed,
	/// US Dollar
	#[serde(rename = "USD")]
	Usd,
	/// Uganda Shilling
	#[serde(rename = "UGX")]
	Ugx,
	/// Unidad de Valor Constante (UVC)
	#[serde(rename = "ECV")]
	Ecv,
	/// Unidades de fomento
	#[serde(rename = "CLF")]
	Clf,
	/// Uzbekistan Sum
	#[serde(rename = "UZS")]
	Uzs,
	/// Vatu
	#[serde(rename = "VUV")]
	Vuv,
	/// Won
	#[serde(rename = "KRW")]
	Krw,
	/// Yemeni Rial
	#[serde(rename = "YER")]
	Yer,
	/// Yen
	#[serde(rename = "JPY")]
	Jpy,
	/// Yuan Renminbi
	#[serde(rename = "CNY")]
	Cny,
	/// Zimbabwe Dollar
	#[serde(rename = "ZWD")]
	Zwd,
	/// Zloty
	#[serde(rename = "PLN")]
	Pln,
	/// financial Rand
	#[serde(rename = "ZAL")]
	Zal,
	/// Afghani
	#[serde(rename = "004")]
	N004,
	/// Algerian Dinar
	#[serde(rename = "01")]
	N01,
	/// Andorran Peseta
	#[serde(rename = "020")]
	N020,
	/// Argentine Peso
	#[serde(rename = "032")]
	N032,
	/// Armenian Dram
	#[serde(rename = "051")]
	N051,
	/// Aruban Guilder
	#[serde(rename = "533")]
	N533,
	/// Australian Dollar
	#[serde(rename = "036")]
	N036,
	/// Azerbaijanian Manat
	#[serde(rename = "031")]
	N031,
	/// Bahamian Dollar
	#[serde(rename = "044")]
	N044,
	/// Bahraini Dinar
	#[serde(rename = "048")]
	N048,
	/// Baht
	#[serde(rename = "764")]
	N764,
	/// Balboa
	#[serde(rename = "590")]
	N590,
	/// Barbados Dollar
	#[serde(rename = "052")]
	N052,
	/// Belarussian Ruble
	#[serde(rename = "112")]
	N112,
	/// Belgian Franc
	#[serde(rename = "056")]
	N056,
	/// Belize Dollar
	#[serde(rename = "084")]
	N084,
	/// Bermudian Dollar
	#[serde(rename = "060")]
	N060,
	/// Bolivar
	#[serde(rename = "862")]
	N862,
	/// Boliviano
	#[serde(rename = "068")]
	N068,
	/// Brazilian Real
	#[serde(rename = "986")]
	N986,
	/// Brunei Dollar
	#[serde(rename = "096")]
	N096,
	/// Burundi Franc
	#[serde(rename = "108")]
	N108,
	/// CFA Franc BCEAO+
	#[serde(rename = "952")]
	N952,
	/// CFA Franc BEAC#
	#[serde(rename = "950")]
	N950,
	/// CFP Franc
	#[serde(rename = "953")]
	N953,
	/// Canadian Dollar
	#[serde(rename = "124")]
	N124,
	/// Cape Verde Escudo
	#[serde(rename = "132")]
	N132,
	/// Cayman Islands Dollar
	#[serde(rename = "136")]
	N136,
	/// Cedi
	#[serde(rename = "288")]
	N288,
	/// Chilean Peso
	#[serde(rename = "152")]
	N152,
	/// Colombian Peso
	#[serde(rename = "170")]
	N170,
	/// Comoro Franc
	#[serde(rename = "174")]
	N174,
	/// Convertible Marks
	#[serde(rename = "977")]
	N977,
	/// Cordoba Oro
	#[serde(rename = "558")]
	N558,
	/// Costa Rican Colon
	#[serde(rename = "188")]
	N188,
	/// Cuban Peso
	#[serde(rename = "192")]
	N192,
	/// Cyprus Pound
	#[serde(rename = "196")]
	N196,
	/// Czech Koruna
	#[serde(rename = "203")]
	N203,
	/// Dalasi
	#[serde(rename = "270")]
	N270,
	/// Danish Krone
	#[serde(rename = "208")]
	N208,
	/// Denar
	#[serde(rename = "807")]
	N807,
	/// Deutsche Mark
	#[serde(rename = "280")]
	N280,
	/// Djibouti Franc
	#[serde(rename = "262")]
	N262,
	/// Dobra
	#[serde(rename = "678")]
	N678,
	/// Dominican Peso
	#[serde(rename = "214")]
	N214,
	/// Dong
	#[serde(rename = "704")]
	N704,
	/// Drachma
	#[serde(rename = "300")]
	N300,
	/// East Caribbean Dollar
	#[serde(rename = "951")]
	N951,
	/// Egyptian Pound
	#[serde(rename = "818")]
	N818,
	/// El Salvador Colon
	#[serde(rename = "222")]
	N222,
	/// Ethiopian Birr
	#[serde(rename = "230")]
	N230,
	/// Euro
	#[serde(rename = "978")]
	N978,
	/// Falkland Islands Pound
	#[serde(rename = "238")]
	N238,
	/// Fiji Dollar
	#[serde(rename = "242")]
	N242,
	/// Forint
	#[serde(rename = "348")]
	N348,
	/// Franc Congolais
	#[serde(rename = "976")]
	N976,
	/// French Franc
	#[serde(rename = "250")]
	N250,
	/// Gibraltar Pound
	#[serde(rename = "292")]
	N292,
	/// Gourde
	#[serde(rename = "332")]
	N332,
	/// Guarani
	#[serde(rename = "600")]
	N600,
	/// Guinea Franc
	#[serde(rename = "324")]
	N324,
	/// Guinea-Bissau Peso
	#[serde(rename = "624")]
	N624,
	/// Guyana Dollar
	#[serde(rename = "328")]
	N328,
	/// Hong Kong Dollar
	#[serde(rename = "344")]
	N344,
	/// Hryvnia
	#[serde(rename = "980")]
	N980,
	/// Iceland Krona
	#[serde(rename = "352")]
	N352,
	/// Indian Rupee
	#[serde(rename = "356")]
	N356,
	/// Iranian Rial
	#[serde(rename = "364")]
	N364,
	/// Iraqi Dinar
	#[serde(rename = "368")]
	N368,
	/// Irish Pound
	#[serde(rename = "372")]
	N372,
	/// Italian Lira
	#[serde(rename = "380")]
	N380,
	/// Jamaican Dollar
	#[serde(rename = "388")]
	N388,
	/// Jordanian Dinar
	#[serde(rename = "400")]
	N400,
	/// Kenyan Shilling
	#[serde(rename = "404")]
	N404,
	/// Kina
	#[serde(rename = "598")]
	N598,
	/// Kip
	#[serde(rename = "418")]
	N418,
	/// Kroon
	#[serde(rename = "233")]
	N233,
	/// Kuna
	#[serde(rename = "191")]
	N191,
	/// Kuwaiti Dinar
	#[serde(rename = "414")]
	N414,
	/// Kwacha
	#[serde(rename = "454")]
	N454,
	/// Kwacha
	#[serde(rename = "894")]
	N894,
	/// Kwanza Reajustado
	#[serde(rename = "982")]
	N982,
	/// Kyat
	#[serde(rename = "104")]
	N104,
	/// Lari
	#[serde(rename = "981")]
	N981,
	/// Latvian Lats
	#[serde(rename = "428")]
	N428,
	/// Lebanese Pound
	#[serde(rename = "422")]
	N422,
	/// Lek
	#[serde(rename = "008")]
	N008,
	/// Lempira
	#[serde(rename = "340")]
	N340,
	/// Leone
	#[serde(rename = "694")]
	N694,
	/// Leu
	#[serde(rename = "642")]
	N642,
	/// Lev
	#[serde(rename = "100")]
	N100,
	/// Liberian Dollar
	#[serde(rename = "430")]
	N430,
	/// Libyan Dinar
	#[serde(rename = "434")]
	N434,
	/// Lilangeni
	#[serde(rename = "748")]
	N748,
	/// Lithuanian Litas
	#[serde(rename = "440")]
	N440,
	/// Loti
	#[serde(rename = "426")]
	N426,
	/// Luxembourg Franc
	#[serde(rename = "442")]
	N442,
	/// Malagasy Franc
	#[serde(rename = "450")]
	N450,
	/// Malaysian Ringgit
	#[serde(rename = "458")]
	N458,
	/// Maltese Lira
	#[serde(rename = "470")]
	N470,
	/// Manat
	#[serde(rename = "795")]
	N795,
	/// Markka
	#[serde(rename = "246")]
	N246,
	/// Mauritius Rupee
	#[serde(rename = "480")]
	N480,
	/// Metical
	#[serde(rename = "508")]
	N508,
	/// Mexican Peso
	#[serde(rename = "484")]
	N484,
	/// Mexican Unidad de Inversion (UDI)
	#[serde(rename = "979")]
	N979,
	/// Moldovan Leu
	#[serde(rename = "498")]
	N498,
	/// Moroccan Dirham
	#[serde(rename = "504")]
	N504,
	/// Mvdol
	#[serde(rename = "984")]
	N984,
	/// Naira
	#[serde(rename = "566")]
	N566,
	/// Nakfa
	#[serde(rename = "232")]
	N232,
	/// Namibia Dollar
	#[serde(rename = "516")]
	N516,
	/// Nepalese Rupee
	#[serde(rename = "524")]
	N524,
	/// Netherlands Antillian Guilder
	#[serde(rename = "532")]
	N532,
	/// Netherlands Guilder
	#[serde(rename = "528")]
	N528,
	/// New Dinar
	#[serde(rename = "891")]
	N891,
	/// New Israeli Sheqel
	#[serde(rename = "376")]
	N376,
	/// New Kwanza
	#[serde(rename = "02")]
	N02,
	/// New Taiwan Dollar
	#[serde(rename = "901")]
	N901,
	/// New Zaire
	#[serde(rename = "180")]
	N180,
	/// New Zealand Dollar
	#[serde(rename = "554")]
	N554,
	/// Next day
	#[serde(rename = "997")]
	N997,
	/// Ngultrum
	#[serde(rename = "064")]
	N064,
	/// North Korean Won
	#[serde(rename = "408")]
	N408,
	/// Norwegian Krone
	#[serde(rename = "578")]
	N578,
	/// Nuevo Sol
	#[serde(rename = "604")]
	N604,
	/// Ouguiya
	#[serde(rename = "478")]
	N478,
	/// Pa'anga
	#[serde(rename = "776")]
	N776,
	/// Pakistan Rupee
	#[serde(rename = "586")]
	N586,
	/// Pataca
	#[serde(rename = "446")]
	N446,
	/// Peso Uruguayo
	#[serde(rename = "858")]
	N858,
	/// Philippine Peso
	#[serde(rename = "608")]
	N608,
	/// Portuguese Escudo
	#[serde(rename = "620")]
	N620,
	/// Pound Sterling
	#[serde(rename = "826")]
	N826,
	/// Pula
	#[serde(rename = "072")]
	N072,
	/// Qatari Rial
	#[serde(rename = "634")]
	N634,
	/// Quetzal
	#[serde(rename = "320")]
	N320,
	/// Rand
	#[serde(rename = "710")]
	N710,
	/// Rial Omani
	#[serde(rename = "512")]
	N512,
	/// Riel
	#[serde(rename = "116")]
	N116,
	/// Rufiyaa
	#[serde(rename = "462")]
	N462,
	/// Rupiah
	#[serde(rename = "360")]
	N360,
	/// Russian Ruble
	#[serde(rename = "643")]
	N643,
	/// Russian Ruble
	#[serde(rename = "810")]
	N810,
	/// Rwanda Franc
	#[serde(rename = "646")]
	N646,
	/// SDR
	#[serde(rename = "960")]
	N960,
	/// Same day
	#[serde(rename = "998")]
	N998,
	/// Saudi Riyal
	#[serde(rename = "682")]
	N682,
	/// Schilling
	#[serde(rename = "040")]
	N040,
	/// Seychelles Rupee
	#[serde(rename = "690")]
	N690,
	/// Singapore Dollar
	#[serde(rename = "702")]
	N702,
	/// Slovak Koruna
	#[serde(rename = "703")]
	N703,
	/// Solomon Islands Dollar
	#[serde(rename = "090")]
	N090,
	/// Som
	#[serde(rename = "417")]
	N417,
	/// Somali Shilling
	#[serde(rename = "706")]
	N706,
	/// Spanish Peseta
	#[serde(rename = "724")]
	N724,
	/// Sri Lanka Rupee
	#[serde(rename = "144")]
	N144,
	/// St Helena Pound
	#[serde(rename = "654")]
	N654,
	/// Sucre
	#[serde(rename = "218")]
	N218,
	/// Sudanese Dinar
	#[serde(rename = "736")]
	N736,
	/// Surinam Guilder
	#[serde(rename = "740")]
	N740,
	/// Swedish Krona
	#[serde(rename = "752")]
	N752,
	/// Swiss Franc
	#[serde(rename = "756")]
	N756,
	/// Syrian Pound
	#[serde(rename = "760")]
	N760,
	/// Tajik Ruble
	#[serde(rename = "762")]
	N762,
	/// Taka
	#[serde(rename = "050")]
	N050,
	/// Tala
	#[serde(rename = "882")]
	N882,
	/// Tanzanian Shilling
	#[serde(rename = "834")]
	N834,
	/// Tenge
	#[serde(rename = "398")]
	N398,
	/// Timor Escudo
	#[serde(rename = "626")]
	N626,
	/// Tolar
	#[serde(rename = "705")]
	N705,
	/// Trinidad and Tobago Dollar
	#[serde(rename = "780")]
	N780,
	/// Tugrik
	#[serde(rename = "496")]
	N496,
	/// Tunisian Dinar
	#[serde(rename = "788")]
	N788,
	/// Turkish Lira
	#[serde(rename = "792")]
	N792,
	/// UAE Dirham
	#[serde(rename = "784")]
	N784,
	/// US Dollar
	#[serde(rename = "840")]
	N840,
	/// Uganda Shilling
	#[serde(rename = "800")]
	N800,
	/// Unidad de Valor Constante (UVC)
	#[serde(rename = "983")]
	N983,
	/// Unidades de fomento
	#[serde(rename = "990")]
	N990,
	/// Uzbekistan Sum
	#[serde(rename = "860")]
	N860,
	/// Vatu
	#[serde(rename = "548")]
	N548,
	/// Won
	#[serde(rename = "410")]
	N410,
	/// Yemeni Rial
	#[serde(rename = "886")]
	N886,
	/// Yen
	#[serde(rename = "392")]
	N392,
	/// Yuan Renminbi
	#[serde(rename = "156")]
	N156,
	/// Zimbabwe Dollar
	#[serde(rename = "716")]
	N716,
	/// Zloty
	#[serde(rename = "985")]
	N985,
	/// financial Rand
	#[serde(rename = "991")]
	N991,
	/// Gold
	#[serde(rename = "XAU")]
	Xau,
	/// European Composite Unit (EURCO)
	#[serde(rename = "XBA")]
	Xba,
	/// European Monetary Unit (E.M.U.-6)
	#[serde(rename = "XBB")]
	Xbb,
	/// European Unit of Account 9 (E.U.A.- 9)
	#[serde(rename = "XBC")]
	Xbc,
	/// European Unit of Account 17 (E.U.A.- 17)
	#[serde(rename = "XBD")]
	Xbd,
	/// Palladium
	#[serde(rename = "XPD")]
	Xpd,
	/// Platinum
	#[serde(rename = "XPT")]
	Xpt,
	/// Silver
	#[serde(rename = "XAG")]
	Xag,
	/// UIC-Franc
	#[serde(rename = "XFU")]
	Xfu,
	/// Gold-Franc
	#[serde(rename = "XFO")]
	Xfo,
	/// Codes specifically reserved for testing purposes
	#[serde(rename = "XTS")]
	Xts,
	/// Codes assigned for transactions where no currency is involved
	#[serde(rename = "XXX")]
	Xxx,
	/// Gold
	#[serde(rename = "959")]
	N959,
	/// European Composite Unit (EURCO)
	#[serde(rename = "955")]
	N955,
	/// European Monetary Unit (E.M.U.-6)
	#[serde(rename = "956")]
	N956,
	/// European Unit of Account 9 (E.U.A.- 9)
	#[serde(rename = "957")]
	N957,
	/// European Unit of Account 17 (E.U.A.- 17)
	#[serde(rename = "958")]
	N958,
	/// Palladium
	#[serde(rename = "964")]
	N964,
	/// Platinum
	#[serde(rename = "962")]
	N962,
	/// Silver
	#[serde(rename = "961")]
	N961,
	/// Codes specifically reserved for testing purposes
	#[serde(rename = "963")]
	N963,
	/// Codes assigned for transactions where no currency is involved
	#[serde(rename = "999")]
	N999,
}

impl Default for Currency {
	fn default() -> Self {
		Currency::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlCurrency {
	/// Afghani
	#[serde(rename = "AFA")]
	Afa,
	/// Algerian Dinar
	#[serde(rename = "DZD")]
	Dzd,
	/// Andorran Peseta
	#[serde(rename = "ADP")]
	Adp,
	/// Argentine Peso
	#[serde(rename = "ARS")]
	Ars,
	/// Armenian Dram
	#[serde(rename = "AMD")]
	Amd,
	/// Aruban Guilder
	#[serde(rename = "AWG")]
	Awg,
	/// Australian Dollar
	#[serde(rename = "AUD")]
	Aud,
	/// Azerbaijanian Manat
	#[serde(rename = "AZM")]
	Azm,
	/// Bahamian Dollar
	#[serde(rename = "BSD")]
	Bsd,
	/// Bahraini Dinar
	#[serde(rename = "BHD")]
	Bhd,
	/// Baht
	#[serde(rename = "THB")]
	Thb,
	/// Balboa
	#[serde(rename = "PAB")]
	Pab,
	/// Barbados Dollar
	#[serde(rename = "BBD")]
	Bbd,
	/// Belarussian Ruble
	#[serde(rename = "BYB")]
	Byb,
	/// Belgian Franc
	#[serde(rename = "BEF")]
	Bef,
	/// Belize Dollar
	#[serde(rename = "BZD")]
	Bzd,
	/// Bermudian Dollar
	#[serde(rename = "BMD")]
	Bmd,
	/// Bolivar
	#[serde(rename = "VEB")]
	Veb,
	/// Boliviano
	#[serde(rename = "BOB")]
	Bob,
	/// Brazilian Real
	#[serde(rename = "BRL")]
	Brl,
	/// Brunei Dollar
	#[serde(rename = "BND")]
	Bnd,
	/// Burundi Franc
	#[serde(rename = "BIF")]
	Bif,
	/// CFA Franc BCEAO+
	#[serde(rename = "XOF")]
	Xof,
	/// CFA Franc BEAC#
	#[serde(rename = "XAF")]
	Xaf,
	/// CFP Franc
	#[serde(rename = "XPF")]
	Xpf,
	/// Canadian Dollar
	#[serde(rename = "CAD")]
	Cad,
	/// Cape Verde Escudo
	#[serde(rename = "CVE")]
	Cve,
	/// Cayman Islands Dollar
	#[serde(rename = "KYD")]
	Kyd,
	/// Cedi
	#[serde(rename = "GHC")]
	Ghc,
	/// Chilean Peso
	#[serde(rename = "CLP")]
	Clp,
	/// Colombian Peso
	#[serde(rename = "COP")]
	Cop,
	/// Comoro Franc
	#[serde(rename = "KMF")]
	Kmf,
	/// Convertible Marks
	#[serde(rename = "BAM")]
	Bam,
	/// Cordoba Oro
	#[serde(rename = "NIO")]
	Nio,
	/// Costa Rican Colon
	#[serde(rename = "CRC")]
	Crc,
	/// Cuban Peso
	#[serde(rename = "CUP")]
	Cup,
	/// Cyprus Pound
	#[serde(rename = "CYP")]
	Cyp,
	/// Czech Koruna
	#[serde(rename = "CZK")]
	Czk,
	/// Dalasi
	#[serde(rename = "GMD")]
	Gmd,
	/// Danish Krone
	#[serde(rename = "DKK")]
	Dkk,
	/// Denar
	#[serde(rename = "MKD")]
	Mkd,
	/// Deutsche Mark
	#[serde(rename = "DEM")]
	Dem,
	/// Djibouti Franc
	#[serde(rename = "DJF")]
	Djf,
	/// Dobra
	#[serde(rename = "STD")]
	Std,
	/// Dominican Peso
	#[serde(rename = "DOP")]
	Dop,
	/// Dong
	#[serde(rename = "VND")]
	Vnd,
	/// Drachma
	#[serde(rename = "GRD")]
	Grd,
	/// East Caribbean Dollar
	#[serde(rename = "XCD")]
	Xcd,
	/// Egyptian Pound
	#[serde(rename = "EGP")]
	Egp,
	/// El Salvador Colon
	#[serde(rename = "SVC")]
	Svc,
	/// Ethiopian Birr
	#[serde(rename = "ETB")]
	Etb,
	/// Euro
	#[serde(rename = "EUR")]
	Eur,
	/// Falkland Islands Pound
	#[serde(rename = "FKP")]
	Fkp,
	/// Fiji Dollar
	#[serde(rename = "FJD")]
	Fjd,
	/// Forint
	#[serde(rename = "HUF")]
	Huf,
	/// Franc Congolais
	#[serde(rename = "CDF")]
	Cdf,
	/// French Franc
	#[serde(rename = "FRF")]
	Frf,
	/// Gibraltar Pound
	#[serde(rename = "GIP")]
	Gip,
	/// Gourde
	#[serde(rename = "HTG")]
	Htg,
	/// Guarani
	#[serde(rename = "PYG")]
	Pyg,
	/// Guinea Franc
	#[serde(rename = "GNF")]
	Gnf,
	/// Guinea-Bissau Peso
	#[serde(rename = "GWP")]
	Gwp,
	/// Guyana Dollar
	#[serde(rename = "GYD")]
	Gyd,
	/// Hong Kong Dollar
	#[serde(rename = "HKD")]
	Hkd,
	/// Hryvnia
	#[serde(rename = "UAH")]
	Uah,
	/// Iceland Krona
	#[serde(rename = "ISK")]
	Isk,
	/// Indian Rupee
	#[serde(rename = "INR")]
	Inr,
	/// Iranian Rial
	#[serde(rename = "IRR")]
	Irr,
	/// Iraqi Dinar
	#[serde(rename = "IQD")]
	Iqd,
	/// Irish Pound
	#[serde(rename = "IEP")]
	Iep,
	/// Italian Lira
	#[serde(rename = "ITL")]
	Itl,
	/// Jamaican Dollar
	#[serde(rename = "JMD")]
	Jmd,
	/// Jordanian Dinar
	#[serde(rename = "JOD")]
	Jod,
	/// Kenyan Shilling
	#[serde(rename = "KES")]
	Kes,
	/// Kina
	#[serde(rename = "PGK")]
	Pgk,
	/// Kip
	#[serde(rename = "LAK")]
	Lak,
	/// Kroon
	#[serde(rename = "EEK")]
	Eek,
	/// Kuna
	#[serde(rename = "HRK")]
	Hrk,
	/// Kuwaiti Dinar
	#[serde(rename = "KWD")]
	Kwd,
	/// Kwacha
	#[serde(rename = "MWK")]
	Mwk,
	/// Kwacha
	#[serde(rename = "ZMK")]
	Zmk,
	/// Kwanza Reajustado
	#[serde(rename = "AOR")]
	Aor,
	/// Kyat
	#[serde(rename = "MMK")]
	Mmk,
	/// Lari
	#[serde(rename = "GEL")]
	Gel,
	/// Latvian Lats
	#[serde(rename = "LVL")]
	Lvl,
	/// Lebanese Pound
	#[serde(rename = "LBP")]
	Lbp,
	/// Lek
	#[serde(rename = "ALL")]
	All,
	/// Lempira
	#[serde(rename = "HNL")]
	Hnl,
	/// Leone
	#[serde(rename = "SLL")]
	Sll,
	/// Leu
	#[serde(rename = "ROL")]
	Rol,
	/// Lev
	#[serde(rename = "BGL")]
	Bgl,
	/// Liberian Dollar
	#[serde(rename = "LRD")]
	Lrd,
	/// Libyan Dinar
	#[serde(rename = "LYD")]
	Lyd,
	/// Lilangeni
	#[serde(rename = "SZL")]
	Szl,
	/// Lithuanian Litas
	#[serde(rename = "LTL")]
	Ltl,
	/// Loti
	#[serde(rename = "LSL")]
	Lsl,
	/// Luxembourg Franc
	#[serde(rename = "LUF")]
	Luf,
	/// Malagasy Franc
	#[serde(rename = "MGF")]
	Mgf,
	/// Malaysian Ringgit
	#[serde(rename = "MYR")]
	Myr,
	/// Maltese Lira
	#[serde(rename = "MTL")]
	Mtl,
	/// Manat
	#[serde(rename = "TMM")]
	Tmm,
	/// Markka
	#[serde(rename = "FIM")]
	Fim,
	/// Mauritius Rupee
	#[serde(rename = "MUR")]
	Mur,
	/// Metical
	#[serde(rename = "MZM")]
	Mzm,
	/// Mexican Peso
	#[serde(rename = "MXN")]
	Mxn,
	/// Mexican Unidad de Inversion (UDI)
	#[serde(rename = "MXV")]
	Mxv,
	/// Moldovan Leu
	#[serde(rename = "MDL")]
	Mdl,
	/// Moroccan Dirham
	#[serde(rename = "MAD")]
	Mad,
	/// Mvdol
	#[serde(rename = "BOV")]
	Bov,
	/// Naira
	#[serde(rename = "NGN")]
	Ngn,
	/// Nakfa
	#[serde(rename = "ERN")]
	Ern,
	/// Namibia Dollar
	#[serde(rename = "NAD")]
	Nad,
	/// Nepalese Rupee
	#[serde(rename = "NPR")]
	Npr,
	/// Netherlands Antillian Guilder
	#[serde(rename = "ANG")]
	Ang,
	/// Netherlands Guilder
	#[serde(rename = "NLG")]
	Nlg,
	/// New Dinar
	#[serde(rename = "YUM")]
	Yum,
	/// New Israeli Sheqel
	#[serde(rename = "ILS")]
	Ils,
	/// New Kwanza
	#[serde(rename = "AON")]
	Aon,
	/// New Taiwan Dollar
	#[serde(rename = "TWD")]
	Twd,
	/// New Zaire
	#[serde(rename = "ZRN")]
	Zrn,
	/// New Zealand Dollar
	#[serde(rename = "NZD")]
	Nzd,
	/// Next day
	#[serde(rename = "USN")]
	Usn,
	/// Ngultrum
	#[serde(rename = "BTN")]
	Btn,
	/// North Korean Won
	#[serde(rename = "KPW")]
	Kpw,
	/// Norwegian Krone
	#[serde(rename = "NOK")]
	Nok,
	/// Nuevo Sol
	#[serde(rename = "PEN")]
	Pen,
	/// Ouguiya
	#[serde(rename = "MRO")]
	Mro,
	/// Pa'anga
	#[serde(rename = "TOP")]
	Top,
	/// Pakistan Rupee
	#[serde(rename = "PKR")]
	Pkr,
	/// Pataca
	#[serde(rename = "MOP")]
	Mop,
	/// Peso Uruguayo
	#[serde(rename = "UYU")]
	Uyu,
	/// Philippine Peso
	#[serde(rename = "PHP")]
	Php,
	/// Portuguese Escudo
	#[serde(rename = "PTE")]
	Pte,
	/// Pound Sterling
	#[serde(rename = "GBP")]
	Gbp,
	/// Pula
	#[serde(rename = "BWP")]
	Bwp,
	/// Qatari Rial
	#[serde(rename = "QAR")]
	Qar,
	/// Quetzal
	#[serde(rename = "GTQ")]
	Gtq,
	/// Rand
	#[serde(rename = "ZAR")]
	Zar,
	/// Rial Omani
	#[serde(rename = "OMR")]
	Omr,
	/// Riel
	#[serde(rename = "KHR")]
	Khr,
	/// Rufiyaa
	#[serde(rename = "MVR")]
	Mvr,
	/// Rupiah
	#[serde(rename = "IDR")]
	Idr,
	/// Russian Ruble
	#[serde(rename = "RUB")]
	Rub,
	/// Russian Ruble
	#[serde(rename = "RUR")]
	Rur,
	/// Rwanda Franc
	#[serde(rename = "RWF")]
	Rwf,
	/// SDR
	#[serde(rename = "XDR")]
	Xdr,
	/// Same day
	#[serde(rename = "USS")]
	Uss,
	/// Saudi Riyal
	#[serde(rename = "SAR")]
	Sar,
	/// Schilling
	#[serde(rename = "ATS")]
	Ats,
	/// Seychelles Rupee
	#[serde(rename = "SCR")]
	Scr,
	/// Singapore Dollar
	#[serde(rename = "SGD")]
	Sgd,
	/// Slovak Koruna
	#[serde(rename = "SKK")]
	Skk,
	/// Solomon Islands Dollar
	#[serde(rename = "SBD")]
	Sbd,
	/// Som
	#[serde(rename = "KGS")]
	Kgs,
	/// Somali Shilling
	#[serde(rename = "SOS")]
	Sos,
	/// Spanish Peseta
	#[serde(rename = "ESP")]
	Esp,
	/// Sri Lanka Rupee
	#[serde(rename = "LKR")]
	Lkr,
	/// St Helena Pound
	#[serde(rename = "SHP")]
	Shp,
	/// Sucre
	#[serde(rename = "ECS")]
	Ecs,
	/// Sudanese Dinar
	#[serde(rename = "SDD")]
	Sdd,
	/// Surinam Guilder
	#[serde(rename = "SRG")]
	Srg,
	/// Swedish Krona
	#[serde(rename = "SEK")]
	Sek,
	/// Swiss Franc
	#[serde(rename = "CHF")]
	Chf,
	/// Syrian Pound
	#[serde(rename = "SYP")]
	Syp,
	/// Tajik Ruble
	#[serde(rename = "TJR")]
	Tjr,
	/// Taka
	#[serde(rename = "BDT")]
	Bdt,
	/// Tala
	#[serde(rename = "WST")]
	Wst,
	/// Tanzanian Shilling
	#[serde(rename = "TZS")]
	Tzs,
	/// Tenge
	#[serde(rename = "KZT")]
	Kzt,
	/// Timor Escudo
	#[serde(rename = "TPE")]
	Tpe,
	/// Tolar
	#[serde(rename = "SIT")]
	Sit,
	/// Trinidad and Tobago Dollar
	#[serde(rename = "TTD")]
	Ttd,
	/// Tugrik
	#[serde(rename = "MNT")]
	Mnt,
	/// Tunisian Dinar
	#[serde(rename = "TND")]
	Tnd,
	/// Turkish Lira
	#[serde(rename = "TRL")]
	Trl,
	/// UAE Dirham
	#[serde(rename = "AED")]
	Aed,
	/// US Dollar
	#[serde(rename = "USD")]
	Usd,
	/// Uganda Shilling
	#[serde(rename = "UGX")]
	Ugx,
	/// Unidad de Valor Constante (UVC)
	#[serde(rename = "ECV")]
	Ecv,
	/// Unidades de fomento
	#[serde(rename = "CLF")]
	Clf,
	/// Uzbekistan Sum
	#[serde(rename = "UZS")]
	Uzs,
	/// Vatu
	#[serde(rename = "VUV")]
	Vuv,
	/// Won
	#[serde(rename = "KRW")]
	Krw,
	/// Yemeni Rial
	#[serde(rename = "YER")]
	Yer,
	/// Yen
	#[serde(rename = "JPY")]
	Jpy,
	/// Yuan Renminbi
	#[serde(rename = "CNY")]
	Cny,
	/// Zimbabwe Dollar
	#[serde(rename = "ZWD")]
	Zwd,
	/// Zloty
	#[serde(rename = "PLN")]
	Pln,
	/// financial Rand
	#[serde(rename = "ZAL")]
	Zal,
	/// Afghani
	#[serde(rename = "004")]
	N004,
	/// Algerian Dinar
	#[serde(rename = "01")]
	N01,
	/// Andorran Peseta
	#[serde(rename = "020")]
	N020,
	/// Argentine Peso
	#[serde(rename = "032")]
	N032,
	/// Armenian Dram
	#[serde(rename = "051")]
	N051,
	/// Aruban Guilder
	#[serde(rename = "533")]
	N533,
	/// Australian Dollar
	#[serde(rename = "036")]
	N036,
	/// Azerbaijanian Manat
	#[serde(rename = "031")]
	N031,
	/// Bahamian Dollar
	#[serde(rename = "044")]
	N044,
	/// Bahraini Dinar
	#[serde(rename = "048")]
	N048,
	/// Baht
	#[serde(rename = "764")]
	N764,
	/// Balboa
	#[serde(rename = "590")]
	N590,
	/// Barbados Dollar
	#[serde(rename = "052")]
	N052,
	/// Belarussian Ruble
	#[serde(rename = "112")]
	N112,
	/// Belgian Franc
	#[serde(rename = "056")]
	N056,
	/// Belize Dollar
	#[serde(rename = "084")]
	N084,
	/// Bermudian Dollar
	#[serde(rename = "060")]
	N060,
	/// Bolivar
	#[serde(rename = "862")]
	N862,
	/// Boliviano
	#[serde(rename = "068")]
	N068,
	/// Brazilian Real
	#[serde(rename = "986")]
	N986,
	/// Brunei Dollar
	#[serde(rename = "096")]
	N096,
	/// Burundi Franc
	#[serde(rename = "108")]
	N108,
	/// CFA Franc BCEAO+
	#[serde(rename = "952")]
	N952,
	/// CFA Franc BEAC#
	#[serde(rename = "950")]
	N950,
	/// CFP Franc
	#[serde(rename = "953")]
	N953,
	/// Canadian Dollar
	#[serde(rename = "124")]
	N124,
	/// Cape Verde Escudo
	#[serde(rename = "132")]
	N132,
	/// Cayman Islands Dollar
	#[serde(rename = "136")]
	N136,
	/// Cedi
	#[serde(rename = "288")]
	N288,
	/// Chilean Peso
	#[serde(rename = "152")]
	N152,
	/// Colombian Peso
	#[serde(rename = "170")]
	N170,
	/// Comoro Franc
	#[serde(rename = "174")]
	N174,
	/// Convertible Marks
	#[serde(rename = "977")]
	N977,
	/// Cordoba Oro
	#[serde(rename = "558")]
	N558,
	/// Costa Rican Colon
	#[serde(rename = "188")]
	N188,
	/// Cuban Peso
	#[serde(rename = "192")]
	N192,
	/// Cyprus Pound
	#[serde(rename = "196")]
	N196,
	/// Czech Koruna
	#[serde(rename = "203")]
	N203,
	/// Dalasi
	#[serde(rename = "270")]
	N270,
	/// Danish Krone
	#[serde(rename = "208")]
	N208,
	/// Denar
	#[serde(rename = "807")]
	N807,
	/// Deutsche Mark
	#[serde(rename = "280")]
	N280,
	/// Djibouti Franc
	#[serde(rename = "262")]
	N262,
	/// Dobra
	#[serde(rename = "678")]
	N678,
	/// Dominican Peso
	#[serde(rename = "214")]
	N214,
	/// Dong
	#[serde(rename = "704")]
	N704,
	/// Drachma
	#[serde(rename = "300")]
	N300,
	/// East Caribbean Dollar
	#[serde(rename = "951")]
	N951,
	/// Egyptian Pound
	#[serde(rename = "818")]
	N818,
	/// El Salvador Colon
	#[serde(rename = "222")]
	N222,
	/// Ethiopian Birr
	#[serde(rename = "230")]
	N230,
	/// Euro
	#[serde(rename = "978")]
	N978,
	/// Falkland Islands Pound
	#[serde(rename = "238")]
	N238,
	/// Fiji Dollar
	#[serde(rename = "242")]
	N242,
	/// Forint
	#[serde(rename = "348")]
	N348,
	/// Franc Congolais
	#[serde(rename = "976")]
	N976,
	/// French Franc
	#[serde(rename = "250")]
	N250,
	/// Gibraltar Pound
	#[serde(rename = "292")]
	N292,
	/// Gourde
	#[serde(rename = "332")]
	N332,
	/// Guarani
	#[serde(rename = "600")]
	N600,
	/// Guinea Franc
	#[serde(rename = "324")]
	N324,
	/// Guinea-Bissau Peso
	#[serde(rename = "624")]
	N624,
	/// Guyana Dollar
	#[serde(rename = "328")]
	N328,
	/// Hong Kong Dollar
	#[serde(rename = "344")]
	N344,
	/// Hryvnia
	#[serde(rename = "980")]
	N980,
	/// Iceland Krona
	#[serde(rename = "352")]
	N352,
	/// Indian Rupee
	#[serde(rename = "356")]
	N356,
	/// Iranian Rial
	#[serde(rename = "364")]
	N364,
	/// Iraqi Dinar
	#[serde(rename = "368")]
	N368,
	/// Irish Pound
	#[serde(rename = "372")]
	N372,
	/// Italian Lira
	#[serde(rename = "380")]
	N380,
	/// Jamaican Dollar
	#[serde(rename = "388")]
	N388,
	/// Jordanian Dinar
	#[serde(rename = "400")]
	N400,
	/// Kenyan Shilling
	#[serde(rename = "404")]
	N404,
	/// Kina
	#[serde(rename = "598")]
	N598,
	/// Kip
	#[serde(rename = "418")]
	N418,
	/// Kroon
	#[serde(rename = "233")]
	N233,
	/// Kuna
	#[serde(rename = "191")]
	N191,
	/// Kuwaiti Dinar
	#[serde(rename = "414")]
	N414,
	/// Kwacha
	#[serde(rename = "454")]
	N454,
	/// Kwacha
	#[serde(rename = "894")]
	N894,
	/// Kwanza Reajustado
	#[serde(rename = "982")]
	N982,
	/// Kyat
	#[serde(rename = "104")]
	N104,
	/// Lari
	#[serde(rename = "981")]
	N981,
	/// Latvian Lats
	#[serde(rename = "428")]
	N428,
	/// Lebanese Pound
	#[serde(rename = "422")]
	N422,
	/// Lek
	#[serde(rename = "008")]
	N008,
	/// Lempira
	#[serde(rename = "340")]
	N340,
	/// Leone
	#[serde(rename = "694")]
	N694,
	/// Leu
	#[serde(rename = "642")]
	N642,
	/// Lev
	#[serde(rename = "100")]
	N100,
	/// Liberian Dollar
	#[serde(rename = "430")]
	N430,
	/// Libyan Dinar
	#[serde(rename = "434")]
	N434,
	/// Lilangeni
	#[serde(rename = "748")]
	N748,
	/// Lithuanian Litas
	#[serde(rename = "440")]
	N440,
	/// Loti
	#[serde(rename = "426")]
	N426,
	/// Luxembourg Franc
	#[serde(rename = "442")]
	N442,
	/// Malagasy Franc
	#[serde(rename = "450")]
	N450,
	/// Malaysian Ringgit
	#[serde(rename = "458")]
	N458,
	/// Maltese Lira
	#[serde(rename = "470")]
	N470,
	/// Manat
	#[serde(rename = "795")]
	N795,
	/// Markka
	#[serde(rename = "246")]
	N246,
	/// Mauritius Rupee
	#[serde(rename = "480")]
	N480,
	/// Metical
	#[serde(rename = "508")]
	N508,
	/// Mexican Peso
	#[serde(rename = "484")]
	N484,
	/// Mexican Unidad de Inversion (UDI)
	#[serde(rename = "979")]
	N979,
	/// Moldovan Leu
	#[serde(rename = "498")]
	N498,
	/// Moroccan Dirham
	#[serde(rename = "504")]
	N504,
	/// Mvdol
	#[serde(rename = "984")]
	N984,
	/// Naira
	#[serde(rename = "566")]
	N566,
	/// Nakfa
	#[serde(rename = "232")]
	N232,
	/// Namibia Dollar
	#[serde(rename = "516")]
	N516,
	/// Nepalese Rupee
	#[serde(rename = "524")]
	N524,
	/// Netherlands Antillian Guilder
	#[serde(rename = "532")]
	N532,
	/// Netherlands Guilder
	#[serde(rename = "528")]
	N528,
	/// New Dinar
	#[serde(rename = "891")]
	N891,
	/// New Israeli Sheqel
	#[serde(rename = "376")]
	N376,
	/// New Kwanza
	#[serde(rename = "02")]
	N02,
	/// New Taiwan Dollar
	#[serde(rename = "901")]
	N901,
	/// New Zaire
	#[serde(rename = "180")]
	N180,
	/// New Zealand Dollar
	#[serde(rename = "554")]
	N554,
	/// Next day
	#[serde(rename = "997")]
	N997,
	/// Ngultrum
	#[serde(rename = "064")]
	N064,
	/// North Korean Won
	#[serde(rename = "408")]
	N408,
	/// Norwegian Krone
	#[serde(rename = "578")]
	N578,
	/// Nuevo Sol
	#[serde(rename = "604")]
	N604,
	/// Ouguiya
	#[serde(rename = "478")]
	N478,
	/// Pa'anga
	#[serde(rename = "776")]
	N776,
	/// Pakistan Rupee
	#[serde(rename = "586")]
	N586,
	/// Pataca
	#[serde(rename = "446")]
	N446,
	/// Peso Uruguayo
	#[serde(rename = "858")]
	N858,
	/// Philippine Peso
	#[serde(rename = "608")]
	N608,
	/// Portuguese Escudo
	#[serde(rename = "620")]
	N620,
	/// Pound Sterling
	#[serde(rename = "826")]
	N826,
	/// Pula
	#[serde(rename = "072")]
	N072,
	/// Qatari Rial
	#[serde(rename = "634")]
	N634,
	/// Quetzal
	#[serde(rename = "320")]
	N320,
	/// Rand
	#[serde(rename = "710")]
	N710,
	/// Rial Omani
	#[serde(rename = "512")]
	N512,
	/// Riel
	#[serde(rename = "116")]
	N116,
	/// Rufiyaa
	#[serde(rename = "462")]
	N462,
	/// Rupiah
	#[serde(rename = "360")]
	N360,
	/// Russian Ruble
	#[serde(rename = "643")]
	N643,
	/// Russian Ruble
	#[serde(rename = "810")]
	N810,
	/// Rwanda Franc
	#[serde(rename = "646")]
	N646,
	/// SDR
	#[serde(rename = "960")]
	N960,
	/// Same day
	#[serde(rename = "998")]
	N998,
	/// Saudi Riyal
	#[serde(rename = "682")]
	N682,
	/// Schilling
	#[serde(rename = "040")]
	N040,
	/// Seychelles Rupee
	#[serde(rename = "690")]
	N690,
	/// Singapore Dollar
	#[serde(rename = "702")]
	N702,
	/// Slovak Koruna
	#[serde(rename = "703")]
	N703,
	/// Solomon Islands Dollar
	#[serde(rename = "090")]
	N090,
	/// Som
	#[serde(rename = "417")]
	N417,
	/// Somali Shilling
	#[serde(rename = "706")]
	N706,
	/// Spanish Peseta
	#[serde(rename = "724")]
	N724,
	/// Sri Lanka Rupee
	#[serde(rename = "144")]
	N144,
	/// St Helena Pound
	#[serde(rename = "654")]
	N654,
	/// Sucre
	#[serde(rename = "218")]
	N218,
	/// Sudanese Dinar
	#[serde(rename = "736")]
	N736,
	/// Surinam Guilder
	#[serde(rename = "740")]
	N740,
	/// Swedish Krona
	#[serde(rename = "752")]
	N752,
	/// Swiss Franc
	#[serde(rename = "756")]
	N756,
	/// Syrian Pound
	#[serde(rename = "760")]
	N760,
	/// Tajik Ruble
	#[serde(rename = "762")]
	N762,
	/// Taka
	#[serde(rename = "050")]
	N050,
	/// Tala
	#[serde(rename = "882")]
	N882,
	/// Tanzanian Shilling
	#[serde(rename = "834")]
	N834,
	/// Tenge
	#[serde(rename = "398")]
	N398,
	/// Timor Escudo
	#[serde(rename = "626")]
	N626,
	/// Tolar
	#[serde(rename = "705")]
	N705,
	/// Trinidad and Tobago Dollar
	#[serde(rename = "780")]
	N780,
	/// Tugrik
	#[serde(rename = "496")]
	N496,
	/// Tunisian Dinar
	#[serde(rename = "788")]
	N788,
	/// Turkish Lira
	#[serde(rename = "792")]
	N792,
	/// UAE Dirham
	#[serde(rename = "784")]
	N784,
	/// US Dollar
	#[serde(rename = "840")]
	N840,
	/// Uganda Shilling
	#[serde(rename = "800")]
	N800,
	/// Unidad de Valor Constante (UVC)
	#[serde(rename = "983")]
	N983,
	/// Unidades de fomento
	#[serde(rename = "990")]
	N990,
	/// Uzbekistan Sum
	#[serde(rename = "860")]
	N860,
	/// Vatu
	#[serde(rename = "548")]
	N548,
	/// Won
	#[serde(rename = "410")]
	N410,
	/// Yemeni Rial
	#[serde(rename = "886")]
	N886,
	/// Yen
	#[serde(rename = "392")]
	N392,
	/// Yuan Renminbi
	#[serde(rename = "156")]
	N156,
	/// Zimbabwe Dollar
	#[serde(rename = "716")]
	N716,
	/// Zloty
	#[serde(rename = "985")]
	N985,
	/// financial Rand
	#[serde(rename = "991")]
	N991,
	/// Gold
	#[serde(rename = "XAU")]
	Xau,
	/// European Composite Unit (EURCO)
	#[serde(rename = "XBA")]
	Xba,
	/// European Monetary Unit (E.M.U.-6)
	#[serde(rename = "XBB")]
	Xbb,
	/// European Unit of Account 9 (E.U.A.- 9)
	#[serde(rename = "XBC")]
	Xbc,
	/// European Unit of Account 17 (E.U.A.- 17)
	#[serde(rename = "XBD")]
	Xbd,
	/// Palladium
	#[serde(rename = "XPD")]
	Xpd,
	/// Platinum
	#[serde(rename = "XPT")]
	Xpt,
	/// Silver
	#[serde(rename = "XAG")]
	Xag,
	/// UIC-Franc
	#[serde(rename = "XFU")]
	Xfu,
	/// Gold-Franc
	#[serde(rename = "XFO")]
	Xfo,
	/// Codes specifically reserved for testing purposes
	#[serde(rename = "XTS")]
	Xts,
	/// Codes assigned for transactions where no currency is involved
	#[serde(rename = "XXX")]
	Xxx,
	/// Gold
	#[serde(rename = "959")]
	N959,
	/// European Composite Unit (EURCO)
	#[serde(rename = "955")]
	N955,
	/// European Monetary Unit (E.M.U.-6)
	#[serde(rename = "956")]
	N956,
	/// European Unit of Account 9 (E.U.A.- 9)
	#[serde(rename = "957")]
	N957,
	/// European Unit of Account 17 (E.U.A.- 17)
	#[serde(rename = "958")]
	N958,
	/// Palladium
	#[serde(rename = "964")]
	N964,
	/// Platinum
	#[serde(rename = "962")]
	N962,
	/// Silver
	#[serde(rename = "961")]
	N961,
	/// Codes specifically reserved for testing purposes
	#[serde(rename = "963")]
	N963,
	/// Codes assigned for transactions where no currency is involved
	#[serde(rename = "999")]
	N999,
}

impl Default for SettlCurrency {
	fn default() -> Self {
		SettlCurrency::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	RoundLotBasedUponAHrefTag996UnitOfMeasureHtmlTargetBottomUnitOfMeasureNbspA,
}

impl Default for LotType {
	fn default() -> Self {
		LotType::OddLot
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TickDirection {
	/// Plus Tick
	#[serde(rename = "0")]
	PlusTick,
	/// Zero-Plus Tick
	#[serde(rename = "1")]
	ZeroPlusTick,
	/// Minus Tick
	#[serde(rename = "2")]
	MinusTick,
	/// Zero-Minus Tick
	#[serde(rename = "3")]
	ZeroMinusTick,
}

impl Default for TickDirection {
	fn default() -> Self {
		TickDirection::PlusTick
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

impl Default for SecurityTradingStatus {
	fn default() -> Self {
		SecurityTradingStatus::OpeningDelay
	}
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

impl Default for HaltReason {
	fn default() -> Self {
		HaltReason::NewsDissemination
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
pub enum MatchType {
	/// ACT Accepted Trade
	#[serde(rename = "M3")]
	ActAcceptedTrade,
	/// ACT Default Trade
	#[serde(rename = "M4")]
	ActDefaultTrade,
	/// ACT Default After M2
	#[serde(rename = "M5")]
	ActDefaultAfterM2,
	/// ACT M6 Match
	#[serde(rename = "M6")]
	ActM6Match,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus four badges and execution
	/// time (within two-minute window)
	#[serde(rename = "A1")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus four badges
	#[serde(rename = "A2")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges and execution
	/// time (within two-minute window)
	#[serde(rename = "A3")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus two badges
	#[serde(rename = "A4")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadges,
	/// Exact match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator plus execution time (within
	/// two-minute window)
	#[serde(rename = "A5")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime,
	/// Compared records resulting from stamped advisories or specialist accepts/pair-offs
	#[serde(rename = "AQ")]
	ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAcceptsPairOffs,
	/// Summarized Match using A1 exact match criteria except quantity is summarized
	#[serde(rename = "S1")]
	SummarizedMatchUsingA1ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A2 exact match criteria except quantity is summarized
	#[serde(rename = "S2")]
	SummarizedMatchUsingA2ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A3 exact match criteria except quantity is summarized
	#[serde(rename = "S3")]
	SummarizedMatchUsingA3ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A4 exact match criteria except quantity is summarized
	#[serde(rename = "S4")]
	SummarizedMatchUsingA4ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Summarized Match using A5 exact match criteria except quantity is summarized
	#[serde(rename = "S5")]
	SummarizedMatchUsingA5ExactMatchCriteriaExceptQuantityIsSummarized,
	/// Exact Match on Trade Date, Stock Symbol, Quantity, Price, Trade Type, and Special Trade Indicator minus badges and times
	#[serde(rename = "M1")]
	ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorMinusBadgesAndTimes,
	/// Summarized Match minus badges and times
	#[serde(rename = "M2")]
	SummarizedMatchMinusBadgesAndTimes,
	/// OCS Locked In
	#[serde(rename = "MT")]
	OcsLockedIn,
	/// One-Party Trade Report (privately negotiated trade)
	#[serde(rename = "1")]
	OnePartyTradeReport,
	/// Two-Party Trade Report (privately negotiated trade)
	#[serde(rename = "2")]
	TwoPartyTradeReport,
	/// Confirmed Trade Report (reporting from recognized markets)
	#[serde(rename = "3")]
	ConfirmedTradeReport,
	/// Auto-match
	#[serde(rename = "4")]
	AutoMatch,
	/// Cross Auction
	#[serde(rename = "5")]
	CrossAuction,
	/// Counter-Order Selection
	#[serde(rename = "6")]
	CounterOrderSelection,
	/// Call Auction
	#[serde(rename = "7")]
	CallAuction,
	/// Issuing/Buy Back Auction
	#[serde(rename = "8")]
	IssuingBuyBackAuction,
	/// Systematic Internaliser (SI)
	#[serde(rename = "9")]
	SystematicInternaliser,
	/// Auto-match with last look
	#[serde(rename = "10")]
	AutoMatchWithLastLook,
	/// Cross auction with last look
	#[serde(rename = "11")]
	CrossAuctionWithLastLook,
}

impl Default for MatchType {
	fn default() -> Self {
		MatchType::ActAcceptedTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OpenCloseSettlFlag {
	/// Daily Open / Close / Settlement entry
	#[serde(rename = "0")]
	DailyOpenCloseSettlementEntry,
	/// Session Open / Close / Settlement entry
	#[serde(rename = "1")]
	SessionOpenCloseSettlementEntry,
	/// Delivery Settlement entry
	#[serde(rename = "2")]
	DeliverySettlementEntry,
	/// Expected entry
	#[serde(rename = "3")]
	ExpectedEntry,
	/// Entry from previous business day
	#[serde(rename = "4")]
	EntryFromPreviousBusinessDay,
	/// Theoretical Price value
	#[serde(rename = "5")]
	TheoreticalPriceValue,
}

impl Default for OpenCloseSettlFlag {
	fn default() -> Self {
		OpenCloseSettlFlag::DailyOpenCloseSettlementEntry
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Scope {
	/// Local (Exchange, ECN, ATS)
	#[serde(rename = "1")]
	Local,
	/// National
	#[serde(rename = "2")]
	National,
	/// Global
	#[serde(rename = "3")]
	Global,
}

impl Default for Scope {
	fn default() -> Self {
		Scope::Local
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlType {
	/// Regular / FX Spot settlement (T+1 or T+2 depending on currency)
	#[serde(rename = "0")]
	RegularFxSpotSettlement,
	/// Cash (TOD / T+0)
	#[serde(rename = "1")]
	Cash,
	/// Next Day (TOM / T+1)
	#[serde(rename = "2")]
	NextDay,
	/// T+2
	#[serde(rename = "3")]
	T2,
	/// T+3
	#[serde(rename = "4")]
	T3,
	/// T+4
	#[serde(rename = "5")]
	T4,
	/// Future
	#[serde(rename = "6")]
	Future,
	/// When And If Issued
	#[serde(rename = "7")]
	WhenAndIfIssued,
	/// Sellers Option
	#[serde(rename = "8")]
	SellersOption,
	/// T+5
	#[serde(rename = "9")]
	T5,
	/// Broken date - for FX expressing non-standard tenor, <a href="tag_64_SettlDate.html" target="bottom">SettlDate&nbsp;(64)</a> must be specified
	#[serde(rename = "B")]
	BrokenDateForFxExpressingNonStandardTenorAHrefTag64SettlDateHtmlTargetBottomSettlDateNbspAMustBeSpecified,
	/// FX Spot Next settlement (Spot+1, aka next day)
	#[serde(rename = "C")]
	FxSpotNextSettlement,
}

impl Default for SettlType {
	fn default() -> Self {
		SettlType::RegularFxSpotSettlement
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MDQuoteType {
	/// Indicative
	#[serde(rename = "0")]
	Indicative,
	/// Tradeable
	#[serde(rename = "1")]
	Tradeable,
	/// Restricted Tradeable
	#[serde(rename = "2")]
	RestrictedTradeable,
	/// Counter
	#[serde(rename = "3")]
	Counter,
	/// Indicative and Tradeable
	#[serde(rename = "4")]
	IndicativeAndTradeable,
}

impl Default for MDQuoteType {
	fn default() -> Self {
		MDQuoteType::Indicative
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DealingCapacity {
	/// Agent
	#[serde(rename = "A")]
	Agent,
	/// Principal
	#[serde(rename = "P")]
	Principal,
	/// Riskless Principal
	#[serde(rename = "R")]
	RisklessPrincipal,
}

impl Default for DealingCapacity {
	fn default() -> Self {
		DealingCapacity::Agent
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
pub enum OrderCategory {
	/// Order
	#[serde(rename = "1")]
	Order,
	/// Quote
	#[serde(rename = "2")]
	Quote,
	/// Privately Negotiated Trade
	#[serde(rename = "3")]
	PrivatelyNegotiatedTrade,
	/// Multileg order
	#[serde(rename = "4")]
	MultilegOrder,
	/// Linked order
	#[serde(rename = "5")]
	LinkedOrder,
	/// Quote Request
	#[serde(rename = "6")]
	QuoteRequest,
	/// Implied Order
	#[serde(rename = "7")]
	ImpliedOrder,
	/// Cross Order
	#[serde(rename = "8")]
	CrossOrder,
	/// Streaming price (quote)
	#[serde(rename = "9")]
	StreamingPrice,
	/// Internal Cross Order
	#[serde(rename = "A")]
	InternalCrossOrder,
}

impl Default for OrderCategory {
	fn default() -> Self {
		OrderCategory::Order
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TradePublishIndicator {
	/// Do Not Publish Trade
	#[serde(rename = "0")]
	DoNotPublishTrade,
	/// Publish Trade
	#[serde(rename = "1")]
	PublishTrade,
	/// Deferred Publication
	#[serde(rename = "2")]
	DeferredPublication,
	/// Published
	#[serde(rename = "3")]
	Published,
}

impl Default for TradePublishIndicator {
	fn default() -> Self {
		TradePublishIndicator::DoNotPublishTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecMethod {
	/// Undefined/unspecified - (default when not specified)
	#[serde(rename = "0")]
	UndefinedUnspecified,
	/// Manual (The transaction was executed in a manual or other non-automated manner e.g. by voice directly between the counterparties.
	/// Also used to identify MTT code M "Off Book Non-Automated".)
	#[serde(rename = "1")]
	Manual,
	/// Automated (The transaction was executed on an automated execution platform such as an automated systematic internaliser system,
	/// broker crossing network, broker crossing system, dark pool trading, "direct to capital" systems, broker position unwind mechanisms,
	/// etc.)
	#[serde(rename = "2")]
	Automated,
	/// Voice brokered
	#[serde(rename = "3")]
	VoiceBrokered,
}

impl Default for ExecMethod {
	fn default() -> Self {
		ExecMethod::UndefinedUnspecified
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlPriceDeterminationMethod {
	/// Unknown
	#[serde(rename = "0")]
	Unknown,
	/// Last trade price
	#[serde(rename = "1")]
	LastTradePrice,
	/// Last bid price
	#[serde(rename = "2")]
	LastBidPrice,
	/// Last offer price
	#[serde(rename = "3")]
	LastOfferPrice,
	/// Mid price (Average The price at the mid-point between last bid and last offer price)
	#[serde(rename = "4")]
	MidPrice,
	/// Average of last trade prices (The Aaverage price across a bi-laterally agreed number of trades, e.g. last five trades)
	#[serde(rename = "5")]
	AverageOfLastTradePrices,
	/// Average of last trade period (Average price across bilaterally agreed time period, e.g. last minute of trading)
	#[serde(rename = "6")]
	AverageOfLastTradePeriod,
	/// Underlying price (Based on price of underlying instrument)
	#[serde(rename = "7")]
	UnderlyingPrice,
	/// Calculated price (Other calculation method,e.g. theoretical price)
	#[serde(rename = "8")]
	CalculatedPrice,
	/// Manual price (Manually entered price)
	#[serde(rename = "9")]
	ManualPrice,
}

impl Default for SettlPriceDeterminationMethod {
	fn default() -> Self {
		SettlPriceDeterminationMethod::Unknown
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AggressorSide {
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
}

impl Default for AggressorSide {
	fn default() -> Self {
		AggressorSide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AlgorithmicTradeIndicator {
	/// Non-algorithmic trade
	#[serde(rename = "0")]
	NonAlgorithmicTrade,
	/// Algorithmic trade
	#[serde(rename = "1")]
	AlgorithmicTrade,
}

impl Default for AlgorithmicTradeIndicator {
	fn default() -> Self {
		AlgorithmicTradeIndicator::NonAlgorithmicTrade
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RegulatoryReportType {
	/// Real-time (RT)
	#[serde(rename = "0")]
	RealTime,
	/// Primary economic terms (PET)
	#[serde(rename = "1")]
	PrimaryEconomicTerms,
	/// Snapshot
	#[serde(rename = "2")]
	Snapshot,
	/// Confirmation
	#[serde(rename = "3")]
	Confirmation,
	/// Combination of RT and PET
	#[serde(rename = "4")]
	CombinationOfRtAndPet,
	/// Combination of PET and confirmation
	#[serde(rename = "5")]
	CombinationOfPetAndConfirmation,
	/// Combination of RT, PET and confirmation
	#[serde(rename = "6")]
	CombinationOfRtPetAndConfirmation,
	/// Post-trade valuation
	#[serde(rename = "7")]
	PostTradeValuation,
	/// Verification
	#[serde(rename = "8")]
	Verification,
	/// Post-trade event
	#[serde(rename = "9")]
	PostTradeEvent,
	/// Post trade event RT reportable (Report of a regulated transaction continuation event that falls within the requirements for
	/// real-time reporting and public dissemination. If dissemination is to be suppressed due to an end user exception or to local
	/// regulatory rules that allow suppression of certain types of transactions, use TradePublishIndicator(1390) = 0 (Do not publish
	/// trade)
	#[serde(rename = "10")]
	PostTradeEventRtReportable0,
	/// Limited Details Trade
	#[serde(rename = "11")]
	LimitedDetailsTrade,
	/// Daily Aggregated Trade
	#[serde(rename = "12")]
	DailyAggregatedTrade,
	/// Volume Omission Trade
	#[serde(rename = "13")]
	VolumeOmissionTrade,
	/// Four Weeks Aggregation Trade
	#[serde(rename = "14")]
	FourWeeksAggregationTrade,
	/// Indefinite Aggregation Trade
	#[serde(rename = "15")]
	IndefiniteAggregationTrade,
	/// Volume Omission Trade Eligible for Subsequent Aggregated Enrichment
	#[serde(rename = "16")]
	VolumeOmissionTradeEligibleForSubsequentAggregatedEnrichment,
	/// Full Details Trade of "Limited Details Trade"
	#[serde(rename = "17")]
	FullDetailsTradeOfLimitedDetailsTrade,
	/// Full Details of "Daily Aggregated Trade"
	#[serde(rename = "18")]
	FullDetailsOfDailyAggregatedTrade,
	/// Full Details of "Volume Omission Trade"
	#[serde(rename = "19")]
	FullDetailsOfVolumeOmissionTrade,
	/// Full Details of "Four Weeks Aggregation Trade"
	#[serde(rename = "20")]
	FullDetailsOfFourWeeksAggregationTrade,
	/// Full Details in Aggregated Form of "Volume Omission Trade Eligible for Subsequent Aggregated Enrichment"
	#[serde(rename = "21")]
	FullDetailsInAggregatedFormOfVolumeOmissionTradeEligibleForSubsequentAggregatedEnrichment,
	/// Order
	#[serde(rename = "22")]
	Order,
	/// Child order
	#[serde(rename = "23")]
	ChildOrder,
	/// Order route
	#[serde(rename = "24")]
	OrderRoute,
	/// Trade
	#[serde(rename = "25")]
	Trade,
	/// Quote
	#[serde(rename = "26")]
	Quote,
	/// Supplement
	#[serde(rename = "27")]
	Supplement,
	/// New transaction
	#[serde(rename = "28")]
	NewTransaction,
	/// Transaction correction
	#[serde(rename = "29")]
	TransactionCorrection,
	/// Transaction modification
	#[serde(rename = "30")]
	TransactionModification,
	/// Collateral update
	#[serde(rename = "31")]
	CollateralUpdate,
	/// Margin update
	#[serde(rename = "32")]
	MarginUpdate,
	/// Transaction reported in error
	#[serde(rename = "33")]
	TransactionReportedInError,
	/// Termination / Early termination
	#[serde(rename = "34")]
	TerminationEarlyTermination,
}

impl Default for RegulatoryReportType {
	fn default() -> Self {
		RegulatoryReportType::RealTime
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PreviouslyReported {
	/// Not reported to counterparty or market
	#[serde(rename = "N")]
	NotReportedToCounterpartyOrMarket,
	/// Previously reported to counterparty or market
	#[serde(rename = "Y")]
	PreviouslyReportedToCounterpartyOrMarket,
}

impl Default for PreviouslyReported {
	fn default() -> Self {
		PreviouslyReported::NotReportedToCounterpartyOrMarket
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MarketCondition {
	/// Normal
	#[serde(rename = "0")]
	Normal,
	/// Stressed
	#[serde(rename = "1")]
	Stressed,
	/// Exceptional
	#[serde(rename = "2")]
	Exceptional,
}

impl Default for MarketCondition {
	fn default() -> Self {
		MarketCondition::Normal
	}
}
