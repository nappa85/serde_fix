
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDataRequest {
	/// MsgType = V
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'V', ' '>,
	/// Must be unique, or the ID of previous <a href="message_Market_Data_Request_V.html" target="main">Market Data Request&nbsp;(V)</a> to disable if <a href="tag_263_SubscriptionRequestType.html" target="bottom">SubscriptionRequestType&nbsp;(263)</a> = '2' (Disable previous Snapshot + Updates Request).
	#[serde(rename = "262")]
	pub md_req_id: String,
	/// SubscriptionRequestType indicates to the other party what type of response is expected. A snapshot request only asks for current
	/// information. A subscribe request asks for updates as the status changes. Unsubscribe will cancel any future update messages
	/// from the counter party.
	#[serde(rename = "263")]
	pub subscription_request_type: SubscriptionRequestType,
	/// Insert here the set of Parties (firm identification) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// MarketDepth
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
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
	/// MDReqGrp 
	#[serde(flatten)]
	pub md_req_grp: Option<super::super::md_req_grp::MDReqGrp>,
	/// InstrmtMDReqGrp
	#[serde(flatten)]
	pub instrmt_md_req_grp: super::super::instrmt_md_req_grp::InstrmtMDReqGrp,
	/// TrdgSesGrp
	#[serde(flatten)]
	pub trdg_ses_grp: Option<super::super::trdg_ses_grp::TrdgSesGrp>,
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
	/// MDQuoteType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1070")]
	pub md_quote_type: Option<MDQuoteType>,
	/// Can be used to limit the result set to the specified markets or market segments
	#[serde(flatten)]
	pub market_segment_scope_grp: Option<super::super::market_segment_scope_grp::MarketSegmentScopeGrp>,
	/// FastMarketIndicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2447")]
	pub fast_market_indicator: Option<fix_common::Boolean>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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
	/// Full refresh
	#[serde(rename = "0")]
	FullRefresh,
	/// Incremental refresh
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
	/// book entries to be aggregated
	#[serde(rename = "Y")]
	BookEntriesToBeAggregated,
	/// book entries should not be aggregated
	#[serde(rename = "N")]
	BookEntriesShouldNotBeAggregated,
}

impl Default for AggregatedBook {
	fn default() -> Self {
		AggregatedBook::BookEntriesToBeAggregated
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
	/// Server must send an explicit delete for bids or offers falling outside the requested MarketDepth of the request
	#[serde(rename = "N")]
	ServerMustSendAnExplicitDeleteForBidsOrOffersFallingOutsideTheRequestedMarketDepthOfTheRequest,
	/// Client has responsibility for implicitly deleting bids or offers falling outside the MarketDepth of the request
	#[serde(rename = "Y")]
	ClientHasResponsibilityForImplicitlyDeletingBidsOrOffersFallingOutsideTheMarketDepthOfTheRequest,
}

impl Default for MDImplicitDelete {
	fn default() -> Self {
		MDImplicitDelete::ServerMustSendAnExplicitDeleteForBidsOrOffersFallingOutsideTheRequestedMarketDepthOfTheRequest
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ApplQueueAction {
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

impl Default for ApplQueueAction {
	fn default() -> Self {
		ApplQueueAction::NoActionTaken
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
