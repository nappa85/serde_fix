
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderCancelReject {
	/// MsgType = 9
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'9'>,
	/// OrderID
	#[serde(rename = "37")]
	pub order_id: char,
	/// Unique order id assigned by institution to the cancel request or to the replacement order.
	#[serde(rename = "11")]
	pub cl_ord_id: char,
	/// Used for firm identification in third-party transactions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "109")]
	pub client_id: Option<char>,
	/// Used for firm identification in third-party transactions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "76")]
	pub exec_broker: Option<char>,
	/// Required for rejects against orders which were submitted as part of a list.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "66")]
	pub list_id: Option<char>,
	/// CxlRejReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "102")]
	pub cxl_rej_reason: Option<CxlRejReason>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<char>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CxlRejReason {
	/// Too late to cancel
	#[serde(rename = "0")]
	TooLateToCancel,
	/// Unknown order
	#[serde(rename = "1")]
	UnknownOrder,
}

impl Default for CxlRejReason {
	fn default() -> Self {
		CxlRejReason::TooLateToCancel
	}
}
