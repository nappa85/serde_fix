
use serde::{Deserialize, Serialize};

use fix_common::ApplVerID;

use crate::has_header::*;

/// FIXT11
pub use fixt11::*;
#[cfg(feature = "fix_50")]
/// FIX50
pub use fix50;
#[cfg(feature = "fix_50sp1")]
/// FIX50SP1
pub use fix50sp1;
#[cfg(feature = "fix_50sp2")]
/// FIX50SP2
pub use fix50sp2;

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "35")]
pub enum Message {
    /// Heartbeat
    #[serde(rename = "0")]
    Heartbeat(Box<fixt11::messages::Heartbeat>),
    /// Test Request
    #[serde(rename = "1")]
    TestRequest(Box<fixt11::messages::TestRequest>),
    /// Resend Request
    #[serde(rename = "2")]
    ResendRequest(Box<fixt11::messages::ResendRequest>),
    /// Reject
    #[serde(rename = "3")]
    Reject(Box<fixt11::messages::Reject>),
    /// Sequence Reset
    #[serde(rename = "4")]
    SequenceReset(Box<fixt11::messages::SequenceReset>),
    /// Logout
    #[serde(rename = "5")]
    Logout(Box<fixt11::messages::Logout>),
    /// Indication of Interest
    #[serde(rename = "6")]
    IndicationOfInterest(IndicationOfInterest),
    /// Advertisement
    #[serde(rename = "7")]
    Advertisement(Advertisement),
    /// Execution Report
    #[serde(rename = "8")]
    ExecutionReport(ExecutionReport),
    /// Order Cancel Reject
    #[serde(rename = "9")]
    OrderCancelReject(OrderCancelReject),
    /// Logon
    #[serde(rename = "A")]
    Logon(Box<fixt11::messages::Logon>),
    /// News
    #[serde(rename = "B")]
    News(News),
    /// Email
    #[serde(rename = "C")]
    Email(Email),
    /// New Order - Single
    #[serde(rename = "D")]
    NewOrderSingle(NewOrderSingle),
    /// New Order - List
    #[serde(rename = "E")]
    NewOrderList(NewOrderList),
    /// Order Cancel Request
    #[serde(rename = "F")]
    OrderCancelRequest(OrderCancelRequest),
    /// Order Cancel/Replace Request
    #[serde(rename = "G")]
    OrderCancelReplaceRequest(OrderCancelReplaceRequest),
    /// Order Status Request
    #[serde(rename = "H")]
    OrderStatusRequest(OrderStatusRequest),
    /// Allocation Instruction
    #[serde(rename = "J")]
    AllocationInstruction(AllocationInstruction),
    /// List Cancel Request
    #[serde(rename = "K")]
    ListCancelRequest(ListCancelRequest),
    /// List Execute
    #[serde(rename = "L")]
    ListExecute(ListExecute),
    /// List Status Request
    #[serde(rename = "M")]
    ListStatusRequest(ListStatusRequest),
    /// List Status
    #[serde(rename = "N")]
    ListStatus(ListStatus),
    /// Allocation Instruction Ack
    #[serde(rename = "P")]
    AllocationInstructionAck(AllocationInstructionAck),
    /// Don't Know Trade
    #[serde(rename = "Q")]
    DontKnowTrade(DontKnowTrade),
    /// Quote Request
    #[serde(rename = "R")]
    QuoteRequest(QuoteRequest),
    /// Quote
    #[serde(rename = "S")]
    Quote(Quote),
    /// Settlement Instructions
    #[serde(rename = "T")]
    SettlementInstructions(SettlementInstructions),
    /// Market Data Request
    #[serde(rename = "V")]
    MarketDataRequest(MarketDataRequest),
    /// Market Data - Snapshot/Full Refresh
    #[serde(rename = "W")]
    MarketDataSnapshotFullRefresh(MarketDataSnapshotFullRefresh),
    /// Market Data - Incremental Refresh
    #[serde(rename = "X")]
    MarketDataIncrementalRefresh(MarketDataIncrementalRefresh),
    /// Market Data Request Reject
    #[serde(rename = "Y")]
    MarketDataRequestReject(MarketDataRequestReject),
    /// Quote Cancel
    #[serde(rename = "Z")]
    QuoteCancel(QuoteCancel),
    /// Quote Status Request
    #[serde(rename = "a")]
    QuoteStatusRequest(QuoteStatusRequest),
    /// Mass Quote Acknowledgement
    #[serde(rename = "b")]
    MassQuoteAcknowledgement(MassQuoteAcknowledgement),
    /// Security Definition Request
    #[serde(rename = "c")]
    SecurityDefinitionRequest(SecurityDefinitionRequest),
    /// Security Definition
    #[serde(rename = "d")]
    SecurityDefinition(SecurityDefinition),
    /// Security Status Request
    #[serde(rename = "e")]
    SecurityStatusRequest(SecurityStatusRequest),
    /// Security Status
    #[serde(rename = "f")]
    SecurityStatus(SecurityStatus),
    /// Trading Session Status Request
    #[serde(rename = "g")]
    TradingSessionStatusRequest(TradingSessionStatusRequest),
    /// Trading Session Status
    #[serde(rename = "h")]
    TradingSessionStatus(TradingSessionStatus),
    /// Mass Quote
    #[serde(rename = "i")]
    MassQuote(MassQuote),
    /// Business Message Reject
    #[serde(rename = "j")]
    BusinessMessageReject(BusinessMessageReject),
    /// Bid Request
    #[serde(rename = "k")]
    BidRequest(BidRequest),
    /// Bid Response
    #[serde(rename = "l")]
    BidResponse(BidResponse),
    /// List Strike Price
    #[serde(rename = "m")]
    ListStrikePrice(ListStrikePrice),
    /// XML message
    #[serde(rename = "n")]
    XmlMessage(XmlMessage),
    /// Registration Instructions
    #[serde(rename = "o")]
    RegistrationInstructions(RegistrationInstructions),
    /// Registration Instructions Response
    #[serde(rename = "p")]
    RegistrationInstructionsResponse(RegistrationInstructionsResponse),
    /// Order Mass Cancel Request
    #[serde(rename = "q")]
    OrderMassCancelRequest(OrderMassCancelRequest),
    /// Order Mass Cancel Report
    #[serde(rename = "r")]
    OrderMassCancelReport(OrderMassCancelReport),
    /// New Order - Cross
    #[serde(rename = "s")]
    NewOrderCross(NewOrderCross),
    /// Cross Order Cancel/Replace Request
    #[serde(rename = "t")]
    CrossOrderCancelReplaceRequest(CrossOrderCancelReplaceRequest),
    /// Cross Order Cancel Request
    #[serde(rename = "u")]
    CrossOrderCancelRequest(CrossOrderCancelRequest),
    /// Security Type Request
    #[serde(rename = "v")]
    SecurityTypeRequest(SecurityTypeRequest),
    /// Security Types
    #[serde(rename = "w")]
    SecurityTypes(SecurityTypes),
    /// Security List Request
    #[serde(rename = "x")]
    SecurityListRequest(SecurityListRequest),
    /// Security List
    #[serde(rename = "y")]
    SecurityList(SecurityList),
    /// Derivative Security List Request
    #[serde(rename = "z")]
    DerivativeSecurityListRequest(DerivativeSecurityListRequest),
    /// Derivative Security List
    #[serde(rename = "AA")]
    DerivativeSecurityList(DerivativeSecurityList),
    /// New Order - Multileg
    #[serde(rename = "AB")]
    NewOrderMultileg(NewOrderMultileg),
    /// Multileg Order Cancel/Replace
    #[serde(rename = "AC")]
    MultilegOrderCancelReplace(MultilegOrderCancelReplace),
    /// Trade Capture Report Request
    #[serde(rename = "AD")]
    TradeCaptureReportRequest(TradeCaptureReportRequest),
    /// Trade Capture Report
    #[serde(rename = "AE")]
    TradeCaptureReport(TradeCaptureReport),
    /// Order Mass Status Request
    #[serde(rename = "AF")]
    OrderMassStatusRequest(OrderMassStatusRequest),
    /// Quote Request Reject
    #[serde(rename = "AG")]
    QuoteRequestReject(QuoteRequestReject),
    /// RFQ Request
    #[serde(rename = "AH")]
    RfqRequest(RfqRequest),
    /// Quote Status Report
    #[serde(rename = "AI")]
    QuoteStatusReport(QuoteStatusReport),
    /// Quote Response
    #[serde(rename = "AJ")]
    QuoteResponse(QuoteResponse),
    /// Confirmation
    #[serde(rename = "AK")]
    Confirmation(Confirmation),
    /// Position Maintenance Request
    #[serde(rename = "AL")]
    PositionMaintenanceRequest(PositionMaintenanceRequest),
    /// Position Maintenance Report
    #[serde(rename = "AM")]
    PositionMaintenanceReport(PositionMaintenanceReport),
    /// Request For Positions
    #[serde(rename = "AN")]
    RequestForPositions(RequestForPositions),
    /// Request For Positions Ack
    #[serde(rename = "AO")]
    RequestForPositionsAck(RequestForPositionsAck),
    /// Position Report
    #[serde(rename = "AP")]
    PositionReport(PositionReport),
    /// Trade Capture Report Request Ack
    #[serde(rename = "AQ")]
    TradeCaptureReportRequestAck(TradeCaptureReportRequestAck),
    /// Trade Capture Report Ack
    #[serde(rename = "AR")]
    TradeCaptureReportAck(TradeCaptureReportAck),
    /// Allocation Report
    #[serde(rename = "AS")]
    AllocationReport(AllocationReport),
    /// Allocation Report Ack
    #[serde(rename = "AT")]
    AllocationReportAck(AllocationReportAck),
    /// Confirmation Ack
    #[serde(rename = "AU")]
    ConfirmationAck(ConfirmationAck),
    /// Settlement Instruction Request
    #[serde(rename = "AV")]
    SettlementInstructionRequest(SettlementInstructionRequest),
    /// Assignment Report
    #[serde(rename = "AW")]
    AssignmentReport(AssignmentReport),
    /// Collateral Request
    #[serde(rename = "AX")]
    CollateralRequest(CollateralRequest),
    /// Collateral Assignment
    #[serde(rename = "AY")]
    CollateralAssignment(CollateralAssignment),
    /// Collateral Response
    #[serde(rename = "AZ")]
    CollateralResponse(CollateralResponse),
    /// Collateral Report
    #[serde(rename = "BA")]
    CollateralReport(CollateralReport),
    /// Collateral Inquiry
    #[serde(rename = "BB")]
    CollateralInquiry(CollateralInquiry),
    /// Network Counterparty System Status Request
    #[serde(rename = "BC")]
    NetworkCounterpartySystemStatusRequest(NetworkCounterpartySystemStatusRequest),
    /// Network Counterparty System Status Response
    #[serde(rename = "BD")]
    NetworkCounterpartySystemStatusResponse(NetworkCounterpartySystemStatusResponse),
    /// User Request
    #[serde(rename = "BE")]
    UserRequest(UserRequest),
    /// User Response
    #[serde(rename = "BF")]
    UserResponse(UserResponse),
    /// Collateral Inquiry Ack
    #[serde(rename = "BG")]
    CollateralInquiryAck(CollateralInquiryAck),
    /// Confirmation Request
    #[serde(rename = "BH")]
    ConfirmationRequest(ConfirmationRequest),
    /// Trading Session List Request
    #[serde(rename = "BI")]
    TradingSessionListRequest(TradingSessionListRequest),
    /// Trading Session List
    #[serde(rename = "BJ")]
    TradingSessionList(TradingSessionList),
    /// Security List Update Report
    #[serde(rename = "BK")]
    SecurityListUpdateReport(SecurityListUpdateReport),
    /// Adjusted Position Report
    #[serde(rename = "BL")]
    AdjustedPositionReport(AdjustedPositionReport),
    /// Allocation Instruction Alert
    #[serde(rename = "BM")]
    AllocationInstructionAlert(AllocationInstructionAlert),
    /// Execution Acknowledgement
    #[serde(rename = "BN")]
    ExecutionAcknowledgement(ExecutionAcknowledgement),
    /// Contrary Intention Report
    #[serde(rename = "BO")]
    ContraryIntentionReport(ContraryIntentionReport),
    /// Security Definition Update Report
    #[serde(rename = "BP")]
    SecurityDefinitionUpdateReport(SecurityDefinitionUpdateReport),
    /// Settlement Obligation Report
    #[serde(rename = "BQ")]
    SettlementObligationReport(SettlementObligationReport),
    /// Derivative Security List Update Report
    #[serde(rename = "BR")]
    DerivativeSecurityListUpdateReport(DerivativeSecurityListUpdateReport),
    /// Trading Session List Update Report
    #[serde(rename = "BS")]
    TradingSessionListUpdateReport(TradingSessionListUpdateReport),
    /// Market Definition Request
    #[serde(rename = "BT")]
    MarketDefinitionRequest(MarketDefinitionRequest),
    /// Market Definition
    #[serde(rename = "BU")]
    MarketDefinition(MarketDefinition),
    /// Market Definition Update Report
    #[serde(rename = "BV")]
    MarketDefinitionUpdateReport(MarketDefinitionUpdateReport),
    /// Application Message Request
    #[serde(rename = "BW")]
    ApplicationMessageRequest(ApplicationMessageRequest),
    /// Application Message Request Ack
    #[serde(rename = "BX")]
    ApplicationMessageRequestAck(ApplicationMessageRequestAck),
    /// Application Message Report
    #[serde(rename = "BY")]
    ApplicationMessageReport(ApplicationMessageReport),
    /// Order Mass Action Report
    #[serde(rename = "BZ")]
    OrderMassActionReport(OrderMassActionReport),
    /// Order Mass Action Request
    #[serde(rename = "CA")]
    OrderMassActionRequest(OrderMassActionRequest),
    /// User Notification
    #[serde(rename = "CB")]
    UserNotification(UserNotification),
    /// Stream Assignment Request
    #[serde(rename = "CC")]
    StreamAssignmentRequest(StreamAssignmentRequest),
    /// Stream Assignment Report
    #[serde(rename = "CD")]
    StreamAssignmentReport(StreamAssignmentReport),
    /// Stream Assignment Report Ack
    #[serde(rename = "CE")]
    StreamAssignmentReportAck(StreamAssignmentReportAck),
}

impl Serialize for Message {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Message::Heartbeat(m) => m.serialize(serializer),
            Message::TestRequest(m) => m.serialize(serializer),
            Message::ResendRequest(m) => m.serialize(serializer),
            Message::Reject(m) => m.serialize(serializer),
            Message::SequenceReset(m) => m.serialize(serializer),
            Message::Logout(m) => m.serialize(serializer),
            Message::IndicationOfInterest(m) => m.serialize(serializer),
            Message::Advertisement(m) => m.serialize(serializer),
            Message::ExecutionReport(m) => m.serialize(serializer),
            Message::OrderCancelReject(m) => m.serialize(serializer),
            Message::Logon(m) => m.serialize(serializer),
            Message::News(m) => m.serialize(serializer),
            Message::Email(m) => m.serialize(serializer),
            Message::NewOrderSingle(m) => m.serialize(serializer),
            Message::NewOrderList(m) => m.serialize(serializer),
            Message::OrderCancelRequest(m) => m.serialize(serializer),
            Message::OrderCancelReplaceRequest(m) => m.serialize(serializer),
            Message::OrderStatusRequest(m) => m.serialize(serializer),
            Message::AllocationInstruction(m) => m.serialize(serializer),
            Message::ListCancelRequest(m) => m.serialize(serializer),
            Message::ListExecute(m) => m.serialize(serializer),
            Message::ListStatusRequest(m) => m.serialize(serializer),
            Message::ListStatus(m) => m.serialize(serializer),
            Message::AllocationInstructionAck(m) => m.serialize(serializer),
            Message::DontKnowTrade(m) => m.serialize(serializer),
            Message::QuoteRequest(m) => m.serialize(serializer),
            Message::Quote(m) => m.serialize(serializer),
            Message::SettlementInstructions(m) => m.serialize(serializer),
            Message::MarketDataRequest(m) => m.serialize(serializer),
            Message::MarketDataSnapshotFullRefresh(m) => m.serialize(serializer),
            Message::MarketDataIncrementalRefresh(m) => m.serialize(serializer),
            Message::MarketDataRequestReject(m) => m.serialize(serializer),
            Message::QuoteCancel(m) => m.serialize(serializer),
            Message::QuoteStatusRequest(m) => m.serialize(serializer),
            Message::MassQuoteAcknowledgement(m) => m.serialize(serializer),
            Message::SecurityDefinitionRequest(m) => m.serialize(serializer),
            Message::SecurityDefinition(m) => m.serialize(serializer),
            Message::SecurityStatusRequest(m) => m.serialize(serializer),
            Message::SecurityStatus(m) => m.serialize(serializer),
            Message::TradingSessionStatusRequest(m) => m.serialize(serializer),
            Message::TradingSessionStatus(m) => m.serialize(serializer),
            Message::MassQuote(m) => m.serialize(serializer),
            Message::BusinessMessageReject(m) => m.serialize(serializer),
            Message::BidRequest(m) => m.serialize(serializer),
            Message::BidResponse(m) => m.serialize(serializer),
            Message::ListStrikePrice(m) => m.serialize(serializer),
            Message::XmlMessage(m) => m.serialize(serializer),
            Message::RegistrationInstructions(m) => m.serialize(serializer),
            Message::RegistrationInstructionsResponse(m) => m.serialize(serializer),
            Message::OrderMassCancelRequest(m) => m.serialize(serializer),
            Message::OrderMassCancelReport(m) => m.serialize(serializer),
            Message::NewOrderCross(m) => m.serialize(serializer),
            Message::CrossOrderCancelReplaceRequest(m) => m.serialize(serializer),
            Message::CrossOrderCancelRequest(m) => m.serialize(serializer),
            Message::SecurityTypeRequest(m) => m.serialize(serializer),
            Message::SecurityTypes(m) => m.serialize(serializer),
            Message::SecurityListRequest(m) => m.serialize(serializer),
            Message::SecurityList(m) => m.serialize(serializer),
            Message::DerivativeSecurityListRequest(m) => m.serialize(serializer),
            Message::DerivativeSecurityList(m) => m.serialize(serializer),
            Message::NewOrderMultileg(m) => m.serialize(serializer),
            Message::MultilegOrderCancelReplace(m) => m.serialize(serializer),
            Message::TradeCaptureReportRequest(m) => m.serialize(serializer),
            Message::TradeCaptureReport(m) => m.serialize(serializer),
            Message::OrderMassStatusRequest(m) => m.serialize(serializer),
            Message::QuoteRequestReject(m) => m.serialize(serializer),
            Message::RfqRequest(m) => m.serialize(serializer),
            Message::QuoteStatusReport(m) => m.serialize(serializer),
            Message::QuoteResponse(m) => m.serialize(serializer),
            Message::Confirmation(m) => m.serialize(serializer),
            Message::PositionMaintenanceRequest(m) => m.serialize(serializer),
            Message::PositionMaintenanceReport(m) => m.serialize(serializer),
            Message::RequestForPositions(m) => m.serialize(serializer),
            Message::RequestForPositionsAck(m) => m.serialize(serializer),
            Message::PositionReport(m) => m.serialize(serializer),
            Message::TradeCaptureReportRequestAck(m) => m.serialize(serializer),
            Message::TradeCaptureReportAck(m) => m.serialize(serializer),
            Message::AllocationReport(m) => m.serialize(serializer),
            Message::AllocationReportAck(m) => m.serialize(serializer),
            Message::ConfirmationAck(m) => m.serialize(serializer),
            Message::SettlementInstructionRequest(m) => m.serialize(serializer),
            Message::AssignmentReport(m) => m.serialize(serializer),
            Message::CollateralRequest(m) => m.serialize(serializer),
            Message::CollateralAssignment(m) => m.serialize(serializer),
            Message::CollateralResponse(m) => m.serialize(serializer),
            Message::CollateralReport(m) => m.serialize(serializer),
            Message::CollateralInquiry(m) => m.serialize(serializer),
            Message::NetworkCounterpartySystemStatusRequest(m) => m.serialize(serializer),
            Message::NetworkCounterpartySystemStatusResponse(m) => m.serialize(serializer),
            Message::UserRequest(m) => m.serialize(serializer),
            Message::UserResponse(m) => m.serialize(serializer),
            Message::CollateralInquiryAck(m) => m.serialize(serializer),
            Message::ConfirmationRequest(m) => m.serialize(serializer),
            Message::TradingSessionListRequest(m) => m.serialize(serializer),
            Message::TradingSessionList(m) => m.serialize(serializer),
            Message::SecurityListUpdateReport(m) => m.serialize(serializer),
            Message::AdjustedPositionReport(m) => m.serialize(serializer),
            Message::AllocationInstructionAlert(m) => m.serialize(serializer),
            Message::ExecutionAcknowledgement(m) => m.serialize(serializer),
            Message::ContraryIntentionReport(m) => m.serialize(serializer),
            Message::SecurityDefinitionUpdateReport(m) => m.serialize(serializer),
            Message::SettlementObligationReport(m) => m.serialize(serializer),
            Message::DerivativeSecurityListUpdateReport(m) => m.serialize(serializer),
            Message::TradingSessionListUpdateReport(m) => m.serialize(serializer),
            Message::MarketDefinitionRequest(m) => m.serialize(serializer),
            Message::MarketDefinition(m) => m.serialize(serializer),
            Message::MarketDefinitionUpdateReport(m) => m.serialize(serializer),
            Message::ApplicationMessageRequest(m) => m.serialize(serializer),
            Message::ApplicationMessageRequestAck(m) => m.serialize(serializer),
            Message::ApplicationMessageReport(m) => m.serialize(serializer),
            Message::OrderMassActionReport(m) => m.serialize(serializer),
            Message::OrderMassActionRequest(m) => m.serialize(serializer),
            Message::UserNotification(m) => m.serialize(serializer),
            Message::StreamAssignmentRequest(m) => m.serialize(serializer),
            Message::StreamAssignmentReport(m) => m.serialize(serializer),
            Message::StreamAssignmentReportAck(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for Message {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            Message::Heartbeat(m) => Box::new(m.get_header()),
            Message::TestRequest(m) => Box::new(m.get_header()),
            Message::ResendRequest(m) => Box::new(m.get_header()),
            Message::Reject(m) => Box::new(m.get_header()),
            Message::SequenceReset(m) => Box::new(m.get_header()),
            Message::Logout(m) => Box::new(m.get_header()),
            Message::IndicationOfInterest(m) => m.get_header_boxed(),
            Message::Advertisement(m) => m.get_header_boxed(),
            Message::ExecutionReport(m) => m.get_header_boxed(),
            Message::OrderCancelReject(m) => m.get_header_boxed(),
            Message::Logon(m) => Box::new(m.get_header()),
            Message::News(m) => m.get_header_boxed(),
            Message::Email(m) => m.get_header_boxed(),
            Message::NewOrderSingle(m) => m.get_header_boxed(),
            Message::NewOrderList(m) => m.get_header_boxed(),
            Message::OrderCancelRequest(m) => m.get_header_boxed(),
            Message::OrderCancelReplaceRequest(m) => m.get_header_boxed(),
            Message::OrderStatusRequest(m) => m.get_header_boxed(),
            Message::AllocationInstruction(m) => m.get_header_boxed(),
            Message::ListCancelRequest(m) => m.get_header_boxed(),
            Message::ListExecute(m) => m.get_header_boxed(),
            Message::ListStatusRequest(m) => m.get_header_boxed(),
            Message::ListStatus(m) => m.get_header_boxed(),
            Message::AllocationInstructionAck(m) => m.get_header_boxed(),
            Message::DontKnowTrade(m) => m.get_header_boxed(),
            Message::QuoteRequest(m) => m.get_header_boxed(),
            Message::Quote(m) => m.get_header_boxed(),
            Message::SettlementInstructions(m) => m.get_header_boxed(),
            Message::MarketDataRequest(m) => m.get_header_boxed(),
            Message::MarketDataSnapshotFullRefresh(m) => m.get_header_boxed(),
            Message::MarketDataIncrementalRefresh(m) => m.get_header_boxed(),
            Message::MarketDataRequestReject(m) => m.get_header_boxed(),
            Message::QuoteCancel(m) => m.get_header_boxed(),
            Message::QuoteStatusRequest(m) => m.get_header_boxed(),
            Message::MassQuoteAcknowledgement(m) => m.get_header_boxed(),
            Message::SecurityDefinitionRequest(m) => m.get_header_boxed(),
            Message::SecurityDefinition(m) => m.get_header_boxed(),
            Message::SecurityStatusRequest(m) => m.get_header_boxed(),
            Message::SecurityStatus(m) => m.get_header_boxed(),
            Message::TradingSessionStatusRequest(m) => m.get_header_boxed(),
            Message::TradingSessionStatus(m) => m.get_header_boxed(),
            Message::MassQuote(m) => m.get_header_boxed(),
            Message::BusinessMessageReject(m) => m.get_header_boxed(),
            Message::BidRequest(m) => m.get_header_boxed(),
            Message::BidResponse(m) => m.get_header_boxed(),
            Message::ListStrikePrice(m) => m.get_header_boxed(),
            Message::XmlMessage(m) => m.get_header_boxed(),
            Message::RegistrationInstructions(m) => m.get_header_boxed(),
            Message::RegistrationInstructionsResponse(m) => m.get_header_boxed(),
            Message::OrderMassCancelRequest(m) => m.get_header_boxed(),
            Message::OrderMassCancelReport(m) => m.get_header_boxed(),
            Message::NewOrderCross(m) => m.get_header_boxed(),
            Message::CrossOrderCancelReplaceRequest(m) => m.get_header_boxed(),
            Message::CrossOrderCancelRequest(m) => m.get_header_boxed(),
            Message::SecurityTypeRequest(m) => m.get_header_boxed(),
            Message::SecurityTypes(m) => m.get_header_boxed(),
            Message::SecurityListRequest(m) => m.get_header_boxed(),
            Message::SecurityList(m) => m.get_header_boxed(),
            Message::DerivativeSecurityListRequest(m) => m.get_header_boxed(),
            Message::DerivativeSecurityList(m) => m.get_header_boxed(),
            Message::NewOrderMultileg(m) => m.get_header_boxed(),
            Message::MultilegOrderCancelReplace(m) => m.get_header_boxed(),
            Message::TradeCaptureReportRequest(m) => m.get_header_boxed(),
            Message::TradeCaptureReport(m) => m.get_header_boxed(),
            Message::OrderMassStatusRequest(m) => m.get_header_boxed(),
            Message::QuoteRequestReject(m) => m.get_header_boxed(),
            Message::RfqRequest(m) => m.get_header_boxed(),
            Message::QuoteStatusReport(m) => m.get_header_boxed(),
            Message::QuoteResponse(m) => m.get_header_boxed(),
            Message::Confirmation(m) => m.get_header_boxed(),
            Message::PositionMaintenanceRequest(m) => m.get_header_boxed(),
            Message::PositionMaintenanceReport(m) => m.get_header_boxed(),
            Message::RequestForPositions(m) => m.get_header_boxed(),
            Message::RequestForPositionsAck(m) => m.get_header_boxed(),
            Message::PositionReport(m) => m.get_header_boxed(),
            Message::TradeCaptureReportRequestAck(m) => m.get_header_boxed(),
            Message::TradeCaptureReportAck(m) => m.get_header_boxed(),
            Message::AllocationReport(m) => m.get_header_boxed(),
            Message::AllocationReportAck(m) => m.get_header_boxed(),
            Message::ConfirmationAck(m) => m.get_header_boxed(),
            Message::SettlementInstructionRequest(m) => m.get_header_boxed(),
            Message::AssignmentReport(m) => m.get_header_boxed(),
            Message::CollateralRequest(m) => m.get_header_boxed(),
            Message::CollateralAssignment(m) => m.get_header_boxed(),
            Message::CollateralResponse(m) => m.get_header_boxed(),
            Message::CollateralReport(m) => m.get_header_boxed(),
            Message::CollateralInquiry(m) => m.get_header_boxed(),
            Message::NetworkCounterpartySystemStatusRequest(m) => m.get_header_boxed(),
            Message::NetworkCounterpartySystemStatusResponse(m) => m.get_header_boxed(),
            Message::UserRequest(m) => m.get_header_boxed(),
            Message::UserResponse(m) => m.get_header_boxed(),
            Message::CollateralInquiryAck(m) => m.get_header_boxed(),
            Message::ConfirmationRequest(m) => m.get_header_boxed(),
            Message::TradingSessionListRequest(m) => m.get_header_boxed(),
            Message::TradingSessionList(m) => m.get_header_boxed(),
            Message::SecurityListUpdateReport(m) => m.get_header_boxed(),
            Message::AdjustedPositionReport(m) => m.get_header_boxed(),
            Message::AllocationInstructionAlert(m) => m.get_header_boxed(),
            Message::ExecutionAcknowledgement(m) => m.get_header_boxed(),
            Message::ContraryIntentionReport(m) => m.get_header_boxed(),
            Message::SecurityDefinitionUpdateReport(m) => m.get_header_boxed(),
            Message::SettlementObligationReport(m) => m.get_header_boxed(),
            Message::DerivativeSecurityListUpdateReport(m) => m.get_header_boxed(),
            Message::TradingSessionListUpdateReport(m) => m.get_header_boxed(),
            Message::MarketDefinitionRequest(m) => m.get_header_boxed(),
            Message::MarketDefinition(m) => m.get_header_boxed(),
            Message::MarketDefinitionUpdateReport(m) => m.get_header_boxed(),
            Message::ApplicationMessageRequest(m) => m.get_header_boxed(),
            Message::ApplicationMessageRequestAck(m) => m.get_header_boxed(),
            Message::ApplicationMessageReport(m) => m.get_header_boxed(),
            Message::OrderMassActionReport(m) => m.get_header_boxed(),
            Message::OrderMassActionRequest(m) => m.get_header_boxed(),
            Message::UserNotification(m) => m.get_header_boxed(),
            Message::StreamAssignmentRequest(m) => m.get_header_boxed(),
            Message::StreamAssignmentReport(m) => m.get_header_boxed(),
            Message::StreamAssignmentReportAck(m) => m.get_header_boxed(),
        }
    }
}

impl<const V: u8, const T1: char, const T2: char> crate::header::Header for fixt11::standard_message_header::StandardMessageHeader<V, T1, T2> {
    fn get_sender_comp_id(&self) -> &str {
        &self.sender_comp_id
    }
    fn get_target_comp_id(&self) -> &str {
        &self.target_comp_id
    }
    fn get_msg_seq_num(&self) -> u32 {
        self.msg_seq_num
    }
	fn reply_boxed(&mut self, other: Box<&dyn crate::header::Header>) {
		self.sender_comp_id = other.get_target_comp_id().to_owned();
		self.target_comp_id = other.get_sender_comp_id().to_owned();
		self.msg_seq_num = other.get_msg_seq_num();
	}
}

impl<const V: u8, const T1: char, const T2: char> crate::header::HeaderExt for fixt11::standard_message_header::StandardMessageHeader<V, T1, T2> {
    fn get_appl_ver_id<const _V: u8>(&self) -> ApplVerID<_V> {
        self.appl_ver_id.convert()
    }
    fn reply<H: crate::header::HeaderExt>(&mut self, other: &H) {
        self.appl_ver_id = other.get_appl_ver_id();
        self.sender_comp_id = other.get_target_comp_id().to_owned();
        self.target_comp_id = other.get_sender_comp_id().to_owned();
        self.msg_seq_num = other.get_msg_seq_num();
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum IndicationOfInterest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::indication_of_interest::IndicationOfInterest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::indication_of_interest::IndicationOfInterest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "8")]
    FIX50SP2(Box<fix50sp2::messages::indication_of_interest::IndicationOfInterest>),
}

impl Serialize for IndicationOfInterest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			IndicationOfInterest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			IndicationOfInterest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			IndicationOfInterest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for IndicationOfInterest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            IndicationOfInterest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            IndicationOfInterest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            IndicationOfInterest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum Advertisement {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::advertisement::Advertisement>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::advertisement::Advertisement>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::advertisement::Advertisement>),
}

impl Serialize for Advertisement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			Advertisement::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			Advertisement::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			Advertisement::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for Advertisement {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            Advertisement::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            Advertisement::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            Advertisement::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ExecutionReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::execution_report::ExecutionReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::execution_report::ExecutionReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::execution_report::ExecutionReport>),
}

impl Serialize for ExecutionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			ExecutionReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			ExecutionReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ExecutionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ExecutionReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            ExecutionReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            ExecutionReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ExecutionReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderCancelReject {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_cancel_reject::OrderCancelReject>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_cancel_reject::OrderCancelReject>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_cancel_reject::OrderCancelReject>),
}

impl Serialize for OrderCancelReject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			OrderCancelReject::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			OrderCancelReject::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			OrderCancelReject::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for OrderCancelReject {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            OrderCancelReject::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            OrderCancelReject::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            OrderCancelReject::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum News {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::news::News>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::news::News>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::news::News>),
}

impl Serialize for News {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			News::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			News::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			News::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for News {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            News::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            News::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            News::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum Email {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::email::Email>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::email::Email>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::email::Email>),
}

impl Serialize for Email {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			Email::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			Email::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			Email::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for Email {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            Email::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            Email::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            Email::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NewOrderSingle {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::new_order_single::NewOrderSingle>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::new_order_single::NewOrderSingle>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::new_order_single::NewOrderSingle>),
}

impl Serialize for NewOrderSingle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			NewOrderSingle::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			NewOrderSingle::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			NewOrderSingle::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for NewOrderSingle {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            NewOrderSingle::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            NewOrderSingle::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            NewOrderSingle::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NewOrderList {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::new_order_list::NewOrderList>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::new_order_list::NewOrderList>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::new_order_list::NewOrderList>),
}

impl Serialize for NewOrderList {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			NewOrderList::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			NewOrderList::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			NewOrderList::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for NewOrderList {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            NewOrderList::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            NewOrderList::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            NewOrderList::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderCancelRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_cancel_request::OrderCancelRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_cancel_request::OrderCancelRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_cancel_request::OrderCancelRequest>),
}

impl Serialize for OrderCancelRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			OrderCancelRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			OrderCancelRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			OrderCancelRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for OrderCancelRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            OrderCancelRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            OrderCancelRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            OrderCancelRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderCancelReplaceRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_cancel_replace_request::OrderCancelReplaceRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_cancel_replace_request::OrderCancelReplaceRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_cancel_replace_request::OrderCancelReplaceRequest>),
}

impl Serialize for OrderCancelReplaceRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			OrderCancelReplaceRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			OrderCancelReplaceRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			OrderCancelReplaceRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for OrderCancelReplaceRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            OrderCancelReplaceRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            OrderCancelReplaceRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            OrderCancelReplaceRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderStatusRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_status_request::OrderStatusRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_status_request::OrderStatusRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_status_request::OrderStatusRequest>),
}

impl Serialize for OrderStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			OrderStatusRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			OrderStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			OrderStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for OrderStatusRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            OrderStatusRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            OrderStatusRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            OrderStatusRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationInstruction {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_instruction::AllocationInstruction>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_instruction::AllocationInstruction>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_instruction::AllocationInstruction>),
}

impl Serialize for AllocationInstruction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			AllocationInstruction::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			AllocationInstruction::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			AllocationInstruction::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for AllocationInstruction {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            AllocationInstruction::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            AllocationInstruction::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            AllocationInstruction::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListCancelRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_cancel_request::ListCancelRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_cancel_request::ListCancelRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_cancel_request::ListCancelRequest>),
}

impl Serialize for ListCancelRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			ListCancelRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			ListCancelRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ListCancelRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ListCancelRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            ListCancelRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            ListCancelRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ListCancelRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListExecute {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_execute::ListExecute>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_execute::ListExecute>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_execute::ListExecute>),
}

impl Serialize for ListExecute {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			ListExecute::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			ListExecute::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ListExecute::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ListExecute {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            ListExecute::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            ListExecute::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ListExecute::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListStatusRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_status_request::ListStatusRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_status_request::ListStatusRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_status_request::ListStatusRequest>),
}

impl Serialize for ListStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			ListStatusRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			ListStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ListStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ListStatusRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            ListStatusRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            ListStatusRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ListStatusRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListStatus {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_status::ListStatus>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_status::ListStatus>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_status::ListStatus>),
}

impl Serialize for ListStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			ListStatus::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			ListStatus::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ListStatus::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ListStatus {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            ListStatus::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            ListStatus::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ListStatus::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationInstructionAck {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_instruction_ack::AllocationInstructionAck>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_instruction_ack::AllocationInstructionAck>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_instruction_ack::AllocationInstructionAck>),
}

impl Serialize for AllocationInstructionAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			AllocationInstructionAck::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			AllocationInstructionAck::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			AllocationInstructionAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for AllocationInstructionAck {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            AllocationInstructionAck::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            AllocationInstructionAck::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            AllocationInstructionAck::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum DontKnowTrade {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::dont_know_trade::DontKnowTrade>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::dont_know_trade::DontKnowTrade>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::dont_know_trade::DontKnowTrade>),
}

impl Serialize for DontKnowTrade {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			DontKnowTrade::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			DontKnowTrade::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			DontKnowTrade::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for DontKnowTrade {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            DontKnowTrade::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            DontKnowTrade::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            DontKnowTrade::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_request::QuoteRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_request::QuoteRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_request::QuoteRequest>),
}

impl Serialize for QuoteRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			QuoteRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			QuoteRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			QuoteRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for QuoteRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            QuoteRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            QuoteRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            QuoteRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum Quote {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote::Quote>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote::Quote>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote::Quote>),
}

impl Serialize for Quote {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			Quote::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			Quote::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			Quote::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for Quote {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            Quote::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            Quote::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            Quote::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SettlementInstructions {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::settlement_instructions::SettlementInstructions>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::settlement_instructions::SettlementInstructions>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::settlement_instructions::SettlementInstructions>),
}

impl Serialize for SettlementInstructions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SettlementInstructions::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SettlementInstructions::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SettlementInstructions::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SettlementInstructions {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SettlementInstructions::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SettlementInstructions::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SettlementInstructions::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDataRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::market_data_request::MarketDataRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_data_request::MarketDataRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_data_request::MarketDataRequest>),
}

impl Serialize for MarketDataRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			MarketDataRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			MarketDataRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			MarketDataRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MarketDataRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            MarketDataRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            MarketDataRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            MarketDataRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDataSnapshotFullRefresh {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh>),
}

impl Serialize for MarketDataSnapshotFullRefresh {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			MarketDataSnapshotFullRefresh::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			MarketDataSnapshotFullRefresh::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			MarketDataSnapshotFullRefresh::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MarketDataSnapshotFullRefresh {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            MarketDataSnapshotFullRefresh::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            MarketDataSnapshotFullRefresh::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            MarketDataSnapshotFullRefresh::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDataIncrementalRefresh {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh>),
}

impl Serialize for MarketDataIncrementalRefresh {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			MarketDataIncrementalRefresh::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			MarketDataIncrementalRefresh::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			MarketDataIncrementalRefresh::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MarketDataIncrementalRefresh {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            MarketDataIncrementalRefresh::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            MarketDataIncrementalRefresh::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            MarketDataIncrementalRefresh::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDataRequestReject {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::market_data_request_reject::MarketDataRequestReject>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_data_request_reject::MarketDataRequestReject>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_data_request_reject::MarketDataRequestReject>),
}

impl Serialize for MarketDataRequestReject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			MarketDataRequestReject::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			MarketDataRequestReject::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			MarketDataRequestReject::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MarketDataRequestReject {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            MarketDataRequestReject::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            MarketDataRequestReject::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            MarketDataRequestReject::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteCancel {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_cancel::QuoteCancel>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_cancel::QuoteCancel>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_cancel::QuoteCancel>),
}

impl Serialize for QuoteCancel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			QuoteCancel::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			QuoteCancel::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			QuoteCancel::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for QuoteCancel {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            QuoteCancel::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            QuoteCancel::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            QuoteCancel::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteStatusRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_status_request::QuoteStatusRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_status_request::QuoteStatusRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_status_request::QuoteStatusRequest>),
}

impl Serialize for QuoteStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			QuoteStatusRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			QuoteStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			QuoteStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for QuoteStatusRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            QuoteStatusRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            QuoteStatusRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            QuoteStatusRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MassQuoteAcknowledgement {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::mass_quote_acknowledgement::MassQuoteAcknowledgement>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::mass_quote_acknowledgement::MassQuoteAcknowledgement>),
}

impl Serialize for MassQuoteAcknowledgement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			MassQuoteAcknowledgement::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			MassQuoteAcknowledgement::FIX50SP1(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MassQuoteAcknowledgement {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            MassQuoteAcknowledgement::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            MassQuoteAcknowledgement::FIX50SP1(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityDefinitionRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_definition_request::SecurityDefinitionRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_definition_request::SecurityDefinitionRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_definition_request::SecurityDefinitionRequest>),
}

impl Serialize for SecurityDefinitionRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityDefinitionRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityDefinitionRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityDefinitionRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityDefinitionRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityDefinitionRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityDefinitionRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityDefinitionRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityDefinition {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_definition::SecurityDefinition>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_definition::SecurityDefinition>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_definition::SecurityDefinition>),
}

impl Serialize for SecurityDefinition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityDefinition::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityDefinition::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityDefinition::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityDefinition {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityDefinition::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityDefinition::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityDefinition::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityStatusRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_status_request::SecurityStatusRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_status_request::SecurityStatusRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_status_request::SecurityStatusRequest>),
}

impl Serialize for SecurityStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityStatusRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityStatusRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityStatusRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityStatusRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityStatusRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityStatus {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_status::SecurityStatus>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_status::SecurityStatus>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_status::SecurityStatus>),
}

impl Serialize for SecurityStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityStatus::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityStatus::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityStatus::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityStatus {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityStatus::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityStatus::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityStatus::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionStatusRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trading_session_status_request::TradingSessionStatusRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_status_request::TradingSessionStatusRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_status_request::TradingSessionStatusRequest>),
}

impl Serialize for TradingSessionStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			TradingSessionStatusRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			TradingSessionStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			TradingSessionStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for TradingSessionStatusRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            TradingSessionStatusRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            TradingSessionStatusRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            TradingSessionStatusRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionStatus {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trading_session_status::TradingSessionStatus>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_status::TradingSessionStatus>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_status::TradingSessionStatus>),
}

impl Serialize for TradingSessionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			TradingSessionStatus::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			TradingSessionStatus::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			TradingSessionStatus::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for TradingSessionStatus {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            TradingSessionStatus::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            TradingSessionStatus::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            TradingSessionStatus::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MassQuote {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::mass_quote::MassQuote>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::mass_quote::MassQuote>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::mass_quote::MassQuote>),
}

impl Serialize for MassQuote {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			MassQuote::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			MassQuote::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			MassQuote::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MassQuote {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            MassQuote::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            MassQuote::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            MassQuote::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum BusinessMessageReject {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::business_message_reject::BusinessMessageReject>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::business_message_reject::BusinessMessageReject>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::business_message_reject::BusinessMessageReject>),
}

impl Serialize for BusinessMessageReject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			BusinessMessageReject::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			BusinessMessageReject::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			BusinessMessageReject::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for BusinessMessageReject {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            BusinessMessageReject::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            BusinessMessageReject::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            BusinessMessageReject::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum BidRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::bid_request::BidRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::bid_request::BidRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::bid_request::BidRequest>),
}

impl Serialize for BidRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			BidRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			BidRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			BidRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for BidRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            BidRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            BidRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            BidRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum BidResponse {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::bid_response::BidResponse>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::bid_response::BidResponse>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::bid_response::BidResponse>),
}

impl Serialize for BidResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			BidResponse::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			BidResponse::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			BidResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for BidResponse {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            BidResponse::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            BidResponse::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            BidResponse::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListStrikePrice {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_strike_price::ListStrikePrice>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_strike_price::ListStrikePrice>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_strike_price::ListStrikePrice>),
}

impl Serialize for ListStrikePrice {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			ListStrikePrice::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			ListStrikePrice::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ListStrikePrice::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ListStrikePrice {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            ListStrikePrice::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            ListStrikePrice::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ListStrikePrice::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum XmlMessage {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::xml_message::XmlMessage>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::xml_message::XmlMessage>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::xml_message::XmlMessage>),
}

impl Serialize for XmlMessage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			XmlMessage::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			XmlMessage::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			XmlMessage::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for XmlMessage {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            XmlMessage::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            XmlMessage::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            XmlMessage::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RegistrationInstructions {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::registration_instructions::RegistrationInstructions>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::registration_instructions::RegistrationInstructions>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::registration_instructions::RegistrationInstructions>),
}

impl Serialize for RegistrationInstructions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			RegistrationInstructions::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			RegistrationInstructions::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			RegistrationInstructions::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for RegistrationInstructions {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            RegistrationInstructions::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            RegistrationInstructions::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            RegistrationInstructions::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RegistrationInstructionsResponse {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::registration_instructions_response::RegistrationInstructionsResponse>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::registration_instructions_response::RegistrationInstructionsResponse>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::registration_instructions_response::RegistrationInstructionsResponse>),
}

impl Serialize for RegistrationInstructionsResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			RegistrationInstructionsResponse::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			RegistrationInstructionsResponse::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			RegistrationInstructionsResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for RegistrationInstructionsResponse {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            RegistrationInstructionsResponse::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            RegistrationInstructionsResponse::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            RegistrationInstructionsResponse::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassCancelRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_mass_cancel_request::OrderMassCancelRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_cancel_request::OrderMassCancelRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_cancel_request::OrderMassCancelRequest>),
}

impl Serialize for OrderMassCancelRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			OrderMassCancelRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			OrderMassCancelRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			OrderMassCancelRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for OrderMassCancelRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            OrderMassCancelRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            OrderMassCancelRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            OrderMassCancelRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassCancelReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_mass_cancel_report::OrderMassCancelReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_cancel_report::OrderMassCancelReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_cancel_report::OrderMassCancelReport>),
}

impl Serialize for OrderMassCancelReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			OrderMassCancelReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			OrderMassCancelReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			OrderMassCancelReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for OrderMassCancelReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            OrderMassCancelReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            OrderMassCancelReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            OrderMassCancelReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NewOrderCross {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::new_order_cross::NewOrderCross>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::new_order_cross::NewOrderCross>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::new_order_cross::NewOrderCross>),
}

impl Serialize for NewOrderCross {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			NewOrderCross::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			NewOrderCross::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			NewOrderCross::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for NewOrderCross {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            NewOrderCross::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            NewOrderCross::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            NewOrderCross::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CrossOrderCancelReplaceRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::cross_order_cancel_replace_request::CrossOrderCancelReplaceRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::cross_order_cancel_replace_request::CrossOrderCancelReplaceRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::cross_order_cancel_replace_request::CrossOrderCancelReplaceRequest>),
}

impl Serialize for CrossOrderCancelReplaceRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			CrossOrderCancelReplaceRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			CrossOrderCancelReplaceRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			CrossOrderCancelReplaceRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for CrossOrderCancelReplaceRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            CrossOrderCancelReplaceRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            CrossOrderCancelReplaceRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            CrossOrderCancelReplaceRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CrossOrderCancelRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::cross_order_cancel_request::CrossOrderCancelRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::cross_order_cancel_request::CrossOrderCancelRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::cross_order_cancel_request::CrossOrderCancelRequest>),
}

impl Serialize for CrossOrderCancelRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			CrossOrderCancelRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			CrossOrderCancelRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			CrossOrderCancelRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for CrossOrderCancelRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            CrossOrderCancelRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            CrossOrderCancelRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            CrossOrderCancelRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityTypeRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_type_request::SecurityTypeRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_type_request::SecurityTypeRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_type_request::SecurityTypeRequest>),
}

impl Serialize for SecurityTypeRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityTypeRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityTypeRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityTypeRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityTypeRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityTypeRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityTypeRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityTypeRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityTypes {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_types::SecurityTypes>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_types::SecurityTypes>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_types::SecurityTypes>),
}

impl Serialize for SecurityTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityTypes::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityTypes::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityTypes::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityTypes {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityTypes::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityTypes::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityTypes::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityListRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_list_request::SecurityListRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_list_request::SecurityListRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_list_request::SecurityListRequest>),
}

impl Serialize for SecurityListRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityListRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityListRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityListRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityListRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityListRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityListRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityListRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityList {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_list::SecurityList>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_list::SecurityList>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_list::SecurityList>),
}

impl Serialize for SecurityList {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityList::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityList::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityList::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityList {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityList::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityList::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityList::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum DerivativeSecurityListRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::derivative_security_list_request::DerivativeSecurityListRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::derivative_security_list_request::DerivativeSecurityListRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::derivative_security_list_request::DerivativeSecurityListRequest>),
}

impl Serialize for DerivativeSecurityListRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			DerivativeSecurityListRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			DerivativeSecurityListRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			DerivativeSecurityListRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for DerivativeSecurityListRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            DerivativeSecurityListRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            DerivativeSecurityListRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            DerivativeSecurityListRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum DerivativeSecurityList {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::derivative_security_list::DerivativeSecurityList>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::derivative_security_list::DerivativeSecurityList>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::derivative_security_list::DerivativeSecurityList>),
}

impl Serialize for DerivativeSecurityList {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			DerivativeSecurityList::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			DerivativeSecurityList::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			DerivativeSecurityList::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for DerivativeSecurityList {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            DerivativeSecurityList::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            DerivativeSecurityList::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            DerivativeSecurityList::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NewOrderMultileg {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::new_order_multileg::NewOrderMultileg>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::new_order_multileg::NewOrderMultileg>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::new_order_multileg::NewOrderMultileg>),
}

impl Serialize for NewOrderMultileg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			NewOrderMultileg::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			NewOrderMultileg::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			NewOrderMultileg::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for NewOrderMultileg {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            NewOrderMultileg::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            NewOrderMultileg::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            NewOrderMultileg::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MultilegOrderCancelReplace {
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::multileg_order_cancel_replace_request::MultilegOrderCancelReplaceRequest>),
}

impl Serialize for MultilegOrderCancelReplace {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp2")]
			MultilegOrderCancelReplace::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MultilegOrderCancelReplace {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp2")]
            MultilegOrderCancelReplace::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradeCaptureReportRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trade_capture_report_request::TradeCaptureReportRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trade_capture_report_request::TradeCaptureReportRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trade_capture_report_request::TradeCaptureReportRequest>),
}

impl Serialize for TradeCaptureReportRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			TradeCaptureReportRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			TradeCaptureReportRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			TradeCaptureReportRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for TradeCaptureReportRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            TradeCaptureReportRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            TradeCaptureReportRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            TradeCaptureReportRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradeCaptureReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trade_capture_report::TradeCaptureReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trade_capture_report::TradeCaptureReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trade_capture_report::TradeCaptureReport>),
}

impl Serialize for TradeCaptureReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			TradeCaptureReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			TradeCaptureReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			TradeCaptureReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for TradeCaptureReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            TradeCaptureReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            TradeCaptureReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            TradeCaptureReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassStatusRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_mass_status_request::OrderMassStatusRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_status_request::OrderMassStatusRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_status_request::OrderMassStatusRequest>),
}

impl Serialize for OrderMassStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			OrderMassStatusRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			OrderMassStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			OrderMassStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for OrderMassStatusRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            OrderMassStatusRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            OrderMassStatusRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            OrderMassStatusRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteRequestReject {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_request_reject::QuoteRequestReject>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_request_reject::QuoteRequestReject>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_request_reject::QuoteRequestReject>),
}

impl Serialize for QuoteRequestReject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			QuoteRequestReject::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			QuoteRequestReject::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			QuoteRequestReject::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for QuoteRequestReject {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            QuoteRequestReject::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            QuoteRequestReject::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            QuoteRequestReject::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RfqRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::rfq_request::RfqRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::rfq_request::RfqRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::rfq_request::RfqRequest>),
}

impl Serialize for RfqRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			RfqRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			RfqRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			RfqRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for RfqRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            RfqRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            RfqRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            RfqRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteStatusReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_status_report::QuoteStatusReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_status_report::QuoteStatusReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_status_report::QuoteStatusReport>),
}

impl Serialize for QuoteStatusReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			QuoteStatusReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			QuoteStatusReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			QuoteStatusReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for QuoteStatusReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            QuoteStatusReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            QuoteStatusReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            QuoteStatusReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteResponse {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_response::QuoteResponse>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_response::QuoteResponse>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_response::QuoteResponse>),
}

impl Serialize for QuoteResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			QuoteResponse::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			QuoteResponse::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			QuoteResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for QuoteResponse {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            QuoteResponse::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            QuoteResponse::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            QuoteResponse::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum Confirmation {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::confirmation::Confirmation>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::confirmation::Confirmation>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::confirmation::Confirmation>),
}

impl Serialize for Confirmation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			Confirmation::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			Confirmation::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			Confirmation::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for Confirmation {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            Confirmation::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            Confirmation::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            Confirmation::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum PositionMaintenanceRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::position_maintenance_request::PositionMaintenanceRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::position_maintenance_request::PositionMaintenanceRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::position_maintenance_request::PositionMaintenanceRequest>),
}

impl Serialize for PositionMaintenanceRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			PositionMaintenanceRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			PositionMaintenanceRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			PositionMaintenanceRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for PositionMaintenanceRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            PositionMaintenanceRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            PositionMaintenanceRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            PositionMaintenanceRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum PositionMaintenanceReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::position_maintenance_report::PositionMaintenanceReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::position_maintenance_report::PositionMaintenanceReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::position_maintenance_report::PositionMaintenanceReport>),
}

impl Serialize for PositionMaintenanceReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			PositionMaintenanceReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			PositionMaintenanceReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			PositionMaintenanceReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for PositionMaintenanceReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            PositionMaintenanceReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            PositionMaintenanceReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            PositionMaintenanceReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RequestForPositions {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::request_for_positions::RequestForPositions>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::request_for_positions::RequestForPositions>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::request_for_positions::RequestForPositions>),
}

impl Serialize for RequestForPositions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			RequestForPositions::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			RequestForPositions::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			RequestForPositions::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for RequestForPositions {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            RequestForPositions::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            RequestForPositions::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            RequestForPositions::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RequestForPositionsAck {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::request_for_positions_ack::RequestForPositionsAck>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::request_for_positions_ack::RequestForPositionsAck>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::request_for_positions_ack::RequestForPositionsAck>),
}

impl Serialize for RequestForPositionsAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			RequestForPositionsAck::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			RequestForPositionsAck::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			RequestForPositionsAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for RequestForPositionsAck {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            RequestForPositionsAck::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            RequestForPositionsAck::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            RequestForPositionsAck::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum PositionReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::position_report::PositionReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::position_report::PositionReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::position_report::PositionReport>),
}

impl Serialize for PositionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			PositionReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			PositionReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			PositionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for PositionReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            PositionReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            PositionReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            PositionReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradeCaptureReportRequestAck {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trade_capture_report_request_ack::TradeCaptureReportRequestAck>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trade_capture_report_request_ack::TradeCaptureReportRequestAck>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trade_capture_report_request_ack::TradeCaptureReportRequestAck>),
}

impl Serialize for TradeCaptureReportRequestAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			TradeCaptureReportRequestAck::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			TradeCaptureReportRequestAck::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			TradeCaptureReportRequestAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for TradeCaptureReportRequestAck {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            TradeCaptureReportRequestAck::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            TradeCaptureReportRequestAck::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            TradeCaptureReportRequestAck::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradeCaptureReportAck {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trade_capture_report_ack::TradeCaptureReportAck>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trade_capture_report_ack::TradeCaptureReportAck>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trade_capture_report_ack::TradeCaptureReportAck>),
}

impl Serialize for TradeCaptureReportAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			TradeCaptureReportAck::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			TradeCaptureReportAck::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			TradeCaptureReportAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for TradeCaptureReportAck {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            TradeCaptureReportAck::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            TradeCaptureReportAck::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            TradeCaptureReportAck::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_report::AllocationReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_report::AllocationReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_report::AllocationReport>),
}

impl Serialize for AllocationReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			AllocationReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			AllocationReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			AllocationReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for AllocationReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            AllocationReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            AllocationReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            AllocationReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationReportAck {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_report_ack::AllocationReportAck>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_report_ack::AllocationReportAck>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_report_ack::AllocationReportAck>),
}

impl Serialize for AllocationReportAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			AllocationReportAck::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			AllocationReportAck::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			AllocationReportAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for AllocationReportAck {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            AllocationReportAck::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            AllocationReportAck::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            AllocationReportAck::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ConfirmationAck {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::confirmation_ack::ConfirmationAck>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::confirmation_ack::ConfirmationAck>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::confirmation_ack::ConfirmationAck>),
}

impl Serialize for ConfirmationAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			ConfirmationAck::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			ConfirmationAck::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ConfirmationAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ConfirmationAck {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            ConfirmationAck::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            ConfirmationAck::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ConfirmationAck::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SettlementInstructionRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::settlement_instruction_request::SettlementInstructionRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::settlement_instruction_request::SettlementInstructionRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::settlement_instruction_request::SettlementInstructionRequest>),
}

impl Serialize for SettlementInstructionRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SettlementInstructionRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SettlementInstructionRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SettlementInstructionRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SettlementInstructionRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SettlementInstructionRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SettlementInstructionRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SettlementInstructionRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AssignmentReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::assignment_report::AssignmentReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::assignment_report::AssignmentReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::assignment_report::AssignmentReport>),
}

impl Serialize for AssignmentReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			AssignmentReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			AssignmentReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			AssignmentReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for AssignmentReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            AssignmentReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            AssignmentReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            AssignmentReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_request::CollateralRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_request::CollateralRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_request::CollateralRequest>),
}

impl Serialize for CollateralRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			CollateralRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			CollateralRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			CollateralRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for CollateralRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            CollateralRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            CollateralRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            CollateralRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralAssignment {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_assignment::CollateralAssignment>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_assignment::CollateralAssignment>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_assignment::CollateralAssignment>),
}

impl Serialize for CollateralAssignment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			CollateralAssignment::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			CollateralAssignment::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			CollateralAssignment::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for CollateralAssignment {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            CollateralAssignment::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            CollateralAssignment::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            CollateralAssignment::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralResponse {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_response::CollateralResponse>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_response::CollateralResponse>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_response::CollateralResponse>),
}

impl Serialize for CollateralResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			CollateralResponse::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			CollateralResponse::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			CollateralResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for CollateralResponse {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            CollateralResponse::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            CollateralResponse::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            CollateralResponse::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_report::CollateralReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_report::CollateralReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_report::CollateralReport>),
}

impl Serialize for CollateralReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			CollateralReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			CollateralReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			CollateralReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for CollateralReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            CollateralReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            CollateralReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            CollateralReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralInquiry {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_inquiry::CollateralInquiry>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_inquiry::CollateralInquiry>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_inquiry::CollateralInquiry>),
}

impl Serialize for CollateralInquiry {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			CollateralInquiry::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			CollateralInquiry::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			CollateralInquiry::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for CollateralInquiry {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            CollateralInquiry::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            CollateralInquiry::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            CollateralInquiry::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NetworkCounterpartySystemStatusRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::network_counterparty_system_status_request::NetworkCounterpartySystemStatusRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::network_counterparty_system_status_request::NetworkCounterpartySystemStatusRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::network_counterparty_system_status_request::NetworkCounterpartySystemStatusRequest>),
}

impl Serialize for NetworkCounterpartySystemStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			NetworkCounterpartySystemStatusRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			NetworkCounterpartySystemStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			NetworkCounterpartySystemStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for NetworkCounterpartySystemStatusRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            NetworkCounterpartySystemStatusRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            NetworkCounterpartySystemStatusRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            NetworkCounterpartySystemStatusRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NetworkCounterpartySystemStatusResponse {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::network_counterparty_system_status_response::NetworkCounterpartySystemStatusResponse>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::network_counterparty_system_status_response::NetworkCounterpartySystemStatusResponse>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::network_counterparty_system_status_response::NetworkCounterpartySystemStatusResponse>),
}

impl Serialize for NetworkCounterpartySystemStatusResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			NetworkCounterpartySystemStatusResponse::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			NetworkCounterpartySystemStatusResponse::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			NetworkCounterpartySystemStatusResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for NetworkCounterpartySystemStatusResponse {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            NetworkCounterpartySystemStatusResponse::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            NetworkCounterpartySystemStatusResponse::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            NetworkCounterpartySystemStatusResponse::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum UserRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::user_request::UserRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::user_request::UserRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::user_request::UserRequest>),
}

impl Serialize for UserRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			UserRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			UserRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			UserRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for UserRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            UserRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            UserRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            UserRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum UserResponse {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::user_response::UserResponse>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::user_response::UserResponse>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::user_response::UserResponse>),
}

impl Serialize for UserResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			UserResponse::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			UserResponse::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			UserResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for UserResponse {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            UserResponse::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            UserResponse::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            UserResponse::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralInquiryAck {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_inquiry_ack::CollateralInquiryAck>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_inquiry_ack::CollateralInquiryAck>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_inquiry_ack::CollateralInquiryAck>),
}

impl Serialize for CollateralInquiryAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			CollateralInquiryAck::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			CollateralInquiryAck::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			CollateralInquiryAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for CollateralInquiryAck {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            CollateralInquiryAck::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            CollateralInquiryAck::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            CollateralInquiryAck::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ConfirmationRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::confirmation_request::ConfirmationRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::confirmation_request::ConfirmationRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::confirmation_request::ConfirmationRequest>),
}

impl Serialize for ConfirmationRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			ConfirmationRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			ConfirmationRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ConfirmationRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ConfirmationRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            ConfirmationRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            ConfirmationRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ConfirmationRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionListRequest {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trading_session_list_request::TradingSessionListRequest>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_list_request::TradingSessionListRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_list_request::TradingSessionListRequest>),
}

impl Serialize for TradingSessionListRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			TradingSessionListRequest::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			TradingSessionListRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			TradingSessionListRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for TradingSessionListRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            TradingSessionListRequest::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            TradingSessionListRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            TradingSessionListRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionList {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trading_session_list::TradingSessionList>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_list::TradingSessionList>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_list::TradingSessionList>),
}

impl Serialize for TradingSessionList {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			TradingSessionList::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			TradingSessionList::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			TradingSessionList::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for TradingSessionList {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            TradingSessionList::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            TradingSessionList::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            TradingSessionList::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityListUpdateReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_list_update_report::SecurityListUpdateReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_list_update_report::SecurityListUpdateReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_list_update_report::SecurityListUpdateReport>),
}

impl Serialize for SecurityListUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityListUpdateReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityListUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityListUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityListUpdateReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityListUpdateReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityListUpdateReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityListUpdateReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AdjustedPositionReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::adjusted_position_report::AdjustedPositionReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::adjusted_position_report::AdjustedPositionReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::adjusted_position_report::AdjustedPositionReport>),
}

impl Serialize for AdjustedPositionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			AdjustedPositionReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			AdjustedPositionReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			AdjustedPositionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for AdjustedPositionReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            AdjustedPositionReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            AdjustedPositionReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            AdjustedPositionReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationInstructionAlert {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_instruction_alert::AllocationInstructionAlert>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_instruction_alert::AllocationInstructionAlert>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_instruction_alert::AllocationInstructionAlert>),
}

impl Serialize for AllocationInstructionAlert {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			AllocationInstructionAlert::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			AllocationInstructionAlert::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			AllocationInstructionAlert::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for AllocationInstructionAlert {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            AllocationInstructionAlert::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            AllocationInstructionAlert::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            AllocationInstructionAlert::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ExecutionAcknowledgement {
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::execution_ack::ExecutionAck>),
}

impl Serialize for ExecutionAcknowledgement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp2")]
			ExecutionAcknowledgement::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ExecutionAcknowledgement {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp2")]
            ExecutionAcknowledgement::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ContraryIntentionReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::contrary_intention_report::ContraryIntentionReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::contrary_intention_report::ContraryIntentionReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::contrary_intention_report::ContraryIntentionReport>),
}

impl Serialize for ContraryIntentionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			ContraryIntentionReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			ContraryIntentionReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ContraryIntentionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ContraryIntentionReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            ContraryIntentionReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            ContraryIntentionReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ContraryIntentionReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityDefinitionUpdateReport {
    /// FIX50
	#[cfg(feature = "fix_50")]
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_definition_update_report::SecurityDefinitionUpdateReport>),
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_definition_update_report::SecurityDefinitionUpdateReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_definition_update_report::SecurityDefinitionUpdateReport>),
}

impl Serialize for SecurityDefinitionUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50")]
			SecurityDefinitionUpdateReport::FIX50(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp1")]
			SecurityDefinitionUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SecurityDefinitionUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SecurityDefinitionUpdateReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50")]
            SecurityDefinitionUpdateReport::FIX50(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp1")]
            SecurityDefinitionUpdateReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SecurityDefinitionUpdateReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SettlementObligationReport {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::settlement_obligation_report::SettlementObligationReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::settlement_obligation_report::SettlementObligationReport>),
}

impl Serialize for SettlementObligationReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			SettlementObligationReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			SettlementObligationReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for SettlementObligationReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            SettlementObligationReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            SettlementObligationReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum DerivativeSecurityListUpdateReport {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::derivative_security_list_update_report::DerivativeSecurityListUpdateReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::derivative_security_list_update_report::DerivativeSecurityListUpdateReport>),
}

impl Serialize for DerivativeSecurityListUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			DerivativeSecurityListUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			DerivativeSecurityListUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for DerivativeSecurityListUpdateReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            DerivativeSecurityListUpdateReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            DerivativeSecurityListUpdateReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionListUpdateReport {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_list_update_report::TradingSessionListUpdateReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_list_update_report::TradingSessionListUpdateReport>),
}

impl Serialize for TradingSessionListUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			TradingSessionListUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			TradingSessionListUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for TradingSessionListUpdateReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            TradingSessionListUpdateReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            TradingSessionListUpdateReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDefinitionRequest {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_definition_request::MarketDefinitionRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_definition_request::MarketDefinitionRequest>),
}

impl Serialize for MarketDefinitionRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			MarketDefinitionRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			MarketDefinitionRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MarketDefinitionRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            MarketDefinitionRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            MarketDefinitionRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDefinition {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_definition::MarketDefinition>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_definition::MarketDefinition>),
}

impl Serialize for MarketDefinition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			MarketDefinition::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			MarketDefinition::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MarketDefinition {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            MarketDefinition::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            MarketDefinition::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDefinitionUpdateReport {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_definition_update_report::MarketDefinitionUpdateReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_definition_update_report::MarketDefinitionUpdateReport>),
}

impl Serialize for MarketDefinitionUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			MarketDefinitionUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			MarketDefinitionUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for MarketDefinitionUpdateReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            MarketDefinitionUpdateReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            MarketDefinitionUpdateReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ApplicationMessageRequest {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::application_message_request::ApplicationMessageRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::application_message_request::ApplicationMessageRequest>),
}

impl Serialize for ApplicationMessageRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			ApplicationMessageRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ApplicationMessageRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ApplicationMessageRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            ApplicationMessageRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ApplicationMessageRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ApplicationMessageRequestAck {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::application_message_request_ack::ApplicationMessageRequestAck>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::application_message_request_ack::ApplicationMessageRequestAck>),
}

impl Serialize for ApplicationMessageRequestAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			ApplicationMessageRequestAck::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ApplicationMessageRequestAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ApplicationMessageRequestAck {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            ApplicationMessageRequestAck::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ApplicationMessageRequestAck::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ApplicationMessageReport {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::application_message_report::ApplicationMessageReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::application_message_report::ApplicationMessageReport>),
}

impl Serialize for ApplicationMessageReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			ApplicationMessageReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			ApplicationMessageReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for ApplicationMessageReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            ApplicationMessageReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            ApplicationMessageReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassActionReport {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_action_report::OrderMassActionReport>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_action_report::OrderMassActionReport>),
}

impl Serialize for OrderMassActionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			OrderMassActionReport::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			OrderMassActionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for OrderMassActionReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            OrderMassActionReport::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            OrderMassActionReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassActionRequest {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_action_request::OrderMassActionRequest>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_action_request::OrderMassActionRequest>),
}

impl Serialize for OrderMassActionRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			OrderMassActionRequest::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			OrderMassActionRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for OrderMassActionRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            OrderMassActionRequest::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            OrderMassActionRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum UserNotification {
    /// FIX50SP1
	#[cfg(feature = "fix_50sp1")]
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::user_notification::UserNotification>),
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::user_notification::UserNotification>),
}

impl Serialize for UserNotification {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp1")]
			UserNotification::FIX50SP1(m) => m.serialize(serializer),
            #[cfg(feature = "fix_50sp2")]
			UserNotification::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for UserNotification {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp1")]
            UserNotification::FIX50SP1(m) => Box::new(m.get_header()),
            #[cfg(feature = "fix_50sp2")]
            UserNotification::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}
#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum StreamAssignmentRequest {
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::stream_assignment_request::StreamAssignmentRequest>),
}

impl Serialize for StreamAssignmentRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp2")]
			StreamAssignmentRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for StreamAssignmentRequest {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp2")]
            StreamAssignmentRequest::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum StreamAssignmentReport {
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::stream_assignment_report::StreamAssignmentReport>),
}

impl Serialize for StreamAssignmentReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp2")]
			StreamAssignmentReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for StreamAssignmentReport {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp2")]
            StreamAssignmentReport::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum StreamAssignmentReportAck {
    /// FIX50SP2
	#[cfg(feature = "fix_50sp2")]
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::stream_assignment_report_ack::StreamAssignmentReportAck>),
}

impl Serialize for StreamAssignmentReportAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            #[cfg(feature = "fix_50sp2")]
			StreamAssignmentReportAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeaderBoxed for StreamAssignmentReportAck {
    fn get_header_boxed<'a>(&'a self) -> Box<&'a dyn crate::header::Header> {
        match self {
            #[cfg(feature = "fix_50sp2")]
            StreamAssignmentReportAck::FIX50SP2(m) => Box::new(m.get_header()),
        }
    }
}

#[cfg(feature = "fix_50")]
mod _fix50 {
    use crate::has_header::HasHeader;

    impl HasHeader for fix50::messages::adjusted_position_report::AdjustedPositionReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'L'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::advertisement::Advertisement {
        type Output = fix50::standard_message_header::StandardMessageHeader<'7', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::allocation_instruction_ack::AllocationInstructionAck {
        type Output = fix50::standard_message_header::StandardMessageHeader<'P', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::allocation_instruction_alert::AllocationInstructionAlert {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'M'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::allocation_instruction::AllocationInstruction {
        type Output = fix50::standard_message_header::StandardMessageHeader<'J', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::allocation_report_ack::AllocationReportAck {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'T'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::allocation_report::AllocationReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'S'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::assignment_report::AssignmentReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'W'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::bid_request::BidRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'k', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::bid_response::BidResponse {
        type Output = fix50::standard_message_header::StandardMessageHeader<'l', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::business_message_reject::BusinessMessageReject {
        type Output = fix50::standard_message_header::StandardMessageHeader<'j', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::collateral_assignment::CollateralAssignment {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'Y'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::collateral_inquiry_ack::CollateralInquiryAck {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'G'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::collateral_inquiry::CollateralInquiry {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::collateral_report::CollateralReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::collateral_request::CollateralRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'X'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::collateral_response::CollateralResponse {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'Z'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::confirmation_ack::ConfirmationAck {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'U'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::confirmation_request::ConfirmationRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'H'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::confirmation::Confirmation {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'K'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::contrary_intention_report::ContraryIntentionReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'O'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::cross_order_cancel_replace_request::CrossOrderCancelReplaceRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'t', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::cross_order_cancel_request::CrossOrderCancelRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'u', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::derivative_security_list_request::DerivativeSecurityListRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'z', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::derivative_security_list::DerivativeSecurityList {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::dont_know_trade::DontKnowTrade {
        type Output = fix50::standard_message_header::StandardMessageHeader<'Q', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::email::Email {
        type Output = fix50::standard_message_header::StandardMessageHeader<'C', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::execution_report_acknowledgement::ExecutionReportAcknowledgement {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'N'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::execution_report::ExecutionReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'8', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::indication_of_interest::IndicationOfInterest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'6', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::list_cancel_request::ListCancelRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'K', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::list_execute::ListExecute {
        type Output = fix50::standard_message_header::StandardMessageHeader<'L', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::list_status_request::ListStatusRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'M', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::list_status::ListStatus {
        type Output = fix50::standard_message_header::StandardMessageHeader<'N', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::list_strike_price::ListStrikePrice {
        type Output = fix50::standard_message_header::StandardMessageHeader<'m', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh {
        type Output = fix50::standard_message_header::StandardMessageHeader<'X', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::market_data_request_reject::MarketDataRequestReject {
        type Output = fix50::standard_message_header::StandardMessageHeader<'Y', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::market_data_request::MarketDataRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'V', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh {
        type Output = fix50::standard_message_header::StandardMessageHeader<'W', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::mass_quote_acknowledgement::MassQuoteAcknowledgement {
        type Output = fix50::standard_message_header::StandardMessageHeader<'b', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::mass_quote::MassQuote {
        type Output = fix50::standard_message_header::StandardMessageHeader<'i', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::multileg_order_cancel_replace_request::MultilegOrderCancelReplaceRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'C'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::network_counterparty_system_status_request::NetworkCounterpartySystemStatusRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'C'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::network_counterparty_system_status_response::NetworkCounterpartySystemStatusResponse {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'D'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::new_order_cross::NewOrderCross {
        type Output = fix50::standard_message_header::StandardMessageHeader<'s', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::new_order_list::NewOrderList {
        type Output = fix50::standard_message_header::StandardMessageHeader<'E', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::new_order_multileg::NewOrderMultileg {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::new_order_single::NewOrderSingle {
        type Output = fix50::standard_message_header::StandardMessageHeader<'D', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::news::News {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::order_cancel_reject::OrderCancelReject {
        type Output = fix50::standard_message_header::StandardMessageHeader<'9', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::order_cancel_replace_request::OrderCancelReplaceRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'G', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::order_cancel_request::OrderCancelRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'F', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::order_mass_cancel_report::OrderMassCancelReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'r', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::order_mass_cancel_request::OrderMassCancelRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'q', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::order_mass_status_request::OrderMassStatusRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'F'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::order_status_request::OrderStatusRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'H', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::position_maintenance_report::PositionMaintenanceReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'M'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::position_maintenance_request::PositionMaintenanceRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'L'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::position_report::PositionReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'P'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::quote_cancel::QuoteCancel {
        type Output = fix50::standard_message_header::StandardMessageHeader<'Z', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::quote_request_reject::QuoteRequestReject {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'G'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::quote_request::QuoteRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'R', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::quote_response::QuoteResponse {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'J'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::quote::Quote {
        type Output = fix50::standard_message_header::StandardMessageHeader<'S', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::quote_status_report::QuoteStatusReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'I'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::quote_status_request::QuoteStatusRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'a', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::registration_instructions_response::RegistrationInstructionsResponse {
        type Output = fix50::standard_message_header::StandardMessageHeader<'p', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::registration_instructions::RegistrationInstructions {
        type Output = fix50::standard_message_header::StandardMessageHeader<'o', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::request_for_positions_ack::RequestForPositionsAck {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'O'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::request_for_positions::RequestForPositions {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'N'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::rfq_request::RfqRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'H'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_definition_request::SecurityDefinitionRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'c', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_definition::SecurityDefinition {
        type Output = fix50::standard_message_header::StandardMessageHeader<'d', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_definition_update_report::SecurityDefinitionUpdateReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'P'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_list_request::SecurityListRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'x', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_list::SecurityList {
        type Output = fix50::standard_message_header::StandardMessageHeader<'y', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_list_update_report::SecurityListUpdateReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'K'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_status_request::SecurityStatusRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'e', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_status::SecurityStatus {
        type Output = fix50::standard_message_header::StandardMessageHeader<'f', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_type_request::SecurityTypeRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'v', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::security_types::SecurityTypes {
        type Output = fix50::standard_message_header::StandardMessageHeader<'w', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::settlement_instruction_request::SettlementInstructionRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'V'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::settlement_instructions::SettlementInstructions {
        type Output = fix50::standard_message_header::StandardMessageHeader<'T', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::trade_capture_report_ack::TradeCaptureReportAck {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'R'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::trade_capture_report_request_ack::TradeCaptureReportRequestAck {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'Q'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::trade_capture_report_request::TradeCaptureReportRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'D'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::trade_capture_report::TradeCaptureReport {
        type Output = fix50::standard_message_header::StandardMessageHeader<'A', 'E'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::trading_session_list_request::TradingSessionListRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'I'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::trading_session_list::TradingSessionList {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'J'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::trading_session_status_request::TradingSessionStatusRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'g', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::trading_session_status::TradingSessionStatus {
        type Output = fix50::standard_message_header::StandardMessageHeader<'h', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::user_request::UserRequest {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'E'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::user_response::UserResponse {
        type Output = fix50::standard_message_header::StandardMessageHeader<'B', 'F'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50::messages::xml_message::XmlMessage {
        type Output = fix50::standard_message_header::StandardMessageHeader<'n', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fixt11::messages::heartbeat::Heartbeat {
        type Output = fixt11::standard_message_header::StandardMessageHeader<5, '0', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fixt11::messages::logon::Logon {
        type Output = fixt11::standard_message_header::StandardMessageHeader<5, 'A', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fixt11::messages::logout::Logout {
        type Output = fixt11::standard_message_header::StandardMessageHeader<5, '5', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fixt11::messages::reject::Reject {
        type Output = fixt11::standard_message_header::StandardMessageHeader<5, '3', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fixt11::messages::resend_request::ResendRequest {
        type Output = fixt11::standard_message_header::StandardMessageHeader<5, '2', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fixt11::messages::sequence_reset::SequenceReset {
        type Output = fixt11::standard_message_header::StandardMessageHeader<5, '4', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fixt11::messages::test_request::TestRequest {
        type Output = fixt11::standard_message_header::StandardMessageHeader<5, '1', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }
}

#[cfg(feature = "fix_50sp1")]
mod _fix50sp1 {
    use crate::has_header::HasHeader;

    impl HasHeader for fix50sp1::messages::adjusted_position_report::AdjustedPositionReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'L'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::advertisement::Advertisement {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'7', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::allocation_instruction_ack::AllocationInstructionAck {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'P', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::allocation_instruction_alert::AllocationInstructionAlert {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'M'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::allocation_instruction::AllocationInstruction {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'J', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::allocation_report_ack::AllocationReportAck {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'T'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::allocation_report::AllocationReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'S'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::application_message_report::ApplicationMessageReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'Y'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::application_message_request_ack::ApplicationMessageRequestAck {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'X'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::application_message_request::ApplicationMessageRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'W'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::assignment_report::AssignmentReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'W'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::bid_request::BidRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'k', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::bid_response::BidResponse {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'l', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::business_message_reject::BusinessMessageReject {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'j', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::collateral_assignment::CollateralAssignment {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'Y'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::collateral_inquiry_ack::CollateralInquiryAck {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'G'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::collateral_inquiry::CollateralInquiry {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::collateral_report::CollateralReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::collateral_request::CollateralRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'X'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::collateral_response::CollateralResponse {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'Z'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::confirmation_ack::ConfirmationAck {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'U'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::confirmation_request::ConfirmationRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'H'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::confirmation::Confirmation {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'K'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::contrary_intention_report::ContraryIntentionReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'O'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::cross_order_cancel_replace_request::CrossOrderCancelReplaceRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'t', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::cross_order_cancel_request::CrossOrderCancelRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'u', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::derivative_security_list_request::DerivativeSecurityListRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'z', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::derivative_security_list::DerivativeSecurityList {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::derivative_security_list_update_report::DerivativeSecurityListUpdateReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'R'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::dont_know_trade::DontKnowTrade {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'Q', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::email::Email {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'C', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::execution_report_acknowledgement::ExecutionReportAcknowledgement {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'N'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::execution_report::ExecutionReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'8', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::indication_of_interest::IndicationOfInterest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'6', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::list_cancel_request::ListCancelRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'K', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::list_execute::ListExecute {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'L', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::list_status_request::ListStatusRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'M', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::list_status::ListStatus {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'N', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::list_strike_price::ListStrikePrice {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'m', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'X', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::market_data_request_reject::MarketDataRequestReject {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'Y', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::market_data_request::MarketDataRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'V', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'W', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::market_definition_request::MarketDefinitionRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'T'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::market_definition::MarketDefinition {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'U'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::market_definition_update_report::MarketDefinitionUpdateReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'V'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::mass_quote_acknowledgement::MassQuoteAcknowledgement {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'b', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::mass_quote::MassQuote {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'i', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::multileg_order_cancel_replace_request::MultilegOrderCancelReplaceRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'C'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::network_counterparty_system_status_request::NetworkCounterpartySystemStatusRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'C'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::network_counterparty_system_status_response::NetworkCounterpartySystemStatusResponse {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'D'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::new_order_cross::NewOrderCross {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'s', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::new_order_list::NewOrderList {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'E', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::new_order_multileg::NewOrderMultileg {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::new_order_single::NewOrderSingle {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'D', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::news::News {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::order_cancel_reject::OrderCancelReject {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'9', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::order_cancel_replace_request::OrderCancelReplaceRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'G', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::order_cancel_request::OrderCancelRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'F', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::order_mass_action_report::OrderMassActionReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'Z'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::order_mass_action_request::OrderMassActionRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'C', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::order_mass_cancel_report::OrderMassCancelReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'r', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::order_mass_cancel_request::OrderMassCancelRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'q', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::order_mass_status_request::OrderMassStatusRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'F'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::order_status_request::OrderStatusRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'H', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::position_maintenance_report::PositionMaintenanceReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'M'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::position_maintenance_request::PositionMaintenanceRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'L'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::position_report::PositionReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'P'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::quote_cancel::QuoteCancel {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'Z', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::quote_request_reject::QuoteRequestReject {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'G'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::quote_request::QuoteRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'R', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::quote_response::QuoteResponse {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'J'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::quote::Quote {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'S', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::quote_status_report::QuoteStatusReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'I'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::quote_status_request::QuoteStatusRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'a', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::registration_instructions_response::RegistrationInstructionsResponse {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'p', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::registration_instructions::RegistrationInstructions {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'o', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::request_for_positions_ack::RequestForPositionsAck {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'O'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::request_for_positions::RequestForPositions {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'N'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::rfq_request::RfqRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'H'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_definition_request::SecurityDefinitionRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'c', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_definition::SecurityDefinition {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'d', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_definition_update_report::SecurityDefinitionUpdateReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'P'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_list_request::SecurityListRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'x', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_list::SecurityList {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'y', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_list_update_report::SecurityListUpdateReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'K'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_status_request::SecurityStatusRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'e', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_status::SecurityStatus {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'f', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_type_request::SecurityTypeRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'v', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::security_types::SecurityTypes {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'w', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::settlement_instruction_request::SettlementInstructionRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'V'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::settlement_instructions::SettlementInstructions {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'T', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::settlement_obligation_report::SettlementObligationReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'Q'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::trade_capture_report_ack::TradeCaptureReportAck {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'R'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::trade_capture_report_request_ack::TradeCaptureReportRequestAck {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'Q'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::trade_capture_report_request::TradeCaptureReportRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'D'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::trade_capture_report::TradeCaptureReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'A', 'E'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::trading_session_list_request::TradingSessionListRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'I'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::trading_session_list::TradingSessionList {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'J'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::trading_session_list_update_report::TradingSessionListUpdateReport {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'S'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::trading_session_status_request::TradingSessionStatusRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'g', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::trading_session_status::TradingSessionStatus {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'h', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::user_notification::UserNotification {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'C', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::user_request::UserRequest {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'E'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::user_response::UserResponse {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'B', 'F'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp1::messages::xml_message::XmlMessage {
        type Output = fix50sp1::standard_message_header::StandardMessageHeader<'n', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }
}

#[cfg(feature = "fix_50sp2")]
mod _fix50sp2 {
    use crate::has_header::HasHeader;

    impl HasHeader for fix50sp2::messages::account_summary_report::AccountSummaryReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'Q'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::adjusted_position_report::AdjustedPositionReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'L'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::advertisement::Advertisement {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'7', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::allocation_instruction_ack::AllocationInstructionAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'P', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::allocation_instruction_alert_request_ack::AllocationInstructionAlertRequestAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'V'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::allocation_instruction_alert_request::AllocationInstructionAlertRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'U'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::allocation_instruction_alert::AllocationInstructionAlert {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'M'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::allocation_instruction::AllocationInstruction {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'J', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::allocation_report_ack::AllocationReportAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'T'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::allocation_report::AllocationReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'S'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::application_message_report::ApplicationMessageReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'Y'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::application_message_request_ack::ApplicationMessageRequestAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'X'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::application_message_request::ApplicationMessageRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'W'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::assignment_report::AssignmentReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'W'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::bid_request::BidRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'k', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::bid_response::BidResponse {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'l', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::business_message_reject::BusinessMessageReject {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'j', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::collateral_assignment::CollateralAssignment {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'Y'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::collateral_inquiry_ack::CollateralInquiryAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'G'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::collateral_inquiry::CollateralInquiry {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::collateral_report_ack::CollateralReportAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'Q'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::collateral_report::CollateralReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::collateral_request::CollateralRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'X'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::collateral_response::CollateralResponse {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'Z'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::confirmation_ack::ConfirmationAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'U'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::confirmation_request::ConfirmationRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'H'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::confirmation::Confirmation {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'K'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::contrary_intention_report::ContraryIntentionReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'O'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::cross_order_cancel_replace_request::CrossOrderCancelReplaceRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'t', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::cross_order_cancel_request::CrossOrderCancelRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'u', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::cross_request_ack::CrossRequestAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'T'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::cross_request::CrossRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'S'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::derivative_security_list_request::DerivativeSecurityListRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'z', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::derivative_security_list::DerivativeSecurityList {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::derivative_security_list_update_report::DerivativeSecurityListUpdateReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'R'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::dont_know_trade::DontKnowTrade {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'Q', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::email::Email {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::execution_ack::ExecutionAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'N'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::execution_report::ExecutionReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'8', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::indication_of_interest::IndicationOfInterest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'6', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::list_cancel_request::ListCancelRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'K', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::list_execute::ListExecute {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'L', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::list_status_request::ListStatusRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'M', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::list_status::ListStatus {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'N', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::list_strike_price::ListStrikePrice {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'m', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::margin_requirement_inquiry_ack::MarginRequirementInquiryAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'I'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::margin_requirement_inquiry::MarginRequirementInquiry {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'H'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::margin_requirement_report::MarginRequirementReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'J'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'X', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_data_report::MarketDataReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'R'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_data_request_reject::MarketDataRequestReject {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'Y', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_data_request::MarketDataRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'V', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'W', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_data_statistics_report::MarketDataStatisticsReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'P'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_data_statistics_request::MarketDataStatisticsRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'O'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_definition_request::MarketDefinitionRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'T'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_definition::MarketDefinition {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'U'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::market_definition_update_report::MarketDefinitionUpdateReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'V'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::mass_order_ack::MassOrderAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'K'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::mass_order::MassOrder {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'J'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::mass_quote_ack::MassQuoteAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'b', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::mass_quote::MassQuote {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'i', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::multileg_order_cancel_replace_request::MultilegOrderCancelReplaceRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'C'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::network_counterparty_system_status_request::NetworkCounterpartySystemStatusRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'C'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::network_counterparty_system_status_response::NetworkCounterpartySystemStatusResponse {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'D'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::new_order_cross::NewOrderCross {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'s', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::new_order_list::NewOrderList {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'E', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::new_order_multileg::NewOrderMultileg {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::new_order_single::NewOrderSingle {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::news::News {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::order_cancel_reject::OrderCancelReject {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'9', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::order_cancel_replace_request::OrderCancelReplaceRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'G', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::order_cancel_request::OrderCancelRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'F', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::order_mass_action_report::OrderMassActionReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'Z'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::order_mass_action_request::OrderMassActionRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::order_mass_cancel_report::OrderMassCancelReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'r', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::order_mass_cancel_request::OrderMassCancelRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'q', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::order_mass_status_request::OrderMassStatusRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'F'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::order_status_request::OrderStatusRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'H', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_action_report::PartyActionReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'I'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_action_request::PartyActionRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'H'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_details_definition_request_ack::PartyDetailsDefinitionRequestAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'Y'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_details_definition_request::PartyDetailsDefinitionRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'X'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_details_list_report::PartyDetailsListReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'G'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_details_list_request::PartyDetailsListRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'F'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_details_list_update_report::PartyDetailsListUpdateReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'K'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_entitlements_definition_request_ack::PartyEntitlementsDefinitionRequestAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_entitlements_definition_request::PartyEntitlementsDefinitionRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_entitlements_report::PartyEntitlementsReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'V'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_entitlements_request::PartyEntitlementsRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'U'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_entitlements_update_report::PartyEntitlementsUpdateReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'Z'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_risk_limit_check_request_ack::PartyRiskLimitCheckRequestAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'G'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_risk_limit_check_request::PartyRiskLimitCheckRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'F'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_risk_limits_definition_request_ack::PartyRiskLimitsDefinitionRequestAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'T'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_risk_limits_definition_request::PartyRiskLimitsDefinitionRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'S'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_risk_limits_report_ack::PartyRiskLimitsReportAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'E'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_risk_limits_report::PartyRiskLimitsReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'M'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_risk_limits_request::PartyRiskLimitsRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'L'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::party_risk_limits_update_report::PartyRiskLimitsUpdateReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'R'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::pay_management_report_ack::PayManagementReportAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'Z'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::pay_management_report::PayManagementReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'Y'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::pay_management_request_ack::PayManagementRequestAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'E', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::pay_management_request::PayManagementRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'E', 'A'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::position_maintenance_report::PositionMaintenanceReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'M'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::position_maintenance_request::PositionMaintenanceRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'L'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::position_report::PositionReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'P'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::position_transfer_instruction_ack::PositionTransferInstructionAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'M'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::position_transfer_instruction::PositionTransferInstruction {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'L'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::position_transfer_report::PositionTransferReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'N'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::quote_ack::QuoteAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'W'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::quote_cancel::QuoteCancel {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'Z', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::quote_request_reject::QuoteRequestReject {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'G'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::quote_request::QuoteRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'R', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::quote_response::QuoteResponse {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'J'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::quote::Quote {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'S', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::quote_status_report::QuoteStatusReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'I'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::quote_status_request::QuoteStatusRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'a', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::registration_instructions_response::RegistrationInstructionsResponse {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'p', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::registration_instructions::RegistrationInstructions {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'o', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::request_for_positions_ack::RequestForPositionsAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'O'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::request_for_positions::RequestForPositions {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'N'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::rfq_request::RfqRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'H'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_definition_request::SecurityDefinitionRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'c', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_definition::SecurityDefinition {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'d', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_definition_update_report::SecurityDefinitionUpdateReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'P'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_list_request::SecurityListRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'x', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_list::SecurityList {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'y', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_list_update_report::SecurityListUpdateReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'K'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_mass_status_request::SecurityMassStatusRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'N'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_mass_status::SecurityMassStatus {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'O'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_status_request::SecurityStatusRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'e', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_status::SecurityStatus {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'f', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_type_request::SecurityTypeRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'v', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::security_types::SecurityTypes {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'w', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::settlement_instruction_request::SettlementInstructionRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'V'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::settlement_instructions::SettlementInstructions {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'T', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::settlement_obligation_report::SettlementObligationReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'Q'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::stream_assignment_report_ack::StreamAssignmentReportAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'E'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::stream_assignment_report::StreamAssignmentReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'D'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::stream_assignment_request::StreamAssignmentRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'C'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trade_aggregation_report::TradeAggregationReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'X'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trade_aggregation_request::TradeAggregationRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'W'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trade_capture_report_ack::TradeCaptureReportAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'R'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trade_capture_report_request_ack::TradeCaptureReportRequestAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'Q'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trade_capture_report_request::TradeCaptureReportRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'D'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trade_capture_report::TradeCaptureReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'A', 'E'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trade_match_report_ack::TradeMatchReportAck {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'D'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trade_match_report::TradeMatchReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'D', 'C'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trading_session_list_request::TradingSessionListRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'I'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trading_session_list::TradingSessionList {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'J'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trading_session_list_update_report::TradingSessionListUpdateReport {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'S'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trading_session_status_request::TradingSessionStatusRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'g', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::trading_session_status::TradingSessionStatus {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'h', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::user_notification::UserNotification {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'C', 'B'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::user_request::UserRequest {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'E'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::user_response::UserResponse {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'B', 'F'>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }

    impl HasHeader for fix50sp2::messages::xml_message::XmlMessage {
        type Output = fix50sp2::standard_message_header::StandardMessageHeader<'n', ' '>;
        fn get_header(&self) -> &Self::Output {
            &self.standard_message_header
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Message;

    #[test]
    fn logon() {
        let msg = "8=FIXT.1.1\u{1}9=118\u{1}35=A\u{1}1128=5\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}212=10\u{1}213=0123456789\u{1}369=1\u{1}98=0\u{1}108=1\u{1}789=1\u{1}1137=0\u{1}10=143\u{1}";
        let mut obj = dbg!(serde_fix::from_str_checked::<Message>(msg)).unwrap();
        match obj {
            Message::FIXT11(super::Message::Logon(ref mut l)) => {
                l.standard_message_header.body_length = 0;
            },
            _ => panic!("Deserialized message is not of type Logon"),
        }
        assert_eq!(serde_fix::to_string_checked(&obj), Ok(msg.to_owned()));
    }
}
