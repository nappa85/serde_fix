
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StreamAssignmentReport {
	/// MsgType = CD
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier of the Stream Assignment Report.
	#[serde(rename = "1501")]
	pub stream_asgn_rpt_id: String,
	/// Required if report is being sent in response to a StreamAssignmentRequest. The value should be the same as the value in the
	/// corresponding request.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1498")]
	pub stream_asgn_req_type: Option<StreamAsgnReqType>,
	/// Conditionally required if Stream Assignment Report is being sent in response to a StreamAssignmentRequest(MsgType=CC). Not
	/// required for unsolicited stream assignments.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1497")]
	pub stream_asgn_req_id: Option<String>,
	/// Stream assignments
	#[serde(flatten)]
	pub strm_asgn_rpt_grp: Option<super::super::strm_asgn_rpt_grp::StrmAsgnRptGrp>,
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
