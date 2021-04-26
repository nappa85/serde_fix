
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDataRequest {
	/// MsgType = V
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'V', ' '>,
	/// Must be unique, or the ID of previous <a href="message_Market_Data_Request_V.html" target="main">Market Data Request&nbsp;(V)</a> to disable if <a href="tag_263_SubscriptionRequestType.html" target="bottom">SubscriptionRequestType&nbsp;(263)</a> = Disable previous Snapshot + Updates Request (2).
	#[serde(rename = "262")]
	pub md_req_id: String,
	/// <a href="tag_263_SubscriptionRequestType.html" target="bottom">SubscriptionRequestType&nbsp;(263)</a> indicates to the other party what type of response is expected. A snapshot request only asks for current information. A subscribe
	/// request asks for updates as the status changes. Unsubscribe will cancel any future update messages from the counter party.
	#[serde(rename = "263")]
	pub subscription_request_type: SubscriptionRequestType,
	/// MarketDepth
	#[serde(rename = "264")]
	pub market_depth: u32,
	/// Required if <a href="tag_263_SubscriptionRequestType.html" target="bottom">SubscriptionRequestType&nbsp;(263)</a> = Snapshot + Updates (1).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "265")]
	pub md_update_type: Option<MDUpdateType>,
	/// AggregatedBook
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "266")]
	pub aggregated_book: Option<AggregatedBook>,
	/// Can be used to clarify a request if <a href="tag_269_MDEntryType.html" target="bottom">MDEntryType&nbsp;(269)</a> = Opening Price(4), Closing Price(5), or Settlement Price(6).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "286")]
	pub open_close_settl_flag: Option<fix_common::SeparatedValues<OpenCloseSettlFlag>>,
	/// Defines the scope(s) of the request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "546")]
	pub scope: Option<fix_common::SeparatedValues<Scope>>,
	/// Can be used when <a href="tag_264_MarketDepth.html" target="bottom">MarketDepth&nbsp;(264)</a> &gt;= 2 and <a href="tag_265_MDUpdateType.html" target="bottom">MDUpdateType&nbsp;(265)</a> = Incremental Refresh(1).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "547")]
	pub md_implicit_delete: Option<MDImplicitDelete>,
	/// Number of <a href="tag_269_MDEntryType.html" target="bottom">MDEntryType&nbsp;(269)</a> fields requested.
	#[serde(rename = "267")]
	pub md_entry_types: fix_common::RepeatingValues<MDEntryType>,
	/// Number of symbols (instruments) requested.
	#[serde(rename = "146")]
	pub related_sym: fix_common::RepeatingValues<RelatedSy>,
	/// Number of trading sessions for which the request is valid.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "386")]
	pub trading_sessions: Option<fix_common::RepeatingValues<TradingSession>>,
	/// Action to take if application level queuing exists
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "815")]
	pub appl_queue_action: Option<ApplQueueAction>,
	/// Maximum application queue depth that must be exceeded before queuing action is taken.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "812")]
	pub appl_queue_max: Option<i32>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDEntryType {
	/// Must be the first field in this repeating group. This is a list of all the types of Market Data Entries that the firm requesting
	/// the Market Data is interested in receiving.
	#[serde(rename = "269")]
	pub md_entry_type_item: MDEntryTypeItem,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedSy {
    #[serde(flatten)]
    pub instruments: fix_common::RepeatingValues<super::super::instrument::Instrument>,
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<super::super::underlying_instrument::UnderlyingInstrument>>,
	/// Used for multileg instruments
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<super::super::instrument_leg::InstrumentLeg>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TradingSession {
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDUpdateType {
	/// Full Refresh
	#[serde(rename = "0")]
	FullRefresh,
	/// Incremental Refresh
	#[serde(rename = "1")]
	IncrementalRefresh,
}

impl Default for MDUpdateType {
	fn default() -> Self {
		MDUpdateType::FullRefresh
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AggregatedBook {
	/// one book entry per side per price
	#[serde(rename = "Y")]
	OneBookEntryPerSidePerPrice,
	/// Multiple entries per side per price allowed(Not specified) = broker option
	#[serde(rename = "N")]
	MultipleEntriesPerSidePerPriceAllowedBrokerOption,
}

impl Default for AggregatedBook {
	fn default() -> Self {
		AggregatedBook::OneBookEntryPerSidePerPrice
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDImplicitDelete {
	/// Client has responsibility for implicitly deleting bids or offers falling outside the MarketDepth of the request.
	#[serde(rename = "Y")]
	ClientHasResponsibilityForImplicitlyDeletingBidsOrOffersFallingOutsideTheMarketDepthOfTheRequest,
	/// Server must send an explicit delete for bids or offers falling outside the requested MarketDepth of the request.
	#[serde(rename = "N")]
	ServerMustSendAnExplicitDeleteForBidsOrOffersFallingOutsideTheRequestedMarketDepthOfTheRequest,
}

impl Default for MDImplicitDelete {
	fn default() -> Self {
		MDImplicitDelete::ClientHasResponsibilityForImplicitlyDeletingBidsOrOffersFallingOutsideTheMarketDepthOfTheRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ApplQueueAction {
	/// No action taken
	#[serde(rename = "0")]
	NoActionTaken,
	/// Queue flushed
	#[serde(rename = "1")]
	QueueFlushed,
	/// Overlay last
	#[serde(rename = "2")]
	OverlayLast,
	/// End session
	#[serde(rename = "3")]
	EndSession,
}

impl Default for ApplQueueAction {
	fn default() -> Self {
		ApplQueueAction::NoActionTaken
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for MDEntryTypeItem {
	fn default() -> Self {
		MDEntryTypeItem::Bid
	}
}
