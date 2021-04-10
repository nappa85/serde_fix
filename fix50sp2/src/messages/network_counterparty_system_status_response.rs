
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Network {
	/// MsgType = BD
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
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
	/// Required when NetworkStatusResponseType=2
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "934")]
	pub last_network_response_id: Option<String>,
	/// Specifies the number of repeating CompId's
	#[serde(flatten)]
	pub comp_id_stat_grp: super::super::comp_id_stat_grp::CompIDStatGrp,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum NetworkStatusResponseType {
	/// Full
	#[serde(rename = "1")]
	Full,
	/// Incremental Update
	#[serde(rename = "2")]
	IncrementalUpdate,
}
