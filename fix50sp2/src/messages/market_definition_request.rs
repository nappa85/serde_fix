
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDefinitionRequest {
	/// MsgType = BT
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Must be unique, or the ID of previous Market Segment Request to disable if SubscriptionRequestType = Disable previous Snapshot
	/// + Updates Request(2).
	#[serde(rename = "1393")]
	pub market_req_id: String,
	/// SubscriptionRequestType
	#[serde(rename = "263")]
	pub subscription_request_type: SubscriptionRequestType,
	/// Conditionally required if MarketSegmentID(1300) is specified on the request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1301")]
	pub market_id: Option<String>,
	/// MarketSegmentID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1300")]
	pub market_segment_id: Option<String>,
	/// Specifies that the Market Segment is a sub segment of the Market Segment defined in this field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1325")]
	pub parent_mkt_segm_id: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
