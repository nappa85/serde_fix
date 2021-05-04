
use serde::{Serialize, Deserialize};

use fix_common::{ApplVerID, Boolean, EncodedText, RepeatingValues, UTCTimestamp, FixVersion};

/// Standard Message Header
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StandardMessageHeader<const V: u8, const T1: char, const T2: char> {
    /// FIXT.1.1 (Always unencrypted, must be first field in message)
    #[serde(rename = "8")]
    #[serde(default)]
    pub begin_string: FixVersion<5>,
    /// (Always unencrypted, must be second field in message)
    #[serde(rename = "9")]
    #[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub body_length: usize,
    /// (Always unencrypted, must be third field in message)
    #[serde(rename = "35")]
    #[serde(default)]
    pub msg_type: MsgType<T1, T2>,
    /// Indicates application version using a service pack identifier. The ApplVerID (1128) applies to a specific message occurrence.
    #[serde(rename = "1128")]
    #[serde(default)]
    pub appl_ver_id: ApplVerID<V>,
    /// Identifies the Extension Pack which is to be applied to the FIX version specified in the ApplVerID.
    #[serde(rename = "1156")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appl_ext_id: Option<String>,
    /// Used to support bilaterally agreed custom functionality
    #[serde(rename = "1129")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cstm_appl_ver_id: Option<String>,
    /// (Always unencrypted)
    #[serde(rename = "49")]
    pub sender_comp_id: String,
    /// (Always unencrypted)
    #[serde(rename = "56")]
    pub target_comp_id: String,
    /// Trading partner company ID used when sending messages via a third party (Can be embedded within encrypted data section.)
    #[serde(rename = "115")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_comp_id: Option<String>,
    /// Trading partner company ID used when sending messages via a third party (Can be embedded within encrypted data section.)
    #[serde(rename = "128")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_to_comp_id: Option<String>,
    /// Required to identify length of encrypted section of message. (Always unencrypted)
    #[serde(rename = "90")]
    /// Required when message body is encrypted. Always immediately follows SecureDataLen (90) field.
    #[serde(alias = "91")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_data: Option<EncodedText<91>>,
    /// (Can be embedded within encrypted data section.)
    #[serde(rename = "34")]
    #[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
    pub msg_seq_num: u32,
    /// (Can be embedded within encrypted data section.)
    #[serde(rename = "50")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_sub_id: Option<String>,
    /// Sender's LocationID (i.e. geographic location and/or desk) (Can be embedded within encrypted data section.)
    #[serde(rename = "142")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_location_id: Option<String>,
    /// "ADMIN" reserved for administrative messages not intended for a specific user. (Can be embedded within encrypted data section.)
    #[serde(rename = "57")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_dub_id: Option<String>,
    /// Trading partner LocationID (i.e. geographic location and/or desk) (Can be embedded within encrypted data section.)
    #[serde(rename = "143")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_location_id: Option<String>,
    /// Trading partner SubID used when delivering messages via a third party. (Can be embedded within encrypted data section.)
    #[serde(rename = "116")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_sub_id: Option<String>,
    /// Trading partner LocationID (i.e. geographic location and/or desk) used when delivering messages via a third party. (Can be embedded within encrypted data section.)
    #[serde(rename = "144")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_location_id: Option<String>,
    /// Trading partner SubID used when delivering messages via a third party. (Can be embedded within encrypted data section.)
    #[serde(rename = "129")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_to_sub_id: Option<String>,
    /// Trading partner LocationID (i.e. geographic location and/or desk) used when delivering messages via a third party. (Can be embedded within encrypted data section.)
    #[serde(rename = "145")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_to_location_id: Option<String>,
    /// Always required for retransmitted messages, whether prompted by the sending system or as the result of a resend request. (Can be embedded within encrypted data section.)
    #[serde(rename = "43")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poss_dup_flag: Option<Boolean>,
    /// Required when message may be duplicate of another message sent under a different sequence number. (Can be embedded within encrypted data section.)
    #[serde(rename = "97")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poss_resend: Option<Boolean>,
    /// (Can be embedded within encrypted data section.)
    #[serde(rename = "52")]
    pub sending_time: UTCTimestamp,
    /// Required for message resent as a result of a ResendRequest. If data is not available set to same value as SendingTime (52) (Can be embedded within encrypted data section.)
    #[serde(rename = "122")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_sending_time: Option<UTCTimestamp>,
    /// Required when specifying XmlData (213) to identify the length of a XmlData (213) message block. (Can be embedded within encrypted data section.)
    #[serde(rename = "212")]
    /// Can contain a XML formatted message block (e.g. FIXML). Always immediately follows XmlDataLen (212) field. (Can be embedded within encrypted data section.) See Volume 1: FIXML Support
    #[serde(alias = "213")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_data: Option<EncodedText<213>>,
    /// Type of message encoding (non-ASCII characters) used in a message's "Encoded" fields. Required if any "Encoding" fields are used.
    #[serde(rename = "347")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_encoding: Option<String>,
    /// The last MsgSeqNum (34) value received by the FIX engine and processed by downstream application, such as trading system or order routing system. Can be specified on every message sent. Useful for detecting a backlog with a counterparty.
    #[serde(rename = "369")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
    #[serde(default)]
    pub last_msg_seq_num_processed: Option<u64>,
    #[serde(rename = "627")]
    #[serde(alias = "628")]
    #[serde(alias = "629")]
    #[serde(alias = "630")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hops: Option<RepeatingValues<Hop>>,
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
    /// Allocation Instruction
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
    /// Allocation Instruction Ack
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
    /// Mass Quote Acknowledgement
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
    XMLMessage,
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
    /// Multileg Order Cancel/Replace
    #[serde(rename = "AC")]
    MultilegOrderCancelReplace,
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
    RFQRequest,
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
    /// Request For Positions
    #[serde(rename = "AN")]
    RequestForPositions,
    /// Request For Positions Ack
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
    /// Network Counterparty System Status Request
    #[serde(rename = "BC")]
    NetworkCounterpartySystemStatusRequest,
    /// Network Counterparty System Status Response
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
    /// Trading Session List Request
    #[serde(rename = "BI")]
    TradingSessionListRequest,
    /// Trading Session List
    #[serde(rename = "BJ")]
    TradingSessionList,
    /// Security List Update Report
    #[serde(rename = "BK")]
    SecurityListUpdateReport,
    /// Adjusted Position Report
    #[serde(rename = "BL")]
    AdjustedPositionReport,
    /// Allocation Instruction Alert
    #[serde(rename = "BM")]
    AllocationInstructionAlert,
    /// Execution Acknowledgement
    #[serde(rename = "BN")]
    ExecutioNAcknowledgement,
    /// Contrary Intention Report
    #[serde(rename = "BO")]
    ContraryIntentionReport,
    /// Security Definition Update Report
    #[serde(rename = "BP")]
    SecurityDefinitionUpdateReport,
    /// Settlement Obligation Report
    #[serde(rename = "BQ")]
    SettlementObligationReport,
    /// Derivative Security List Update Report
    #[serde(rename = "BR")]
    DerivativeSecurityListUpdateReport,
    /// Trading Session List Update Report
    #[serde(rename = "BS")]
    TradingSessionListUpdateReport,
    /// Market Definition Request
    #[serde(rename = "BT")]
    MarketDefinitionRequest,
    /// Market Definition
    #[serde(rename = "BU")]
    MarketDefinition,
    /// Market Definition Update Report
    #[serde(rename = "BV")]
    MarketDefinitionUpdateReport,
    /// Application Message Request
    #[serde(rename = "BW")]
    ApplicationMessageRequest,
    /// Application Message Request Ack
    #[serde(rename = "BX")]
    ApplicationMessageRequestAck,
    /// Application Message Report
    #[serde(rename = "BY")]
    ApplicationMessageReport,
    /// Order Mass Action Report
    #[serde(rename = "BZ")]
    OrderMassActionReport,
    /// Order Mass Action Request
    #[serde(rename = "CA")]
    OrderMassActionRequest,
    /// User Notification
    #[serde(rename = "CB")]
    UserNotification,
    /// Stream Assignment Request
    #[serde(rename = "CC")]
    StreamAssignmentRequest,
    /// Stream Assignment Report
    #[serde(rename = "CD")]
    StreamAssignmentReport,
    /// Stream Assignment Report ACK
    #[serde(rename = "CE")]
    StreamAssignmentReportACK,
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
            ('n', ' ') => MsgType::XMLMessage,
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
            ('A', 'C') => MsgType::MultilegOrderCancelReplace,
            ('A', 'D') => MsgType::TradeCaptureReportRequest,
            ('A', 'E') => MsgType::TradeCaptureReport,
            ('A', 'F') => MsgType::OrderMassStatusRequest,
            ('A', 'G') => MsgType::QuoteRequestReject,
            ('A', 'H') => MsgType::RFQRequest,
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
            ('B', 'I') => MsgType::TradingSessionListRequest,
            ('B', 'J') => MsgType::TradingSessionList,
            ('B', 'K') => MsgType::SecurityListUpdateReport,
            ('B', 'L') => MsgType::AdjustedPositionReport,
            ('B', 'M') => MsgType::AllocationInstructionAlert,
            ('B', 'N') => MsgType::ExecutioNAcknowledgement,
            ('B', 'O') => MsgType::ContraryIntentionReport,
            ('B', 'P') => MsgType::SecurityDefinitionUpdateReport,
            ('B', 'Q') => MsgType::SettlementObligationReport,
            ('B', 'R') => MsgType::DerivativeSecurityListUpdateReport,
            ('B', 'S') => MsgType::TradingSessionListUpdateReport,
            ('B', 'T') => MsgType::MarketDefinitionRequest,
            ('B', 'U') => MsgType::MarketDefinition,
            ('B', 'V') => MsgType::MarketDefinitionUpdateReport,
            ('B', 'W') => MsgType::ApplicationMessageRequest,
            ('B', 'X') => MsgType::ApplicationMessageRequestAck,
            ('B', 'Y') => MsgType::ApplicationMessageReport,
            ('B', 'Z') => MsgType::OrderMassActionReport,
            ('C', 'A') => MsgType::OrderMassActionRequest,
            ('C', 'B') => MsgType::UserNotification,
            ('C', 'C') => MsgType::StreamAssignmentRequest,
            ('C', 'D') => MsgType::StreamAssignmentReport,
            ('C', 'E') => MsgType::StreamAssignmentReportACK,
            _ => unimplemented!(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Hop {
    #[serde(rename = "628")]
    pub hop_comp_id: Option<String>,
    #[serde(rename = "629")]
    pub hop_sending_time: Option<UTCTimestamp>,
    #[serde(rename = "630")]
    pub hop_ref_id: Option<u64>,
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    #[test]
    fn deserialize_header() {
        let msg = "8=FIXT.1.1\u{1}9=78\u{1}35=A\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}212=10\u{1}213=0123456789\u{1}627=2\u{1}628=A\u{1}629=20210310-16:38:01.821\u{1}630=1\u{1}628=B\u{1}629=20210310-16:38:01.821\u{1}630=2\u{1}";
        dbg!(serde_fix::from_str::<super::StandardMessageHeader::<9, 'A', ' '>>(msg).unwrap());
    }

    #[test]
    fn serialize_header() {
        let obj = super::StandardMessageHeader::<9, 'A', ' '> {
            begin_string: fix_common::version::FixVersion::FIXT11,
            body_length: 78,
            msg_type: super::MsgType::Logon,
            appl_ver_id: fix_common::version::ApplVerID::FIX50SP2,
            appl_ext_id: None,
            cstm_appl_ver_id: None,
            sender_comp_id: "CLIENT1".to_owned(),
            target_comp_id: "EXECUTOR".to_owned(),
            on_behalf_of_comp_id: None,
            deliver_to_comp_id: None,
            secure_data: None,
            msg_seq_num: 17,
            sender_sub_id: None,
            sender_location_id: None,
            target_dub_id: None,
            target_location_id: None,
            on_behalf_of_sub_id: None,
            on_behalf_of_location_id: None,
            deliver_to_sub_id: None,
            deliver_to_location_id: None,
            poss_dup_flag: None,
            poss_resend: None,
            sending_time: fix_common::datetime::UTCTimestamp::try_from("20210310-16:38:01.821").unwrap(),
            orig_sending_time: None,
            xml_data: Some(fix_common::EncodedText("0123456789".to_owned())),
            message_encoding: None,
            last_msg_seq_num_processed: None,
            hops: Some(fix_common::RepeatingValues(vec![
                super::Hop {
                    hop_comp_id: Some(
                        "A".to_owned(),
                    ),
                    hop_sending_time: Some(
                        fix_common::datetime::UTCTimestamp::try_from("20210310-16:38:01.821").unwrap(),
                    ),
                    hop_ref_id: Some(
                        1,
                    ),
                },
                super::Hop {
                    hop_comp_id: Some(
                        "B".to_owned(),
                    ),
                    hop_sending_time: Some(
                        fix_common::datetime::UTCTimestamp::try_from("20210310-16:38:01.821").unwrap(),
                    ),
                    hop_ref_id: Some(
                        2,
                    ),
                },
            ])),
        };
        let msg = "8=FIXT.1.1\u{1}9=78\u{1}35=A\u{1}1128=9\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}212=10\u{1}213=0123456789\u{1}627=2\u{1}628=A\u{1}629=20210310-16:38:01.821\u{1}630=1\u{1}628=B\u{1}629=20210310-16:38:01.821\u{1}630=2\u{1}";
        assert_eq!(serde_fix::to_string(obj), Ok(msg.to_owned()));
    }
}
