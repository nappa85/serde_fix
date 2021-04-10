
use serde::{Deserialize, Serialize};

pub mod heartbeat;
pub use heartbeat::Heartbeat;
pub mod logon;
pub use logon::Logon;
pub mod logout;
pub use logout::Logout;
pub mod reject;
pub use reject::Reject;
pub mod resend_request;
pub use resend_request::ResendRequest;
pub mod sequence_reset;
pub use sequence_reset::SequenceReset;
pub mod test_request;
pub use test_request::TestRequest;
/*
use super::header::{Header, HasHeader};

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "35")]
pub enum Message {
    // FIX50SP2(crate::entities::fix50sp2::messages::Message),
    /// Heartbeat
    #[serde(rename = "0")]
    Heartbeat(Box<Heartbeat>),
    /// Test Request
    #[serde(rename = "1")]
    TestRequest(Box<TestRequest>),
    /// Resend Request
    #[serde(rename = "2")]
    ResendRequest(Box<ResendRequest>),
    /// Reject
    #[serde(rename = "3")]
    Reject(Box<Reject>),
    /// Sequence Reset
    #[serde(rename = "4")]
    SequenceReset(Box<SequenceReset>),
    /// Logout
    #[serde(rename = "5")]
    Logout(Box<Logout>),
    // /// Indication of Interest
    // #[serde(rename = "6")]
    // IndicationOfInterest,
    // /// Advertisement
    // #[serde(rename = "7")]
    // Advertisement,
    /// Execution Report
    #[serde(rename = "8")]
    ExecutionReport(ExecutionReport),
    // /// Order Cancel Reject
    // #[serde(rename = "9")]
    // OrderCancelReject,
    /// Logon
    #[serde(rename = "A")]
    Logon(Box<Logon>),
    // /// News
    // #[serde(rename = "B")]
    // News,
    // /// Email
    // #[serde(rename = "C")]
    // Email,
    /// New Order - Single
    #[serde(rename = "D")]
    NewOrderSingle(NewOrderSingle),
    // /// New Order - List
    // #[serde(rename = "E")]
    // NewOrderList,
    // /// Order Cancel Request
    // #[serde(rename = "F")]
    // OrderCancelRequest,
    // /// Order Cancel/Replace Request
    // #[serde(rename = "G")]
    // OrderCancelReplaceRequest,
    // /// Order Status Request
    // #[serde(rename = "H")]
    // OrderStatusRequest,
    // /// Allocation Instruction
    // #[serde(rename = "J")]
    // AllocationInstruction,
    // /// List Cancel Request
    // #[serde(rename = "K")]
    // ListCancelRequest,
    // /// List Execute
    // #[serde(rename = "L")]
    // ListExecute,
    // /// List Status Request
    // #[serde(rename = "M")]
    // ListStatusRequest,
    // /// List Status
    // #[serde(rename = "N")]
    // ListStatus,
    // /// Allocation Instruction Ack
    // #[serde(rename = "P")]
    // AllocationInstructionAck,
    // /// Don't Know Trade
    // #[serde(rename = "Q")]
    // DontKnowTrade,
    // /// Quote Request
    // #[serde(rename = "R")]
    // QuoteRequest,
    // /// Quote
    // #[serde(rename = "S")]
    // Quote,
    // /// Settlement Instructions
    // #[serde(rename = "T")]
    // SettlementInstructions,
    // /// Market Data Request
    // #[serde(rename = "V")]
    // MarketDataRequest,
    // /// Market Data - Snapshot/Full Refresh
    // #[serde(rename = "W")]
    // MarketDataSnapshotFullRefresh,
    // /// Market Data - Incremental Refresh
    // #[serde(rename = "X")]
    // MarketDataIncrementalRefresh,
    // /// Market Data Request Reject
    // #[serde(rename = "Y")]
    // MarketDataRequestReject,
    // /// Quote Cancel
    // #[serde(rename = "Z")]
    // QuoteCancel,
    // /// Quote Status Request
    // #[serde(rename = "a")]
    // QuoteStatusRequest,
    // /// Mass Quote Acknowledgement
    // #[serde(rename = "b")]
    // MassQuoteAcknowledgement,
    // /// Security Definition Request
    // #[serde(rename = "c")]
    // SecurityDefinitionRequest,
    // /// Security Definition
    // #[serde(rename = "d")]
    // SecurityDefinition,
    // /// Security Status Request
    // #[serde(rename = "e")]
    // SecurityStatusRequest,
    // /// Security Status
    // #[serde(rename = "f")]
    // SecurityStatus,
    // /// Trading Session Status Request
    // #[serde(rename = "g")]
    // TradingSessionStatusRequest,
    // /// Trading Session Status
    // #[serde(rename = "h")]
    // TradingSessionStatus,
    // /// Mass Quote
    // #[serde(rename = "i")]
    // MassQuote,
    // /// Business Message Reject
    // #[serde(rename = "j")]
    // BusinessMessageReject,
    // /// Bid Request
    // #[serde(rename = "k")]
    // BidRequest,
    // /// Bid Response
    // #[serde(rename = "l")]
    // BidResponse,
    // /// List Strike Price
    // #[serde(rename = "m")]
    // ListStrikePrice,
    // /// XML message
    // #[serde(rename = "n")]
    // XMLMessage,
    // /// Registration Instructions
    // #[serde(rename = "o")]
    // RegistrationInstructions,
    // /// Registration Instructions Response
    // #[serde(rename = "p")]
    // RegistrationInstructionsResponse,
    // /// Order Mass Cancel Request
    // #[serde(rename = "q")]
    // OrderMassCancelRequest,
    // /// Order Mass Cancel Report
    // #[serde(rename = "r")]
    // OrderMassCancelReport,
    // /// New Order - Cross
    // #[serde(rename = "s")]
    // NewOrderCross,
    // /// Cross Order Cancel/Replace Request
    // #[serde(rename = "t")]
    // CrossOrderCancelReplaceRequest,
    // /// Cross Order Cancel Request
    // #[serde(rename = "u")]
    // CrossOrderCancelRequest,
    // /// Security Type Request
    // #[serde(rename = "v")]
    // SecurityTypeRequest,
    // /// Security Types
    // #[serde(rename = "w")]
    // SecurityTypes,
    // /// Security List Request
    // #[serde(rename = "x")]
    // SecurityListRequest,
    // /// Security List
    // #[serde(rename = "y")]
    // SecurityList,
    // /// Derivative Security List Request
    // #[serde(rename = "z")]
    // DerivativeSecurityListRequest,
    // /// Derivative Security List
    // #[serde(rename = "AA")]
    // DerivativeSecurityList,
    // /// New Order - Multileg
    // #[serde(rename = "AB")]
    // NewOrderMultileg,
    // /// Multileg Order Cancel/Replace
    // #[serde(rename = "AC")]
    // MultilegOrderCancelReplace,
    // /// Trade Capture Report Request
    // #[serde(rename = "AD")]
    // TradeCaptureReportRequest,
    // /// Trade Capture Report
    // #[serde(rename = "AE")]
    // TradeCaptureReport,
    // /// Order Mass Status Request
    // #[serde(rename = "AF")]
    // OrderMassStatusRequest,
    // /// Quote Request Reject
    // #[serde(rename = "AG")]
    // QuoteRequestReject,
    // /// RFQ Request
    // #[serde(rename = "AH")]
    // RFQRequest,
    // /// Quote Status Report
    // #[serde(rename = "AI")]
    // QuoteStatusReport,
    // /// Quote Response
    // #[serde(rename = "AJ")]
    // QuoteResponse,
    // /// Confirmation
    // #[serde(rename = "AK")]
    // Confirmation,
    // /// Position Maintenance Request
    // #[serde(rename = "AL")]
    // PositionMaintenanceRequest,
    // /// Position Maintenance Report
    // #[serde(rename = "AM")]
    // PositionMaintenanceReport,
    // /// Request For Positions
    // #[serde(rename = "AN")]
    // RequestForPositions,
    // /// Request For Positions Ack
    // #[serde(rename = "AO")]
    // RequestForPositionsAck,
    // /// Position Report
    // #[serde(rename = "AP")]
    // PositionReport,
    // /// Trade Capture Report Request Ack
    // #[serde(rename = "AQ")]
    // TradeCaptureReportRequestAck,
    // /// Trade Capture Report Ack
    // #[serde(rename = "AR")]
    // TradeCaptureReportAck,
    // /// Allocation Report
    // #[serde(rename = "AS")]
    // AllocationReport,
    // /// Allocation Report Ack
    // #[serde(rename = "AT")]
    // AllocationReportAck,
    // /// Confirmation Ack
    // #[serde(rename = "AU")]
    // ConfirmationAck,
    // /// Settlement Instruction Request
    // #[serde(rename = "AV")]
    // SettlementInstructionRequest,
    // /// Assignment Report
    // #[serde(rename = "AW")]
    // AssignmentReport,
    // /// Collateral Request
    // #[serde(rename = "AX")]
    // CollateralRequest,
    // /// Collateral Assignment
    // #[serde(rename = "AY")]
    // CollateralAssignment,
    // /// Collateral Response
    // #[serde(rename = "AZ")]
    // CollateralResponse,
    // /// Collateral Report
    // #[serde(rename = "BA")]
    // CollateralReport,
    // /// Collateral Inquiry
    // #[serde(rename = "BB")]
    // CollateralInquiry,
    // /// Network Counterparty System Status Request
    // #[serde(rename = "BC")]
    // NetworkCounterpartySystemStatusRequest,
    // /// Network Counterparty System Status Response
    // #[serde(rename = "BD")]
    // NetworkCounterpartySystemStatusResponse,
    // /// User Request
    // #[serde(rename = "BE")]
    // UserRequest,
    // /// User Response
    // #[serde(rename = "BF")]
    // UserResponse,
    // /// Collateral Inquiry Ack
    // #[serde(rename = "BG")]
    // CollateralInquiryAck,
    // /// Confirmation Request
    // #[serde(rename = "BH")]
    // ConfirmationRequest,
    // /// Trading Session List Request
    // #[serde(rename = "BI")]
    // TradingSessionListRequest,
    // /// Trading Session List
    // #[serde(rename = "BJ")]
    // TradingSessionList,
    // /// Security List Update Report
    // #[serde(rename = "BK")]
    // SecurityListUpdateReport,
    // /// Adjusted Position Report
    // #[serde(rename = "BL")]
    // AdjustedPositionReport,
    // /// Allocation Instruction Alert
    // #[serde(rename = "BM")]
    // AllocationInstructionAlert,
    // /// Execution Acknowledgement
    // #[serde(rename = "BN")]
    // ExecutioNAcknowledgement,
    // /// Contrary Intention Report
    // #[serde(rename = "BO")]
    // ContraryIntentionReport,
    // /// Security Definition Update Report
    // #[serde(rename = "BP")]
    // SecurityDefinitionUpdateReport,
    // /// Settlement Obligation Report
    // #[serde(rename = "BQ")]
    // SettlementObligationReport,
    // /// Derivative Security List Update Report
    // #[serde(rename = "BR")]
    // DerivativeSecurityListUpdateReport,
    // /// Trading Session List Update Report
    // #[serde(rename = "BS")]
    // TradingSessionListUpdateReport,
    // /// Market Definition Request
    // #[serde(rename = "BT")]
    // MarketDefinitionRequest,
    // /// Market Definition
    // #[serde(rename = "BU")]
    // MarketDefinition,
    // /// Market Definition Update Report
    // #[serde(rename = "BV")]
    // MarketDefinitionUpdateReport,
    // /// Application Message Request
    // #[serde(rename = "BW")]
    // ApplicationMessageRequest,
    // /// Application Message Request Ack
    // #[serde(rename = "BX")]
    // ApplicationMessageRequestAck,
    // /// Application Message Report
    // #[serde(rename = "BY")]
    // ApplicationMessageReport,
    // /// Order Mass Action Report
    // #[serde(rename = "BZ")]
    // OrderMassActionReport,
    // /// Order Mass Action Request
    // #[serde(rename = "CA")]
    // OrderMassActionRequest,
    // /// User Notification
    // #[serde(rename = "CB")]
    // UserNotification,
    // /// Stream Assignment Request
    // #[serde(rename = "CC")]
    // StreamAssignmentRequest,
    // /// Stream Assignment Report
    // #[serde(rename = "CD")]
    // StreamAssignmentReport,
    // /// Stream Assignment Report ACK
    // #[serde(rename = "CE")]
    // StreamAssignmentReportACK,
}

impl Serialize for Message {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Message::Heartbeat(m) => m.serialize(serializer),
            Message::Logon(m) => m.serialize(serializer),
            Message::Logout(m) => m.serialize(serializer),
            Message::Reject(m) => m.serialize(serializer),
            Message::ResendRequest(m) => m.serialize(serializer),
            Message::SequenceReset(m) => m.serialize(serializer),
            Message::TestRequest(m) => m.serialize(serializer),
            Message::NewOrderSingle(m) => m.serialize(serializer),
            Message::ExecutionReport(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for Message {
    fn get_header(&self) -> &Header {
        match self {
            Message::Heartbeat(m) => m.get_header(),
            Message::Logon(m) => m.get_header(),
            Message::Logout(m) => m.get_header(),
            Message::Reject(m) => m.get_header(),
            Message::ResendRequest(m) => m.get_header(),
            Message::SequenceReset(m) => m.get_header(),
            Message::TestRequest(m) => m.get_header(),
            Message::NewOrderSingle(m) => m.get_header(),
            Message::ExecutionReport(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            Message::Heartbeat(m) => m.get_header_mut(),
            Message::Logon(m) => m.get_header_mut(),
            Message::Logout(m) => m.get_header_mut(),
            Message::Reject(m) => m.get_header_mut(),
            Message::ResendRequest(m) => m.get_header_mut(),
            Message::SequenceReset(m) => m.get_header_mut(),
            Message::TestRequest(m) => m.get_header_mut(),
            Message::NewOrderSingle(m) => m.get_header_mut(),
            Message::ExecutionReport(m) => m.get_header_mut(),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NewOrderSingle {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<crate::entities::fix50::messages::new_order_single::NewOrderSingle>),
    // /// FIX50SP1
    // #[serde(rename = "8")]
    // FIX50SP1(Box<crate::entities::fix50sp1::messages::NewOrderSingle>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<crate::entities::fix50sp2::messages::new_order_single::NewOrderSingle>),
}

impl Serialize for NewOrderSingle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NewOrderSingle::FIX50(m) => m.serialize(serializer),
            // NewOrderSingle::FIX50SP1(m) => m.serialize(serializer),
            NewOrderSingle::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for NewOrderSingle {
    fn get_header(&self) -> &Header {
        match self {
            NewOrderSingle::FIX50(m) => m.get_header(),
            // NewOrderSingle::FIX50SP1(m) => m.get_header(),
            NewOrderSingle::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            NewOrderSingle::FIX50(m) => m.get_header_mut(),
            // NewOrderSingle::FIX50SP1(m) => m.get_header_mut(),
            NewOrderSingle::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ExecutionReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<crate::entities::fix50::messages::execution_report::ExecutionReport>),
    // /// FIX50SP1
    // #[serde(rename = "8")]
    // FIX50SP1(Box<crate::entities::fix50sp1::messages::ExecutionReport>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<crate::entities::fix50sp2::messages::execution_report::ExecutionReport>),
}

impl Serialize for ExecutionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ExecutionReport::FIX50(m) => m.serialize(serializer),
            // ExecutionReport::FIX50SP1(m) => m.serialize(serializer),
            ExecutionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ExecutionReport {
    fn get_header(&self) -> &Header {
        match self {
            ExecutionReport::FIX50(m) => m.get_header(),
            // ExecutionReport::FIX50SP1(m) => m.get_header(),
            ExecutionReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ExecutionReport::FIX50(m) => m.get_header_mut(),
            // ExecutionReport::FIX50SP1(m) => m.get_header_mut(),
            ExecutionReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Message;
    use crate::header::{MsgType, HasHeader};

    #[test]
    fn logon() {
        let msg = "8=FIXT.1.1\u{1}9=111\u{1}35=A\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}212=10\u{1}213=0123456789\u{1}369=1\u{1}98=0\u{1}108=1\u{1}789=1\u{1}1137=0\u{1}10=073\u{1}";
        let mut obj = dbg!(serde_fix::from_str_checked::<Message>(msg)).unwrap();
        obj.get_header_mut().msg_type = Some(MsgType::Logon);
        obj.get_header_mut().body_length = 0;
        assert_eq!(serde_fix::to_string_checked(&obj), Ok(msg.to_owned()));
    }
}
*/
