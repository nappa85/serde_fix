
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Bid {
	/// MsgType = l
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// BidID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "390")]
	pub bid_id: Option<String>,
	/// ClientBidID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "391")]
	pub client_bid_id: Option<String>,
	/// BidCompRspGrp
	#[serde(flatten)]
	pub bid_comp_rsp_grp: super::super::bid_comp_rsp_grp::BidCompRspGrp,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}
