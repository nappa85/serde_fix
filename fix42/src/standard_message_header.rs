
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StandardMessageHeader<const T: char> {
	/// FIX.4.2 (Always unencrypted, must be first field in message)
	#[serde(rename = "8")]
	#[serde(default)]
	pub begin_string: fix_common::FixVersion<2>,
	/// (Always unencrypted, must be second field in message)
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "9")]
	pub body_length: i32,
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
	/// Trading partner company ID used when sending messages via a third party (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "115")]
	pub on_behalf_of_comp_id: Option<String>,
	/// Trading partner company ID used when sending messages via a third party (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "128")]
	pub deliver_to_comp_id: Option<String>,
	/// Required to identify length of encrypted section of message. (Always unencrypted)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "90")]
	pub secure_data_len: Option<i32>,
	/// Required when message body is encrypted. Always immediately follows <a href="tag_90_SecureDataLen.html" target="bottom">SecureDataLen&nbsp;(90)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "91")]
	pub secure_data: Option<String>,
	/// (Can be embedded within encrypted data section.)
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "34")]
	pub msg_seq_num: i32,
	/// (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "50")]
	pub sender_sub_id: Option<String>,
	/// Sender's LocationID (i.e. geographic location and/or desk) (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "142")]
	pub sender_location_id: Option<String>,
	/// "ADMIN" reserved for administrative messages not intended for a specific user. (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "57")]
	pub target_sub_id: Option<String>,
	/// Trading partner LocationID (i.e. geographic location and/or desk) (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "143")]
	pub target_location_id: Option<String>,
	/// Trading partner SubID used when delivering messages via a third party. (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "116")]
	pub on_behalf_of_sub_id: Option<String>,
	/// Trading partner LocationID (i.e. geographic location and/or desk) used when delivering messages via a third party. (Can be
	/// embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "144")]
	pub on_behalf_of_location_id: Option<String>,
	/// Trading partner SubID used when delivering messages via a third party. (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "129")]
	pub deliver_to_sub_id: Option<String>,
	/// Trading partner LocationID (i.e. geographic location and/or desk) used when delivering messages via a third party. (Can be
	/// embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "145")]
	pub deliver_to_location_id: Option<String>,
	/// Always required for retransmitted messages, whether prompted by the sending system or as the result of a resend request. (Can
	/// be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43")]
	pub poss_dup_flag: Option<PossDupFlag>,
	/// Required when message may be duplicate of another message sent under a different sequence number. (Can be embedded within
	/// encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "97")]
	pub poss_resend: Option<fix_common::Boolean>,
	/// (Can be embedded within encrypted data section.)
	#[serde(rename = "52")]
	pub sending_time: fix_common::UTCTimestamp,
	/// Required for message resent as a result of a Resend Request . If data is not available set to same value as <a href="tag_52_SendingTime.html" target="bottom">SendingTime&nbsp;(52)</a> (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "122")]
	pub orig_sending_time: Option<fix_common::UTCTimestamp>,
	/// Required when specifying <a href="tag_213_XmlData.html" target="bottom">XmlData&nbsp;(213)</a> to identify the length of a XmlData message block. (Can be embedded within encrypted data section.)
	#[serde(rename = "212")]
	/// Can contain a XML formatted message block (e.g. FIXML). Always immediately follows XmlDataLen field. (Can be embedded within
	/// encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "213")]
	pub xml_data: Option<fix_common::EncodedText<213>>,
	/// Type of message encoding (non-ASCII characters) used in a message's "Encoded" fields. Required if any "Encoding" fields are
	/// used.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "347")]
	pub message_encoding: Option<MessageEncoding>,
	/// The last <a href="tag_34_MsgSeqNum.html" target="bottom">MsgSeqNum&nbsp;(34)</a> value received and processed. Can be specified on every message sent. Useful for detecting a backlog with a counterparty.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "369")]
	pub last_msg_seq_num_processed: Option<i32>,
	/// Used when a message is sent via a "hub" or "service bureau". If A sends to Q (the hub) who then sends to B via a separate
	/// FIX session, then when Q sends to B the value of this field should represent the <a href="tag_52_SendingTime.html" target="bottom">SendingTime&nbsp;(52)</a> on the message A sent to Q. (always expressed in UTC (Universal Time Coordinated, also known as "GMT")
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "370")]
	pub on_behalf_of_sending_time: Option<fix_common::UTCTimestamp>,
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
	/// Settlement Instructions
	#[serde(rename = "T")]
	SettlementInstructions,
	/// Market Data Request
	#[serde(rename = "V")]
	MarketDataRequest,
	/// Market Data - Snapshot/Full Refresh
	#[serde(rename = "W")]
	MarketDataSnapshotFullRefresh,
	/// Market Data - Incremental Refresh
	#[serde(rename = "X")]
	MarketDataIncrementalRefresh,
	/// Market Data Request Reject
	#[serde(rename = "Y")]
	MarketDataRequestReject,
	/// Quote Cancel
	#[serde(rename = "Z")]
	QuoteCancel,
	/// Quote Status Request
	#[serde(rename = "a")]
	QuoteStatusRequest,
	/// Quote Acknowledgement
	#[serde(rename = "b")]
	QuoteAcknowledgement,
	/// Security Definition Request
	#[serde(rename = "c")]
	SecurityDefinitionRequest,
	/// Security Definition
	#[serde(rename = "d")]
	SecurityDefinition,
	/// Security Status Request
	#[serde(rename = "e")]
	SecurityStatusRequest,
	/// Security Status
	#[serde(rename = "f")]
	SecurityStatus,
	/// Trading Session Status Request
	#[serde(rename = "g")]
	TradingSessionStatusRequest,
	/// Trading Session Status
	#[serde(rename = "h")]
	TradingSessionStatus,
	/// Mass Quote
	#[serde(rename = "i")]
	MassQuote,
	/// Business Message Reject
	#[serde(rename = "j")]
	BusinessMessageReject,
	/// Bid Request
	#[serde(rename = "k")]
	BidRequest,
	/// Bid Response
	#[serde(rename = "l")]
	BidResponse,
	/// List Strike Price
	#[serde(rename = "m")]
	ListStrikePrice,
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
            'T' => MsgType::SettlementInstructions,
            'V' => MsgType::MarketDataRequest,
            'W' => MsgType::MarketDataSnapshotFullRefresh,
            'X' => MsgType::MarketDataIncrementalRefresh,
            'Y' => MsgType::MarketDataRequestReject,
            'Z' => MsgType::QuoteCancel,
            'a' => MsgType::QuoteStatusRequest,
            'b' => MsgType::QuoteAcknowledgement,
            'c' => MsgType::SecurityDefinitionRequest,
            'd' => MsgType::SecurityDefinition,
            'e' => MsgType::SecurityStatusRequest,
            'f' => MsgType::SecurityStatus,
            'g' => MsgType::TradingSessionStatusRequest,
            'h' => MsgType::TradingSessionStatus,
            'i' => MsgType::MassQuote,
            'j' => MsgType::BusinessMessageReject,
            'k' => MsgType::BidRequest,
            'l' => MsgType::BidResponse,
            'm' => MsgType::ListStrikePrice,
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MessageEncoding {
	/// JIS
	#[serde(rename = "ISO-2022-JP")]
	Jis,
	/// EUC
	#[serde(rename = "EUC-JP")]
	Euc,
	/// SJIS
	#[serde(rename = "Shift_JIS")]
	Sjis,
	/// Unicode
	#[serde(rename = "UTF-8")]
	Unicode,
}

impl Default for MessageEncoding {
	fn default() -> Self {
		MessageEncoding::Jis
	}
}
