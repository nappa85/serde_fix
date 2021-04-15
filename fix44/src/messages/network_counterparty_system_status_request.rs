
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
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "936")]
	pub comp_i_ds: Option<fix_common::RepeatingValues<CompID>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CompID {
	/// Used to restrict updates/request to specific CompID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "930")]
	pub ref_comp_id: Option<String>,
	/// Used to restrict updates/request to specific SubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "931")]
	pub ref_sub_id: Option<String>,
	/// Used to restrict updates/request to specific LocationID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "283")]
	pub location_id: Option<String>,
	/// Used to restrict updates/request to specific DeskID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "284")]
	pub desk_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NetworkRequestType {
	/// Snapshot
	#[serde(rename = "1")]
	Snapshot,
	/// Subscribe
	#[serde(rename = "2")]
	Subscribe,
	/// Stop subscribing
	#[serde(rename = "4")]
	StopSubscribing,
	/// Level of detail, then <a href="tag_936_NoCompIDs.html" target="bottom">NoCompIDs&nbsp;(936)</a> becomes required
	#[serde(rename = "8")]
	LevelOfDetailThenAHrefTag936NoCompIDsHtmlTargetBottomNoCompIDsNbspABecomesRequired,
}

impl Default for NetworkRequestType {
	fn default() -> Self {
		NetworkRequestType::Snapshot
	}
}
