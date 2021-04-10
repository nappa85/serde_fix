
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AllocationInstructionAlertRequest {
	/// MsgType = DU
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Unique identifier for this message. If used, other allocation messages may link to the request through this field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2758")]
	pub alloc_request_id: Option<String>,
	/// AllocGroupID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1730")]
	pub alloc_group_id: Option<String>,
	/// AvgPxGroupID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1731")]
	pub avg_px_group_id: Option<String>,
	/// TradeDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}
