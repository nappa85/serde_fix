
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Market {
	/// MsgType = X
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ApplicationSequenceControl
	#[serde(flatten)]
	pub application_sequence_control: Option<super::super::application_sequence_control::ApplicationSequenceControl>,
	/// Describes the type of book for which the feed is intended. Can be used when multiple feeds are provided over the same connection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1021")]
	pub md_book_type: Option<MDBookType>,
	/// Describes a class of service for a given data feed, ie Regular and Market Maker
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1022")]
	pub md_feed_type: Option<String>,
	/// Used to specify the trading date for which a set of market data applies
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// Conditionally required if this message is in response to a Market Data Request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "262")]
	pub md_req_id: Option<String>,
	/// Number of entries following.
	#[serde(flatten)]
	pub md_inc_grp: super::super::md_inc_grp::MDIncGrp,
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
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
