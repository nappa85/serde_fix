
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Stream {
	/// MsgType = CC
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier of the request.
	#[serde(rename = "1497")]
	pub stream_asgn_req_id: String,
	/// Type of assignment being requested.
	#[serde(rename = "1498")]
	pub stream_asgn_req_type: StreamAsgnReqType,
	/// Assignment requests
	#[serde(flatten)]
	pub strm_asgn_req_grp: super::super::strm_asgn_req_grp::StrmAsgnReqGrp,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StreamAsgnReqType {
	/// Stream assignment for new customer(s)
	#[serde(rename = "1")]
	StreamAssignmentForNewCustomer,
	/// Stream assignment for existing customer(s)
	#[serde(rename = "2")]
	StreamAssignmentForExistingCustomer,
}

impl Default for StreamAsgnReqType {
	fn default() -> Self {
		StreamAsgnReqType::StreamAssignmentForNewCustomer
	}
}
