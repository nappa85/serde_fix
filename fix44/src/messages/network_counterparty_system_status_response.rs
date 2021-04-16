
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NetworkCounterpartySystemStatusResponse {
	/// MsgType = BD
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'B', 'D'>,
	/// NetworkStatusResponseType
	#[serde(rename = "937")]
	pub network_status_response_type: NetworkStatusResponseType,
	/// NetworkRequestID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "933")]
	pub network_request_id: Option<String>,
	/// NetworkResponseID
	#[serde(rename = "932")]
	pub network_response_id: String,
	/// Required when <a href="tag_937_NetworkStatusResponseType.html" target="bottom">NetworkStatusResponseType&nbsp;(937)</a> =2
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "934")]
	pub last_network_response_id: Option<String>,
	/// Specifies the number of repeating CompId's
	#[serde(rename = "936")]
	pub comp_i_ds: fix_common::RepeatingValues<CompID>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CompID {
	/// CompID that status is being report for. Required if <a href="tag_936_NoCompIDs.html" target="bottom">NoCompIDs&nbsp;(936)</a> &gt; 0.
	#[serde(rename = "930")]
	pub ref_comp_id: String,
	/// SubID that status is being report for.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "931")]
	pub ref_sub_id: Option<String>,
	/// LocationID that status is being report for.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "283")]
	pub location_id: Option<String>,
	/// DeskID that status is being report for.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "284")]
	pub desk_id: Option<String>,
	/// StatusValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "928")]
	pub status_value: Option<StatusValue>,
	/// Additional information, i.e. "National Holiday"
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "929")]
	pub status_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum NetworkStatusResponseType {
	/// Full
	#[serde(rename = "1")]
	Full,
	/// Incremental update
	#[serde(rename = "2")]
	IncrementalUpdate,
}

impl Default for NetworkStatusResponseType {
	fn default() -> Self {
		NetworkStatusResponseType::Full
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StatusValue {
	/// Connected
	#[serde(rename = "1")]
	Connected,
	/// Not connected - down expected up
	#[serde(rename = "2")]
	NotConnectedDownExpectedUp,
	/// Not connected - down expected down
	#[serde(rename = "3")]
	NotConnectedDownExpectedDown,
	/// In Process
	#[serde(rename = "4")]
	InProcess,
}

impl Default for StatusValue {
	fn default() -> Self {
		StatusValue::Connected
	}
}
