
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StandardMessageHeader<const T1: char, const T2: char> {
	/// FIX.4.4 (Always unencrypted, must be first field in message)
	#[serde(rename = "8")]
	#[serde(default)]
	pub begin_string: fix_common::FixVersion<4>,
	/// (Always unencrypted, must be second field in message)
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "9")]
	pub body_length: usize,
	/// (Always unencrypted, must be third field in message)
	#[serde(rename = "35")]
	#[serde(default)]
	pub msg_type: MsgType<T1, T2>,
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
	#[serde(rename = "90")]
	/// Required when message body is encrypted. Always immediately follows <a href="tag_90_SecureDataLen.html" target="bottom">SecureDataLen&nbsp;(90)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "91")]
	pub secure_data: Option<fix_common::EncodedText<91>>,
	/// (Can be embedded within encrypted data section.)
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "34")]
	pub msg_seq_num: usize,
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
	pub poss_resend: Option<PossResend>,
	/// (Can be embedded within encrypted data section.)
	#[serde(rename = "52")]
	pub sending_time: fix_common::UTCTimestamp,
	/// Required for message resent as a result of a <a href="message_Resend_Request_2.html" target="main">Resend Request&nbsp;(2)</a> . If data is not available set to same value as <a href="tag_52_SendingTime.html" target="bottom">SendingTime&nbsp;(52)</a> (Can be embedded within encrypted data section.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "122")]
	pub orig_sending_time: Option<fix_common::UTCTimestamp>,
	/// Required when specifying <a href="tag_213_XmlData.html" target="bottom">XmlData&nbsp;(213)</a> to identify the length of a XmlData message block. (Can be embedded within encrypted data section.)
	#[serde(rename = "212")]
	/// Can contain a XML formatted message block (e.g. FIXML). Always immediately follows <a href="tag_212_XmlDataLen.html" target="bottom">XmlDataLen&nbsp;(212)</a> field. (Can be embedded within encrypted data section.) <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="https://www.fixtrading.org/standards/" target="_blank">See Volume 1: FIXML Support of FIX Specification</a>
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
	pub last_msg_seq_num_processed: Option<usize>,
	/// Number of repeating groups of historical "hop" information. Only applicable if <a href="tag_115_OnBehalfOfCompID.html" target="bottom">OnBehalfOfCompID&nbsp;(115)</a> is used, however, its use is optional. Note that some market regulations or counterparties may require tracking of message
	/// hops.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "627")]
	pub hops: Option<fix_common::RepeatingValues<Hop>>,
}

impl<const T1: char, const T2: char> StandardMessageHeader<T1, T2> {
    pub fn reply<const _T1: char, const _T2: char>(&mut self, headers: &StandardMessageHeader<_T1, _T2>) {
        self.sender_comp_id = headers.target_comp_id.clone();
        self.target_comp_id = headers.sender_comp_id.clone();
        self.msg_seq_num = headers.msg_seq_num;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Hop {
	/// Third party firm which delivered a specific message either from the firm which originated the message or from another third
	/// party (if multiple "hops" are performed). It is recommended that this value be the <a href="tag_49_SenderCompID.html" target="bottom">SenderCompID&nbsp;(49)</a> of the third party.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "628")]
	pub hop_comp_id: Option<String>,
	/// Time that <a href="tag_628_HopCompID.html" target="bottom">HopCompID&nbsp;(628)</a> sent the message. It is recommended that this value be the <a href="tag_52_SendingTime.html" target="bottom">SendingTime&nbsp;(52)</a> of the message sent by the third party.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "629")]
	pub hop_sending_time: Option<fix_common::UTCTimestamp>,
	/// Reference identifier assigned by <a href="tag_628_HopCompID.html" target="bottom">HopCompID&nbsp;(628)</a> associated with the message sent. It is recommended that this value be the <a href="tag_34_MsgSeqNum.html" target="bottom">MsgSeqNum&nbsp;(34)</a> of the message sent by the third party.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "630")]
	pub hop_ref_id: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MsgType<const T1: char, const T2: char> {
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
	AllocationInstruction,
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
	AllocationInstructionAck,
	/// Don't Know Trade
	#[serde(rename = "Q")]
	DontKnowTrade,
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
	MassQuoteAcknowledgement,
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
	/// XML message
	#[serde(rename = "n")]
	XmlMessage,
	/// Registration Instructions
	#[serde(rename = "o")]
	RegistrationInstructions,
	/// Registration Instructions Response
	#[serde(rename = "p")]
	RegistrationInstructionsResponse,
	/// Order Mass Cancel Request
	#[serde(rename = "q")]
	OrderMassCancelRequest,
	/// Order Mass Cancel Report
	#[serde(rename = "r")]
	OrderMassCancelReport,
	/// New Order - Cross
	#[serde(rename = "s")]
	NewOrderCross,
	/// Cross Order Cancel/Replace Request
	#[serde(rename = "t")]
	CrossOrderCancelReplaceRequest,
	/// Cross Order Cancel Request
	#[serde(rename = "u")]
	CrossOrderCancelRequest,
	/// Security Type Request
	#[serde(rename = "v")]
	SecurityTypeRequest,
	/// Security Types
	#[serde(rename = "w")]
	SecurityTypes,
	/// Security List Request
	#[serde(rename = "x")]
	SecurityListRequest,
	/// Security List
	#[serde(rename = "y")]
	SecurityList,
	/// Derivative Security List Request
	#[serde(rename = "z")]
	DerivativeSecurityListRequest,
	/// Derivative Security List
	#[serde(rename = "AA")]
	DerivativeSecurityList,
	/// New Order - Multileg
	#[serde(rename = "AB")]
	NewOrderMultileg,
	/// Multileg Order Cancel/Replace Request
	#[serde(rename = "AC")]
	MultilegOrderCancelReplaceRequest,
	/// Trade Capture Report Request
	#[serde(rename = "AD")]
	TradeCaptureReportRequest,
	/// Trade Capture Report
	#[serde(rename = "AE")]
	TradeCaptureReport,
	/// Order Mass Status Request
	#[serde(rename = "AF")]
	OrderMassStatusRequest,
	/// Quote Request Reject
	#[serde(rename = "AG")]
	QuoteRequestReject,
	/// RFQ Request
	#[serde(rename = "AH")]
	RfqRequest,
	/// Quote Status Report
	#[serde(rename = "AI")]
	QuoteStatusReport,
	/// Quote Response
	#[serde(rename = "AJ")]
	QuoteResponse,
	/// Confirmation
	#[serde(rename = "AK")]
	Confirmation,
	/// Position Maintenance Request
	#[serde(rename = "AL")]
	PositionMaintenanceRequest,
	/// Position Maintenance Report
	#[serde(rename = "AM")]
	PositionMaintenanceReport,
	/// Request for Positions
	#[serde(rename = "AN")]
	RequestForPositions,
	/// Request for Positions Ack
	#[serde(rename = "AO")]
	RequestForPositionsAck,
	/// Position Report
	#[serde(rename = "AP")]
	PositionReport,
	/// Trade Capture Report Request Ack
	#[serde(rename = "AQ")]
	TradeCaptureReportRequestAck,
	/// Trade Capture Report Ack
	#[serde(rename = "AR")]
	TradeCaptureReportAck,
	/// Allocation Report
	#[serde(rename = "AS")]
	AllocationReport,
	/// Allocation Report Ack
	#[serde(rename = "AT")]
	AllocationReportAck,
	/// Confirmation Ack
	#[serde(rename = "AU")]
	ConfirmationAck,
	/// Settlement Instruction Request
	#[serde(rename = "AV")]
	SettlementInstructionRequest,
	/// Assignment Report
	#[serde(rename = "AW")]
	AssignmentReport,
	/// Collateral Request
	#[serde(rename = "AX")]
	CollateralRequest,
	/// Collateral Assignment
	#[serde(rename = "AY")]
	CollateralAssignment,
	/// Collateral Response
	#[serde(rename = "AZ")]
	CollateralResponse,
	/// Collateral Report
	#[serde(rename = "BA")]
	CollateralReport,
	/// Collateral Inquiry
	#[serde(rename = "BB")]
	CollateralInquiry,
	/// Network Status Request
	#[serde(rename = "BC")]
	NetworkCounterpartySystemStatusRequest,
	/// Network Status Response
	#[serde(rename = "BD")]
	NetworkCounterpartySystemStatusResponse,
	/// User Request
	#[serde(rename = "BE")]
	UserRequest,
	/// User Response
	#[serde(rename = "BF")]
	UserResponse,
	/// Collateral Inquiry Ack
	#[serde(rename = "BG")]
	CollateralInquiryAck,
	/// Confirmation Request
	#[serde(rename = "BH")]
	ConfirmationRequest,
}

impl<const T1: char, const T2: char> Default for MsgType<T1, T2> {
	fn default() -> Self {
        match (T1, T2) {
            ('0', ' ') => MsgType::Heartbeat,
            ('1', ' ') => MsgType::TestRequest,
            ('2', ' ') => MsgType::ResendRequest,
            ('3', ' ') => MsgType::Reject,
            ('4', ' ') => MsgType::SequenceReset,
            ('5', ' ') => MsgType::Logout,
            ('6', ' ') => MsgType::IndicationOfInterest,
            ('7', ' ') => MsgType::Advertisement,
            ('8', ' ') => MsgType::ExecutionReport,
            ('9', ' ') => MsgType::OrderCancelReject,
            ('A', ' ') => MsgType::Logon,
            ('B', ' ') => MsgType::News,
            ('C', ' ') => MsgType::Email,
            ('D', ' ') => MsgType::NewOrderSingle,
            ('E', ' ') => MsgType::NewOrderList,
            ('F', ' ') => MsgType::OrderCancelRequest,
            ('G', ' ') => MsgType::OrderCancelReplaceRequest,
            ('H', ' ') => MsgType::OrderStatusRequest,
            ('J', ' ') => MsgType::AllocationInstruction,
            ('K', ' ') => MsgType::ListCancelRequest,
            ('L', ' ') => MsgType::ListExecute,
            ('M', ' ') => MsgType::ListStatusRequest,
            ('N', ' ') => MsgType::ListStatus,
            ('P', ' ') => MsgType::AllocationInstructionAck,
            ('Q', ' ') => MsgType::DontKnowTrade,
            ('R', ' ') => MsgType::QuoteRequest,
            ('S', ' ') => MsgType::Quote,
            ('T', ' ') => MsgType::SettlementInstructions,
            ('V', ' ') => MsgType::MarketDataRequest,
            ('W', ' ') => MsgType::MarketDataSnapshotFullRefresh,
            ('X', ' ') => MsgType::MarketDataIncrementalRefresh,
            ('Y', ' ') => MsgType::MarketDataRequestReject,
            ('Z', ' ') => MsgType::QuoteCancel,
            ('a', ' ') => MsgType::QuoteStatusRequest,
            ('b', ' ') => MsgType::MassQuoteAcknowledgement,
            ('c', ' ') => MsgType::SecurityDefinitionRequest,
            ('d', ' ') => MsgType::SecurityDefinition,
            ('e', ' ') => MsgType::SecurityStatusRequest,
            ('f', ' ') => MsgType::SecurityStatus,
            ('g', ' ') => MsgType::TradingSessionStatusRequest,
            ('h', ' ') => MsgType::TradingSessionStatus,
            ('i', ' ') => MsgType::MassQuote,
            ('j', ' ') => MsgType::BusinessMessageReject,
            ('k', ' ') => MsgType::BidRequest,
            ('l', ' ') => MsgType::BidResponse,
            ('m', ' ') => MsgType::ListStrikePrice,
            ('n', ' ') => MsgType::XmlMessage,
            ('o', ' ') => MsgType::RegistrationInstructions,
            ('p', ' ') => MsgType::RegistrationInstructionsResponse,
            ('q', ' ') => MsgType::OrderMassCancelRequest,
            ('r', ' ') => MsgType::OrderMassCancelReport,
            ('s', ' ') => MsgType::NewOrderCross,
            ('t', ' ') => MsgType::CrossOrderCancelReplaceRequest,
            ('u', ' ') => MsgType::CrossOrderCancelRequest,
            ('v', ' ') => MsgType::SecurityTypeRequest,
            ('w', ' ') => MsgType::SecurityTypes,
            ('x', ' ') => MsgType::SecurityListRequest,
            ('y', ' ') => MsgType::SecurityList,
            ('z', ' ') => MsgType::DerivativeSecurityListRequest,
            ('A', 'A') => MsgType::DerivativeSecurityList,
            ('A', 'B') => MsgType::NewOrderMultileg,
            ('A', 'C') => MsgType::MultilegOrderCancelReplaceRequest,
            ('A', 'D') => MsgType::TradeCaptureReportRequest,
            ('A', 'E') => MsgType::TradeCaptureReport,
            ('A', 'F') => MsgType::OrderMassStatusRequest,
            ('A', 'G') => MsgType::QuoteRequestReject,
            ('A', 'H') => MsgType::RfqRequest,
            ('A', 'I') => MsgType::QuoteStatusReport,
            ('A', 'J') => MsgType::QuoteResponse,
            ('A', 'K') => MsgType::Confirmation,
            ('A', 'L') => MsgType::PositionMaintenanceRequest,
            ('A', 'M') => MsgType::PositionMaintenanceReport,
            ('A', 'N') => MsgType::RequestForPositions,
            ('A', 'O') => MsgType::RequestForPositionsAck,
            ('A', 'P') => MsgType::PositionReport,
            ('A', 'Q') => MsgType::TradeCaptureReportRequestAck,
            ('A', 'R') => MsgType::TradeCaptureReportAck,
            ('A', 'S') => MsgType::AllocationReport,
            ('A', 'T') => MsgType::AllocationReportAck,
            ('A', 'U') => MsgType::ConfirmationAck,
            ('A', 'V') => MsgType::SettlementInstructionRequest,
            ('A', 'W') => MsgType::AssignmentReport,
            ('A', 'X') => MsgType::CollateralRequest,
            ('A', 'Y') => MsgType::CollateralAssignment,
            ('A', 'Z') => MsgType::CollateralResponse,
            ('B', 'A') => MsgType::CollateralReport,
            ('B', 'B') => MsgType::CollateralInquiry,
            ('B', 'C') => MsgType::NetworkCounterpartySystemStatusRequest,
            ('B', 'D') => MsgType::NetworkCounterpartySystemStatusResponse,
            ('B', 'E') => MsgType::UserRequest,
            ('B', 'F') => MsgType::UserResponse,
            ('B', 'G') => MsgType::CollateralInquiryAck,
            ('B', 'H') => MsgType::ConfirmationRequest,
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
pub enum PossResend {
	/// Possible resend
	#[serde(rename = "Y")]
	PossibleResend,
	/// Original transmission
	#[serde(rename = "N")]
	OriginalTransmission,
}

impl Default for PossResend {
	fn default() -> Self {
		PossResend::PossibleResend
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
