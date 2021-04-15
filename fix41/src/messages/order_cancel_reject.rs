
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderCancelReject {
	/// MsgType = 9
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'9'>,
	/// OrderID
	#[serde(rename = "37")]
	pub order_id: char,
	/// Can be used to provide order id used by exchange or executing system.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<char>,
	/// Unique order id assigned by institution to the cancel request or to the replacement order.
	#[serde(rename = "11")]
	pub cl_ord_id: char,
	/// <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> which could not be canceled/replaced. <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> of the previous order (NOT the initial order of the day) when canceling or replacing an order.
	#[serde(rename = "41")]
	pub orig_cl_ord_id: char,
	/// OrdStatus value after this cancel reject is applied.
	#[serde(rename = "39")]
	pub ord_status: OrdStatus,
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
pub enum OrdStatus {
	/// New
	#[serde(rename = "0")]
	New,
	/// Partially filled
	#[serde(rename = "1")]
	PartiallyFilled,
	/// Filled
	#[serde(rename = "2")]
	Filled,
	/// Done for day
	#[serde(rename = "3")]
	DoneForDay,
	/// Canceled
	#[serde(rename = "4")]
	Canceled,
	/// Replaced
	#[serde(rename = "5")]
	Replaced,
	/// Pending Cancel/Replace
	#[serde(rename = "6")]
	PendingCancelReplace,
	/// Stopped
	#[serde(rename = "7")]
	Stopped,
	/// Rejected
	#[serde(rename = "8")]
	Rejected,
	/// Suspended
	#[serde(rename = "9")]
	Suspended,
	/// Pending New
	#[serde(rename = "A")]
	PendingNew,
	/// Calculated
	#[serde(rename = "B")]
	Calculated,
	/// Expired
	#[serde(rename = "C")]
	Expired,
}

impl Default for OrdStatus {
	fn default() -> Self {
		OrdStatus::New
	}
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
