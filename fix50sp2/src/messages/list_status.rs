
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ListStatus {
	/// MsgType = N
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'N', ' '>,
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
	/// ContingencyType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1385")]
	pub contingency_type: Option<ContingencyType>,
	/// ListRejectReason
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1386")]
	pub list_reject_reason: Option<ListRejectReason>,
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
	#[serde(flatten)]
	pub ord_list_stat_grp: super::super::ord_list_stat_grp::OrdListStatGrp,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ContingencyType {
	/// One Cancels the Other (OCO)
	#[serde(rename = "1")]
	OneCancelsTheOther,
	/// One Triggers the Other (OTO)
	#[serde(rename = "2")]
	OneTriggersTheOther,
	/// One Updates the Other (OUO) - Absolute Quantity Reduction
	#[serde(rename = "3")]
	OneUpdatesTheOtherAbsoluteQuantityReduction,
	/// One Updates the Other (OUO) - Proportional Quantity Reduction
	#[serde(rename = "4")]
	OneUpdatesTheOtherProportionalQuantityReduction,
	/// Bid and Offer (defines a contingency order with two or more sides)
	#[serde(rename = "5")]
	BidAndOffer,
	/// Bid and Offer OCO (defines a contingency order with two or more sides however one side is automatically canceled when the
	/// other side is fully traded away)
	#[serde(rename = "6")]
	BidAndOfferOco,
}

impl Default for ContingencyType {
	fn default() -> Self {
		ContingencyType::OneCancelsTheOther
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ListRejectReason {
	/// Broker / Exchange option
	#[serde(rename = "0")]
	BrokerExchangeOption,
	/// Exchange closed
	#[serde(rename = "2")]
	ExchangeClosed,
	/// Too late to enter
	#[serde(rename = "4")]
	TooLateToEnter,
	/// Unknown order
	#[serde(rename = "5")]
	UnknownOrder,
	/// Duplicate Order (e.g. dupe ClOrdID)
	#[serde(rename = "6")]
	DuplicateOrder,
	/// Unsupported order characteristic
	#[serde(rename = "11")]
	UnsupportedOrderCharacteristic,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for ListRejectReason {
	fn default() -> Self {
		ListRejectReason::BrokerExchangeOption
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
