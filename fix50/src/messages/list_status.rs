
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ListStatus {
	/// MsgType = N
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// ListID
	#[serde(rename = "66")]
	pub list_id: String,
	/// ListStatusType
	#[serde(rename = "429")]
	pub list_status_type: ListStatusType,
	/// Total number of messages required to status complete list.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "82")]
	pub no_rpts: i32,
	/// ListOrderStatus
	#[serde(rename = "431")]
	pub list_order_status: ListOrderStatus,
	/// Sequence number of this report message.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "83")]
	pub rpt_seq: i32,
	/// ListStatusText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "444")]
	pub list_status_text: Option<String>,
	/// Must be set if <a href="tag_446_EncodedListStatusText.html" target="bottom">EncodedListStatusText&nbsp;(446)</a> field is specified and must immediately precede it.
	#[serde(rename = "445")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_444_ListStatusText.html" target="bottom">ListStatusText&nbsp;(444)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "446")]
	pub encoded_list_status_text: Option<fix_common::EncodedText<446>>,
	/// TransactTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
	/// Used to support fragmentation. Sum of <a href="tag_73_NoOrders.html" target="bottom">NoOrders&nbsp;(73)</a> across all messages with the same ListID.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "68")]
	pub tot_no_orders: i32,
	/// Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "893")]
	pub last_fragment: Option<LastFragment>,
	/// Number of orders statused in this message, i.e. number of repeating groups to follow.
	#[serde(rename = "73")]
	pub orders: fix_common::RepeatingValues<Order>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Order {
	/// ClOrdID
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// SecondaryClOrdID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "526")]
	pub secondary_cl_ord_id: Option<String>,
	/// CumQty
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "14")]
	pub cum_qty: f64,
	/// OrdStatus
	#[serde(rename = "39")]
	pub ord_status: OrdStatus,
	/// For optional use with <a href="tag_39_OrdStatus.html" target="bottom">OrdStatus&nbsp;(39)</a> = 0 (New)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "636")]
	pub working_indicator: Option<WorkingIndicator>,
	/// Quantity open for further execution. <a href="tag_151_LeavesQty.html" target="bottom">LeavesQty&nbsp;(151)</a> = <a href="tag_38_OrderQty.html" target="bottom">OrderQty&nbsp;(38)</a> - CumQty.
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "151")]
	pub leaves_qty: f64,
	/// CxlQty
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "84")]
	pub cxl_qty: f64,
	/// AvgPx
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "6")]
	pub avg_px: f64,
	/// Used if the order is rejected
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "103")]
	pub ord_rej_reason: Option<OrdRejReason>,
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListStatusType {
	/// Ack
	#[serde(rename = "1")]
	Ack,
	/// Response
	#[serde(rename = "2")]
	Response,
	/// Timed
	#[serde(rename = "3")]
	Timed,
	/// Exec Started
	#[serde(rename = "4")]
	ExecStarted,
	/// All Done
	#[serde(rename = "5")]
	AllDone,
	/// Alert
	#[serde(rename = "6")]
	Alert,
}

impl Default for ListStatusType {
	fn default() -> Self {
		ListStatusType::Ack
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListOrderStatus {
	/// In bidding process
	#[serde(rename = "1")]
	InBiddingProcess,
	/// Received for execution
	#[serde(rename = "2")]
	ReceivedForExecution,
	/// Executing
	#[serde(rename = "3")]
	Executing,
	/// Cancelling
	#[serde(rename = "4")]
	Cancelling,
	/// Alert
	#[serde(rename = "5")]
	Alert,
	/// All Done
	#[serde(rename = "6")]
	AllDone,
	/// Reject
	#[serde(rename = "7")]
	Reject,
}

impl Default for ListOrderStatus {
	fn default() -> Self {
		ListOrderStatus::InBiddingProcess
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LastFragment {
	/// Not Last Message
	#[serde(rename = "N")]
	NotLastMessage,
	/// Last Message
	#[serde(rename = "Y")]
	LastMessage,
}

impl Default for LastFragment {
	fn default() -> Self {
		LastFragment::NotLastMessage
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	/// Replaced (No longer used)
	#[serde(rename = "5")]
	Replaced,
	/// Pending Cancel (i.e. result of Order Cancel Request)
	#[serde(rename = "6")]
	PendingCancel,
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
	/// Accepted for Bidding
	#[serde(rename = "D")]
	AcceptedForBidding,
	/// Pending Replace (i.e. result of Order Cancel/Replace Request)
	#[serde(rename = "E")]
	PendingReplace,
}

impl Default for OrdStatus {
	fn default() -> Self {
		OrdStatus::New
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WorkingIndicator {
	/// Order has been accepted but not yet in a working state
	#[serde(rename = "N")]
	OrderHasBeenAcceptedButNotYetInAWorkingState,
	/// Order is currently being worked
	#[serde(rename = "Y")]
	OrderIsCurrentlyBeingWorked,
}

impl Default for WorkingIndicator {
	fn default() -> Self {
		WorkingIndicator::OrderHasBeenAcceptedButNotYetInAWorkingState
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrdRejReason {
	/// Broker / Exchange option
	#[serde(rename = "0")]
	BrokerExchangeOption,
	/// Unknown symbol
	#[serde(rename = "1")]
	UnknownSymbol,
	/// Exchange closed
	#[serde(rename = "2")]
	ExchangeClosed,
	/// Order exceeds limit
	#[serde(rename = "3")]
	OrderExceedsLimit,
	/// Too late to enter
	#[serde(rename = "4")]
	TooLateToEnter,
	/// Unknown order
	#[serde(rename = "5")]
	UnknownOrder,
	/// Duplicate Order (e.g. dupe ClOrdID)
	#[serde(rename = "6")]
	DuplicateOrder,
	/// Duplicate of a verbally communicated order
	#[serde(rename = "7")]
	DuplicateOfAVerballyCommunicatedOrder,
	/// Stale order
	#[serde(rename = "8")]
	StaleOrder,
	/// Trade along required
	#[serde(rename = "9")]
	TradeAlongRequired,
	/// Invalid Investor ID
	#[serde(rename = "10")]
	InvalidInvestorId,
	/// Unsupported order characteristic
	#[serde(rename = "11")]
	UnsupportedOrderCharacteristic,
	/// Surveillence Option
	#[serde(rename = "12")]
	SurveillenceOption,
	/// Incorrect quantity
	#[serde(rename = "13")]
	IncorrectQuantity,
	/// Incorrect allocated quantity
	#[serde(rename = "14")]
	IncorrectAllocatedQuantity,
	/// Unknown account(s)
	#[serde(rename = "15")]
	UnknownAccount,
	/// Invalid price increment
	#[serde(rename = "18")]
	InvalidPriceIncrement,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for OrdRejReason {
	fn default() -> Self {
		OrdRejReason::BrokerExchangeOption
	}
}
