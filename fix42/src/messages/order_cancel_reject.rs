
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderCancelReject {
	/// MsgType = 9
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'9'>,
	/// If <a href="tag_102_CxlRejReason.html" target="bottom">CxlRejReason&nbsp;(102)</a> ="Unknown order", specify "NONE".
	#[serde(rename = "37")]
	pub order_id: String,
	/// Can be used to provide order id used by exchange or executing system.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "198")]
	pub secondary_order_id: Option<String>,
	/// Unique order id assigned by institution to the cancel request or to the replacement order.
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> which could not be canceled/replaced. <a href="tag_11_ClOrdID.html" target="bottom">ClOrdID&nbsp;(11)</a> of the previous order (NOT the initial order of the day) when canceling or replacing an order.
	#[serde(rename = "41")]
	pub orig_cl_ord_id: String,
	/// OrdStatus value after this cancel reject is applied.
	#[serde(rename = "39")]
	pub ord_status: OrdStatus,
	/// Used for firm identification in third-party transactions (should not be a substitute for <a href="tag_115_OnBehalfOfCompID.html" target="bottom">OnBehalfOfCompID&nbsp;(115)</a> / <a href="tag_128_DeliverToCompID.html" target="bottom">DeliverToCompID&nbsp;(128)</a> ).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "109")]
	pub client_id: Option<String>,
	/// Used for firm identification in third-party transactions (should not be a substitute for <a href="tag_115_OnBehalfOfCompID.html" target="bottom">OnBehalfOfCompID&nbsp;(115)</a> / <a href="tag_128_DeliverToCompID.html" target="bottom">DeliverToCompID&nbsp;(128)</a> ).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "76")]
	pub exec_broker: Option<String>,
	/// Required for rejects against orders which were submitted as part of a list.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "66")]
	pub list_id: Option<String>,
	/// Account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// CxlRejResponseTo
	#[serde(rename = "434")]
	pub cxl_rej_response_to: CxlRejResponseTo,
	/// CxlRejReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "102")]
	pub cxl_rej_reason: Option<CxlRejReason>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(rename = "354")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "355")]
	pub encoded_text: Option<fix_common::EncodedText<355>>,
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
	/// Pending Cancel (e.g. result of <a href="message_Order_Cancel_Request_F.html" target="main">Order Cancel Request&nbsp;(F)</a> )
	#[serde(rename = "6")]
	PendingCancelA,
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
	/// Accepted for bidding
	#[serde(rename = "D")]
	AcceptedForBidding,
	/// Pending Replace (e.g. result of <a href="message_Order_Cancel_Replace_Request_G.html" target="main">Order Cancel/Replace Request&nbsp;(G)</a> )
	#[serde(rename = "E")]
	PendingReplaceA,
}

impl Default for OrdStatus {
	fn default() -> Self {
		OrdStatus::New
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CxlRejResponseTo {
	/// Order Cancel Request
	#[serde(rename = "1")]
	OrderCancelRequest,
	/// Order Cancel/Replace Request
	#[serde(rename = "2")]
	OrderCancelReplaceRequest,
}

impl Default for CxlRejResponseTo {
	fn default() -> Self {
		CxlRejResponseTo::OrderCancelRequest
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
	/// Broker Option
	#[serde(rename = "2")]
	BrokerOption,
	/// Order already in Pending Cancel or Pending Replace status
	#[serde(rename = "3")]
	OrderAlreadyInPendingCancelOrPendingReplaceStatus,
}

impl Default for CxlRejReason {
	fn default() -> Self {
		CxlRejReason::TooLateToCancel
	}
}
