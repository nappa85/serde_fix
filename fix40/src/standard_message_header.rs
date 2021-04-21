
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StandardMessageHeader<const T: char> {
	/// FIX.4.0 (Always unencrypted, must be first field in message)
	#[serde(rename = "8")]
	#[serde(default)]
	pub begin_string: fix_common::FixVersion<0>,
	/// (Always unencrypted, must be second field in message)
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "9")]
	pub body_length: u32,
	/// (Always unencrypted, must be third field in message)
	#[serde(rename = "35")]
	#[serde(default)]
	pub msg_type: MsgType<T>,
	/// (Always unencrypted)
	#[serde(rename = "49")]
	pub sender_comp_id: String,
	/// (Always unencrypted)
	#[serde(rename = "56")]
	pub target_comp_id: String,
	/// Trading partner company ID used when sending messages via a third party (Can be
	/// embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "115")]
	pub on_behalf_of_comp_id: Option<String>,
	/// Trading partner company ID used when sending messages via a third party (Can be
	/// embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "128")]
	pub deliver_to_comp_id: Option<String>,
	/// Required to identify length of encrypted section of message. (Always unencrypted)
	#[serde(rename = "90")]
	/// Required when message body is encrypted. Always immediately follows <a href="tag_90_SecureDataLen.html" target="bottom">SecureDataLen&nbsp;(90)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "91")]
	pub secure_data: Option<fix_common::EncodedText<91>>,
	/// (Can be embedded within encrypted data section.)
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "34")]
	pub msg_seq_num: u32,
	/// (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "50")]
	pub sender_sub_id: Option<String>,
	/// 'ADMIN' reserved for administrative messages not intended for a specific user. (Can
	/// be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "57")]
	pub target_sub_id: Option<String>,
	/// Trading partner SubID used when delivering messages via a third party. (Can be
	/// embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "116")]
	pub on_behalf_of_sub_id: Option<String>,
	/// Trading partner SubID used when delivering messages via a third party. (Can be
	/// embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "129")]
	pub deliver_to_sub_id: Option<String>,
	/// Always required for retransmissions, whether prompted by the sending system or as
	/// the result of a resend request. (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43")]
	pub poss_dup_flag: Option<PossDupFlag>,
	/// Required when message may be duplicate of another message sent under a different
	/// sequence number. (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "97")]
	pub poss_resend: Option<String>,
	/// (Can be embedded within encrypted data section.)
	#[serde(rename = "52")]
	pub sending_time: fix_common::UTCTimeOnly,
	/// Required for message resends. If data is not available set to same value
	/// as <a href="tag_52_SendingTime.html" target="bottom">SendingTime&nbsp;(52)</a> (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "122")]
	pub orig_sending_time: Option<fix_common::UTCTimeOnly>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MsgType<const T: char> {
	/// Heartbeat
	#[serde(rename = "0")]
	Heartbeat,
	/// Test Request
	#[serde(rename = "1")]
	TestRequest,
	/// Resend Request
	#[serde(rename = "2")]
	ResendRequest,
	/// Reject
	#[serde(rename = "3")]
	Reject,
	/// Sequence Reset
	#[serde(rename = "4")]
	SequenceReset,
	/// Logout
	#[serde(rename = "5")]
	Logout,
	/// Indication of Interest
	#[serde(rename = "6")]
	IndicationOfInterest,
	/// Advertisement
	#[serde(rename = "7")]
	Advertisement,
	/// Execution Report
	#[serde(rename = "8")]
	ExecutionReport,
	/// Order Cancel Reject
	#[serde(rename = "9")]
	OrderCancelReject,
	/// Logon
	#[serde(rename = "A")]
	Logon,
	/// News
	#[serde(rename = "B")]
	News,
	/// Email
	#[serde(rename = "C")]
	Email,
	/// New Order - Single
	#[serde(rename = "D")]
	NewOrderSingle,
	/// New Order - List
	#[serde(rename = "E")]
	NewOrderList,
	/// Order Cancel Request
	#[serde(rename = "F")]
	OrderCancelRequest,
	/// Order Cancel/Replace Request
	#[serde(rename = "G")]
	OrderCancelReplaceRequest,
	/// Order Status Request
	#[serde(rename = "H")]
	OrderStatusRequest,
	/// Allocation
	#[serde(rename = "J")]
	Allocation,
	/// List Cancel Request
	#[serde(rename = "K")]
	ListCancelRequest,
	/// List Execute
	#[serde(rename = "L")]
	ListExecute,
	/// List Status Request
	#[serde(rename = "M")]
	ListStatusRequest,
	/// List Status
	#[serde(rename = "N")]
	ListStatus,
	/// Allocation ACK
	#[serde(rename = "P")]
	AllocationAck,
	/// Don't Know Trade
	#[serde(rename = "Q")]
	DonTKnowTrade,
	/// Quote Request
	#[serde(rename = "R")]
	QuoteRequest,
	/// Quote
	#[serde(rename = "S")]
	Quote,
}

impl<const T: char> Default for MsgType<T> {
	fn default() -> Self {
        match T {
            '0' => MsgType::Heartbeat,
            '1' => MsgType::TestRequest,
            '2' => MsgType::ResendRequest,
            '3' => MsgType::Reject,
            '4' => MsgType::SequenceReset,
            '5' => MsgType::Logout,
            '6' => MsgType::IndicationOfInterest,
            '7' => MsgType::Advertisement,
            '8' => MsgType::ExecutionReport,
            '9' => MsgType::OrderCancelReject,
            'A' => MsgType::Logon,
            'B' => MsgType::News,
            'C' => MsgType::Email,
            'D' => MsgType::NewOrderSingle,
            'E' => MsgType::NewOrderList,
            'F' => MsgType::OrderCancelRequest,
            'G' => MsgType::OrderCancelReplaceRequest,
            'H' => MsgType::OrderStatusRequest,
            'J' => MsgType::Allocation,
            'K' => MsgType::ListCancelRequest,
            'L' => MsgType::ListExecute,
            'M' => MsgType::ListStatusRequest,
            'N' => MsgType::ListStatus,
            'P' => MsgType::AllocationAck,
            'Q' => MsgType::DonTKnowTrade,
            'R' => MsgType::QuoteRequest,
            'S' => MsgType::Quote,
            _ => unimplemented!(),
        }
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PossDupFlag {
	/// Possible duplicate
	#[serde(rename = "Y")]
	PossibleDuplicate,
	/// Original transmission
	#[serde(rename = "N")]
	OriginalTransmission,
}

impl Default for PossDupFlag {
	fn default() -> Self {
		PossDupFlag::PossibleDuplicate
	}
}
