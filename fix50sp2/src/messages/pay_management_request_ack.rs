
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PayManagementRequestAck {
	/// MsgType = EB
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// PayRequestID
	#[serde(rename = "2812")]
	pub pay_request_id: String,
	/// Only PayRequestStuats(2813)=0 (Received) is applicable in this message.
	#[serde(rename = "2813")]
	pub pay_request_status: PayRequestStatus,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PayRequestStatus {
	/// Received, not yet processed
	#[serde(rename = "0")]
	ReceivedNotYetProcessed,
	/// Accepted
	#[serde(rename = "1")]
	Accepted,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
	/// Disputed
	#[serde(rename = "3")]
	Disputed,
}

impl Default for PayRequestStatus {
	fn default() -> Self {
		PayRequestStatus::ReceivedNotYetProcessed
	}
}
