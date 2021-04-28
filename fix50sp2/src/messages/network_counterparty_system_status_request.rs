
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NetworkCounterpartySystemStatusRequest {
	/// MsgType = BC
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'B', 'C'>,
	/// NetworkRequestType
	#[serde(rename = "935")]
	pub network_request_type: NetworkRequestType,
	/// NetworkRequestID
	#[serde(rename = "933")]
	pub network_request_id: String,
	/// Used to restrict updates/request to a list of specific CompID/SubID/LocationID/DeskID combinations. If not present request
	/// applies to all applicable available counterparties. EG Unless one sell side broker was a customer of another you would not
	/// expect to see information about other brokers, similarly one fund manager etc.
	#[serde(flatten)]
	pub comp_id_req_grp: Option<super::super::comp_id_req_grp::CompIDReqGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NetworkRequestType {
	/// Snapshot
	#[serde(rename = "1")]
	Snapshot,
	/// Subscribe
	#[serde(rename = "2")]
	Subscribe,
	/// Stop Subscribing
	#[serde(rename = "4")]
	StopSubscribing,
	/// Level of Detail, then <a href="tag_936_NoCompIDs.html" target="bottom">NoCompIDs&nbsp;(936)</a> becomes required
	#[serde(rename = "8")]
	LevelOfDetailThenNoCompIDsBecomesRequired,
}

impl Default for NetworkRequestType {
	fn default() -> Self {
		NetworkRequestType::Snapshot
	}
}
