
use serde::{Deserialize, Serialize};

use fixt11::header::Header;

pub trait HasHeader {
    fn get_header(&self) -> &Header;
    fn get_header_mut(&mut self) -> &mut Header;
}

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
    IndicationOfInterest(Box<IndicationOfInterest>),
    /// Advertisement
    #[serde(rename = "7")]
    Advertisement(Box<Advertisement>),
    /// Execution Report
    #[serde(rename = "8")]
    ExecutionReport(ExecutionReport),
    /// Order Cancel Reject
    #[serde(rename = "9")]
    OrderCancelReject(Box<OrderCancelReject>),
    /// Logon
    #[serde(rename = "A")]
    Logon(Box<fixt11::messages::Logon>),
    /// News
    #[serde(rename = "B")]
    News(Box<News>),
    /// Email
    #[serde(rename = "C")]
    Email(Box<Email>),
    /// New Order - Single
    #[serde(rename = "D")]
    NewOrderSingle(NewOrderSingle),
    /// New Order - List
    #[serde(rename = "E")]
    NewOrderList(Box<NewOrderList>),
    /// Order Cancel Request
    #[serde(rename = "F")]
    OrderCancelRequest(Box<OrderCancelRequest>),
    /// Order Cancel/Replace Request
    #[serde(rename = "G")]
    OrderCancelReplaceRequest(Box<OrderCancelReplaceRequest>),
    /// Order Status Request
    #[serde(rename = "H")]
    OrderStatusRequest(Box<OrderStatusRequest>),
    /// Allocation Instruction
    #[serde(rename = "J")]
    AllocationInstruction(Box<AllocationInstruction>),
    /// List Cancel Request
    #[serde(rename = "K")]
    ListCancelRequest(Box<ListCancelRequest>),
    /// List Execute
    #[serde(rename = "L")]
    ListExecute(Box<ListExecute>),
    /// List Status Request
    #[serde(rename = "M")]
    ListStatusRequest(Box<ListStatusRequest>),
    /// List Status
    #[serde(rename = "N")]
    ListStatus(Box<ListStatus>),
    /// Allocation Instruction Ack
    #[serde(rename = "P")]
    AllocationInstructionAck(Box<AllocationInstructionAck>),
    /// Don't Know Trade
    #[serde(rename = "Q")]
    DontKnowTrade(Box<DontKnowTrade>),
    /// Quote Request
    #[serde(rename = "R")]
    QuoteRequest(Box<QuoteRequest>),
    /// Quote
    #[serde(rename = "S")]
    Quote(Box<Quote>),
    /// Settlement Instructions
    #[serde(rename = "T")]
    SettlementInstructions(Box<SettlementInstructions>),
    /// Market Data Request
    #[serde(rename = "V")]
    MarketDataRequest(Box<MarketDataRequest>),
    /// Market Data - Snapshot/Full Refresh
    #[serde(rename = "W")]
    MarketDataSnapshotFullRefresh(Box<MarketDataSnapshotFullRefresh>),
    /// Market Data - Incremental Refresh
    #[serde(rename = "X")]
    MarketDataIncrementalRefresh(Box<MarketDataIncrementalRefresh>),
    /// Market Data Request Reject
    #[serde(rename = "Y")]
    MarketDataRequestReject(Box<MarketDataRequestReject>),
    /// Quote Cancel
    #[serde(rename = "Z")]
    QuoteCancel(Box<QuoteCancel>),
    /// Quote Status Request
    #[serde(rename = "a")]
    QuoteStatusRequest(Box<QuoteStatusRequest>),
    /// Mass Quote Acknowledgement
    #[serde(rename = "b")]
    MassQuoteAcknowledgement(Box<MassQuoteAcknowledgement>),
    /// Security Definition Request
    #[serde(rename = "c")]
    SecurityDefinitionRequest(Box<SecurityDefinitionRequest>),
    /// Security Definition
    #[serde(rename = "d")]
    SecurityDefinition(Box<SecurityDefinition>),
    /// Security Status Request
    #[serde(rename = "e")]
    SecurityStatusRequest(Box<SecurityStatusRequest>),
    /// Security Status
    #[serde(rename = "f")]
    SecurityStatus(Box<SecurityStatus>),
    /// Trading Session Status Request
    #[serde(rename = "g")]
    TradingSessionStatusRequest(Box<TradingSessionStatusRequest>),
    /// Trading Session Status
    #[serde(rename = "h")]
    TradingSessionStatus(Box<TradingSessionStatus>),
    /// Mass Quote
    #[serde(rename = "i")]
    MassQuote(Box<MassQuote>),
    /// Business Message Reject
    #[serde(rename = "j")]
    BusinessMessageReject(Box<BusinessMessageReject>),
    /// Bid Request
    #[serde(rename = "k")]
    BidRequest(Box<BidRequest>),
    /// Bid Response
    #[serde(rename = "l")]
    BidResponse(Box<BidResponse>),
    /// List Strike Price
    #[serde(rename = "m")]
    ListStrikePrice(Box<ListStrikePrice>),
    /// XML message
    #[serde(rename = "n")]
    XMLMessage(Box<XMLMessage>),
    /// Registration Instructions
    #[serde(rename = "o")]
    RegistrationInstructions(Box<RegistrationInstructions>),
    /// Registration Instructions Response
    #[serde(rename = "p")]
    RegistrationInstructionsResponse(Box<RegistrationInstructionsResponse>),
    /// Order Mass Cancel Request
    #[serde(rename = "q")]
    OrderMassCancelRequest(Box<OrderMassCancelRequest>),
    /// Order Mass Cancel Report
    #[serde(rename = "r")]
    OrderMassCancelReport(Box<OrderMassCancelReport>),
    /// New Order - Cross
    #[serde(rename = "s")]
    NewOrderCross(Box<NewOrderCross>),
    /// Cross Order Cancel/Replace Request
    #[serde(rename = "t")]
    CrossOrderCancelReplaceRequest(Box<CrossOrderCancelReplaceRequest>),
    /// Cross Order Cancel Request
    #[serde(rename = "u")]
    CrossOrderCancelRequest(Box<CrossOrderCancelRequest>),
    /// Security Type Request
    #[serde(rename = "v")]
    SecurityTypeRequest(Box<SecurityTypeRequest>),
    /// Security Types
    #[serde(rename = "w")]
    SecurityTypes(Box<SecurityTypes>),
    /// Security List Request
    #[serde(rename = "x")]
    SecurityListRequest(Box<SecurityListRequest>),
    /// Security List
    #[serde(rename = "y")]
    SecurityList(Box<SecurityList>),
    /// Derivative Security List Request
    #[serde(rename = "z")]
    DerivativeSecurityListRequest(Box<DerivativeSecurityListRequest>),
    /// Derivative Security List
    #[serde(rename = "AA")]
    DerivativeSecurityList(Box<DerivativeSecurityList>),
    /// New Order - Multileg
    #[serde(rename = "AB")]
    NewOrderMultileg(Box<NewOrderMultileg>),
    /// Multileg Order Cancel/Replace
    #[serde(rename = "AC")]
    MultilegOrderCancelReplace(Box<MultilegOrderCancelReplace>),
    /// Trade Capture Report Request
    #[serde(rename = "AD")]
    TradeCaptureReportRequest(Box<TradeCaptureReportRequest>),
    /// Trade Capture Report
    #[serde(rename = "AE")]
    TradeCaptureReport(Box<TradeCaptureReport>),
    /// Order Mass Status Request
    #[serde(rename = "AF")]
    OrderMassStatusRequest(Box<OrderMassStatusRequest>),
    /// Quote Request Reject
    #[serde(rename = "AG")]
    QuoteRequestReject(Box<QuoteRequestReject>),
    /// RFQ Request
    #[serde(rename = "AH")]
    RFQRequest(Box<RFQRequest>),
    /// Quote Status Report
    #[serde(rename = "AI")]
    QuoteStatusReport(Box<QuoteStatusReport>),
    /// Quote Response
    #[serde(rename = "AJ")]
    QuoteResponse(Box<QuoteResponse>),
    /// Confirmation
    #[serde(rename = "AK")]
    Confirmation(Box<Confirmation>),
    /// Position Maintenance Request
    #[serde(rename = "AL")]
    PositionMaintenanceRequest(Box<PositionMaintenanceRequest>),
    /// Position Maintenance Report
    #[serde(rename = "AM")]
    PositionMaintenanceReport(Box<PositionMaintenanceReport>),
    /// Request For Positions
    #[serde(rename = "AN")]
    RequestForPositions(Box<RequestForPositions>),
    /// Request For Positions Ack
    #[serde(rename = "AO")]
    RequestForPositionsAck(Box<RequestForPositionsAck>),
    /// Position Report
    #[serde(rename = "AP")]
    PositionReport(Box<PositionReport>),
    /// Trade Capture Report Request Ack
    #[serde(rename = "AQ")]
    TradeCaptureReportRequestAck(Box<TradeCaptureReportRequestAck>),
    /// Trade Capture Report Ack
    #[serde(rename = "AR")]
    TradeCaptureReportAck(Box<TradeCaptureReportAck>),
    /// Allocation Report
    #[serde(rename = "AS")]
    AllocationReport(Box<AllocationReport>),
    /// Allocation Report Ack
    #[serde(rename = "AT")]
    AllocationReportAck(Box<AllocationReportAck>),
    /// Confirmation Ack
    #[serde(rename = "AU")]
    ConfirmationAck(Box<ConfirmationAck>),
    /// Settlement Instruction Request
    #[serde(rename = "AV")]
    SettlementInstructionRequest(Box<SettlementInstructionRequest>),
    /// Assignment Report
    #[serde(rename = "AW")]
    AssignmentReport(Box<AssignmentReport>),
    /// Collateral Request
    #[serde(rename = "AX")]
    CollateralRequest(Box<CollateralRequest>),
    /// Collateral Assignment
    #[serde(rename = "AY")]
    CollateralAssignment(Box<CollateralAssignment>),
    /// Collateral Response
    #[serde(rename = "AZ")]
    CollateralResponse(Box<CollateralResponse>),
    /// Collateral Report
    #[serde(rename = "BA")]
    CollateralReport(Box<CollateralReport>),
    /// Collateral Inquiry
    #[serde(rename = "BB")]
    CollateralInquiry(Box<CollateralInquiry>),
    /// Network Counterparty System Status Request
    #[serde(rename = "BC")]
    NetworkCounterpartySystemStatusRequest(Box<NetworkCounterpartySystemStatusRequest>),
    /// Network Counterparty System Status Response
    #[serde(rename = "BD")]
    NetworkCounterpartySystemStatusResponse(Box<NetworkCounterpartySystemStatusResponse>),
    /// User Request
    #[serde(rename = "BE")]
    UserRequest(Box<UserRequest>),
    /// User Response
    #[serde(rename = "BF")]
    UserResponse(Box<UserResponse>),
    /// Collateral Inquiry Ack
    #[serde(rename = "BG")]
    CollateralInquiryAck(Box<CollateralInquiryAck>),
    /// Confirmation Request
    #[serde(rename = "BH")]
    ConfirmationRequest(Box<ConfirmationRequest>),
    /// Trading Session List Request
    #[serde(rename = "BI")]
    TradingSessionListRequest(Box<TradingSessionListRequest>),
    /// Trading Session List
    #[serde(rename = "BJ")]
    TradingSessionList(Box<TradingSessionList>),
    /// Security List Update Report
    #[serde(rename = "BK")]
    SecurityListUpdateReport(Box<SecurityListUpdateReport>),
    /// Adjusted Position Report
    #[serde(rename = "BL")]
    AdjustedPositionReport(Box<AdjustedPositionReport>),
    /// Allocation Instruction Alert
    #[serde(rename = "BM")]
    AllocationInstructionAlert(Box<AllocationInstructionAlert>),
    /// Execution Acknowledgement
    #[serde(rename = "BN")]
    ExecutioNAcknowledgement(Box<ExecutioNAcknowledgement>),
    /// Contrary Intention Report
    #[serde(rename = "BO")]
    ContraryIntentionReport(Box<ContraryIntentionReport>),
    /// Security Definition Update Report
    #[serde(rename = "BP")]
    SecurityDefinitionUpdateReport(Box<SecurityDefinitionUpdateReport>),
    /// Settlement Obligation Report
    #[serde(rename = "BQ")]
    SettlementObligationReport(Box<SettlementObligationReport>),
    /// Derivative Security List Update Report
    #[serde(rename = "BR")]
    DerivativeSecurityListUpdateReport(Box<DerivativeSecurityListUpdateReport>),
    /// Trading Session List Update Report
    #[serde(rename = "BS")]
    TradingSessionListUpdateReport(Box<TradingSessionListUpdateReport>),
    /// Market Definition Request
    #[serde(rename = "BT")]
    MarketDefinitionRequest(Box<MarketDefinitionRequest>),
    /// Market Definition
    #[serde(rename = "BU")]
    MarketDefinition(Box<MarketDefinition>),
    /// Market Definition Update Report
    #[serde(rename = "BV")]
    MarketDefinitionUpdateReport(Box<MarketDefinitionUpdateReport>),
    /// Application Message Request
    #[serde(rename = "BW")]
    ApplicationMessageRequest(Box<ApplicationMessageRequest>),
    /// Application Message Request Ack
    #[serde(rename = "BX")]
    ApplicationMessageRequestAck(Box<ApplicationMessageRequestAck>),
    /// Application Message Report
    #[serde(rename = "BY")]
    ApplicationMessageReport(Box<ApplicationMessageReport>),
    /// Order Mass Action Report
    #[serde(rename = "BZ")]
    OrderMassActionReport(Box<OrderMassActionReport>),
    /// Order Mass Action Request
    #[serde(rename = "CA")]
    OrderMassActionRequest(Box<OrderMassActionRequest>),
    /// User Notification
    #[serde(rename = "CB")]
    UserNotification(Box<UserNotification>),
    /// Stream Assignment Request
    #[serde(rename = "CC")]
    StreamAssignmentRequest(Box<StreamAssignmentRequest>),
    /// Stream Assignment Report
    #[serde(rename = "CD")]
    StreamAssignmentReport(Box<StreamAssignmentReport>),
    /// Stream Assignment Report ACK
    #[serde(rename = "CE")]
    StreamAssignmentReportACK(Box<StreamAssignmentReportACK>),
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
            Message::XMLMessage(m) => m.serialize(serializer),
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
            Message::RFQRequest(m) => m.serialize(serializer),
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
            Message::ExecutioNAcknowledgement(m) => m.serialize(serializer),
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
            Message::StreamAssignmentReportACK(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for Message {
    fn get_header(&self) -> &Header {
        match self {
            Message::Heartbeat(m) => m.get_header(),
            Message::TestRequest(m) => m.get_header(),
            Message::ResendRequest(m) => m.get_header(),
            Message::Reject(m) => m.get_header(),
            Message::SequenceReset(m) => m.get_header(),
            Message::Logout(m) => m.get_header(),
            Message::IndicationOfInterest(m) => m.get_header(),
            Message::Advertisement(m) => m.get_header(),
            Message::ExecutionReport(m) => m.get_header(),
            Message::OrderCancelReject(m) => m.get_header(),
            Message::Logon(m) => m.get_header(),
            Message::News(m) => m.get_header(),
            Message::Email(m) => m.get_header(),
            Message::NewOrderSingle(m) => m.get_header(),
            Message::NewOrderList(m) => m.get_header(),
            Message::OrderCancelRequest(m) => m.get_header(),
            Message::OrderCancelReplaceRequest(m) => m.get_header(),
            Message::OrderStatusRequest(m) => m.get_header(),
            Message::AllocationInstruction(m) => m.get_header(),
            Message::ListCancelRequest(m) => m.get_header(),
            Message::ListExecute(m) => m.get_header(),
            Message::ListStatusRequest(m) => m.get_header(),
            Message::ListStatus(m) => m.get_header(),
            Message::AllocationInstructionAck(m) => m.get_header(),
            Message::DontKnowTrade(m) => m.get_header(),
            Message::QuoteRequest(m) => m.get_header(),
            Message::Quote(m) => m.get_header(),
            Message::SettlementInstructions(m) => m.get_header(),
            Message::MarketDataRequest(m) => m.get_header(),
            Message::MarketDataSnapshotFullRefresh(m) => m.get_header(),
            Message::MarketDataIncrementalRefresh(m) => m.get_header(),
            Message::MarketDataRequestReject(m) => m.get_header(),
            Message::QuoteCancel(m) => m.get_header(),
            Message::QuoteStatusRequest(m) => m.get_header(),
            Message::MassQuoteAcknowledgement(m) => m.get_header(),
            Message::SecurityDefinitionRequest(m) => m.get_header(),
            Message::SecurityDefinition(m) => m.get_header(),
            Message::SecurityStatusRequest(m) => m.get_header(),
            Message::SecurityStatus(m) => m.get_header(),
            Message::TradingSessionStatusRequest(m) => m.get_header(),
            Message::TradingSessionStatus(m) => m.get_header(),
            Message::MassQuote(m) => m.get_header(),
            Message::BusinessMessageReject(m) => m.get_header(),
            Message::BidRequest(m) => m.get_header(),
            Message::BidResponse(m) => m.get_header(),
            Message::ListStrikePrice(m) => m.get_header(),
            Message::XMLMessage(m) => m.get_header(),
            Message::RegistrationInstructions(m) => m.get_header(),
            Message::RegistrationInstructionsResponse(m) => m.get_header(),
            Message::OrderMassCancelRequest(m) => m.get_header(),
            Message::OrderMassCancelReport(m) => m.get_header(),
            Message::NewOrderCross(m) => m.get_header(),
            Message::CrossOrderCancelReplaceRequest(m) => m.get_header(),
            Message::CrossOrderCancelRequest(m) => m.get_header(),
            Message::SecurityTypeRequest(m) => m.get_header(),
            Message::SecurityTypes(m) => m.get_header(),
            Message::SecurityListRequest(m) => m.get_header(),
            Message::SecurityList(m) => m.get_header(),
            Message::DerivativeSecurityListRequest(m) => m.get_header(),
            Message::DerivativeSecurityList(m) => m.get_header(),
            Message::NewOrderMultileg(m) => m.get_header(),
            Message::MultilegOrderCancelReplace(m) => m.get_header(),
            Message::TradeCaptureReportRequest(m) => m.get_header(),
            Message::TradeCaptureReport(m) => m.get_header(),
            Message::OrderMassStatusRequest(m) => m.get_header(),
            Message::QuoteRequestReject(m) => m.get_header(),
            Message::RFQRequest(m) => m.get_header(),
            Message::QuoteStatusReport(m) => m.get_header(),
            Message::QuoteResponse(m) => m.get_header(),
            Message::Confirmation(m) => m.get_header(),
            Message::PositionMaintenanceRequest(m) => m.get_header(),
            Message::PositionMaintenanceReport(m) => m.get_header(),
            Message::RequestForPositions(m) => m.get_header(),
            Message::RequestForPositionsAck(m) => m.get_header(),
            Message::PositionReport(m) => m.get_header(),
            Message::TradeCaptureReportRequestAck(m) => m.get_header(),
            Message::TradeCaptureReportAck(m) => m.get_header(),
            Message::AllocationReport(m) => m.get_header(),
            Message::AllocationReportAck(m) => m.get_header(),
            Message::ConfirmationAck(m) => m.get_header(),
            Message::SettlementInstructionRequest(m) => m.get_header(),
            Message::AssignmentReport(m) => m.get_header(),
            Message::CollateralRequest(m) => m.get_header(),
            Message::CollateralAssignment(m) => m.get_header(),
            Message::CollateralResponse(m) => m.get_header(),
            Message::CollateralReport(m) => m.get_header(),
            Message::CollateralInquiry(m) => m.get_header(),
            Message::NetworkCounterpartySystemStatusRequest(m) => m.get_header(),
            Message::NetworkCounterpartySystemStatusResponse(m) => m.get_header(),
            Message::UserRequest(m) => m.get_header(),
            Message::UserResponse(m) => m.get_header(),
            Message::CollateralInquiryAck(m) => m.get_header(),
            Message::ConfirmationRequest(m) => m.get_header(),
            Message::TradingSessionListRequest(m) => m.get_header(),
            Message::TradingSessionList(m) => m.get_header(),
            Message::SecurityListUpdateReport(m) => m.get_header(),
            Message::AdjustedPositionReport(m) => m.get_header(),
            Message::AllocationInstructionAlert(m) => m.get_header(),
            Message::ExecutioNAcknowledgement(m) => m.get_header(),
            Message::ContraryIntentionReport(m) => m.get_header(),
            Message::SecurityDefinitionUpdateReport(m) => m.get_header(),
            Message::SettlementObligationReport(m) => m.get_header(),
            Message::DerivativeSecurityListUpdateReport(m) => m.get_header(),
            Message::TradingSessionListUpdateReport(m) => m.get_header(),
            Message::MarketDefinitionRequest(m) => m.get_header(),
            Message::MarketDefinition(m) => m.get_header(),
            Message::MarketDefinitionUpdateReport(m) => m.get_header(),
            Message::ApplicationMessageRequest(m) => m.get_header(),
            Message::ApplicationMessageRequestAck(m) => m.get_header(),
            Message::ApplicationMessageReport(m) => m.get_header(),
            Message::OrderMassActionReport(m) => m.get_header(),
            Message::OrderMassActionRequest(m) => m.get_header(),
            Message::UserNotification(m) => m.get_header(),
            Message::StreamAssignmentRequest(m) => m.get_header(),
            Message::StreamAssignmentReport(m) => m.get_header(),
            Message::StreamAssignmentReportACK(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            Message::Heartbeat(m) => m.get_header_mut(),
            Message::TestRequest(m) => m.get_header_mut(),
            Message::ResendRequest(m) => m.get_header_mut(),
            Message::Reject(m) => m.get_header_mut(),
            Message::SequenceReset(m) => m.get_header_mut(),
            Message::Logout(m) => m.get_header_mut(),
            Message::IndicationOfInterest(m) => m.get_header_mut(),
            Message::Advertisement(m) => m.get_header_mut(),
            Message::ExecutionReport(m) => m.get_header_mut(),
            Message::OrderCancelReject(m) => m.get_header_mut(),
            Message::Logon(m) => m.get_header_mut(),
            Message::News(m) => m.get_header_mut(),
            Message::Email(m) => m.get_header_mut(),
            Message::NewOrderSingle(m) => m.get_header_mut(),
            Message::NewOrderList(m) => m.get_header_mut(),
            Message::OrderCancelRequest(m) => m.get_header_mut(),
            Message::OrderCancelReplaceRequest(m) => m.get_header_mut(),
            Message::OrderStatusRequest(m) => m.get_header_mut(),
            Message::AllocationInstruction(m) => m.get_header_mut(),
            Message::ListCancelRequest(m) => m.get_header_mut(),
            Message::ListExecute(m) => m.get_header_mut(),
            Message::ListStatusRequest(m) => m.get_header_mut(),
            Message::ListStatus(m) => m.get_header_mut(),
            Message::AllocationInstructionAck(m) => m.get_header_mut(),
            Message::DontKnowTrade(m) => m.get_header_mut(),
            Message::QuoteRequest(m) => m.get_header_mut(),
            Message::Quote(m) => m.get_header_mut(),
            Message::SettlementInstructions(m) => m.get_header_mut(),
            Message::MarketDataRequest(m) => m.get_header_mut(),
            Message::MarketDataSnapshotFullRefresh(m) => m.get_header_mut(),
            Message::MarketDataIncrementalRefresh(m) => m.get_header_mut(),
            Message::MarketDataRequestReject(m) => m.get_header_mut(),
            Message::QuoteCancel(m) => m.get_header_mut(),
            Message::QuoteStatusRequest(m) => m.get_header_mut(),
            Message::MassQuoteAcknowledgement(m) => m.get_header_mut(),
            Message::SecurityDefinitionRequest(m) => m.get_header_mut(),
            Message::SecurityDefinition(m) => m.get_header_mut(),
            Message::SecurityStatusRequest(m) => m.get_header_mut(),
            Message::SecurityStatus(m) => m.get_header_mut(),
            Message::TradingSessionStatusRequest(m) => m.get_header_mut(),
            Message::TradingSessionStatus(m) => m.get_header_mut(),
            Message::MassQuote(m) => m.get_header_mut(),
            Message::BusinessMessageReject(m) => m.get_header_mut(),
            Message::BidRequest(m) => m.get_header_mut(),
            Message::BidResponse(m) => m.get_header_mut(),
            Message::ListStrikePrice(m) => m.get_header_mut(),
            Message::XMLMessage(m) => m.get_header_mut(),
            Message::RegistrationInstructions(m) => m.get_header_mut(),
            Message::RegistrationInstructionsResponse(m) => m.get_header_mut(),
            Message::OrderMassCancelRequest(m) => m.get_header_mut(),
            Message::OrderMassCancelReport(m) => m.get_header_mut(),
            Message::NewOrderCross(m) => m.get_header_mut(),
            Message::CrossOrderCancelReplaceRequest(m) => m.get_header_mut(),
            Message::CrossOrderCancelRequest(m) => m.get_header_mut(),
            Message::SecurityTypeRequest(m) => m.get_header_mut(),
            Message::SecurityTypes(m) => m.get_header_mut(),
            Message::SecurityListRequest(m) => m.get_header_mut(),
            Message::SecurityList(m) => m.get_header_mut(),
            Message::DerivativeSecurityListRequest(m) => m.get_header_mut(),
            Message::DerivativeSecurityList(m) => m.get_header_mut(),
            Message::NewOrderMultileg(m) => m.get_header_mut(),
            Message::MultilegOrderCancelReplace(m) => m.get_header_mut(),
            Message::TradeCaptureReportRequest(m) => m.get_header_mut(),
            Message::TradeCaptureReport(m) => m.get_header_mut(),
            Message::OrderMassStatusRequest(m) => m.get_header_mut(),
            Message::QuoteRequestReject(m) => m.get_header_mut(),
            Message::RFQRequest(m) => m.get_header_mut(),
            Message::QuoteStatusReport(m) => m.get_header_mut(),
            Message::QuoteResponse(m) => m.get_header_mut(),
            Message::Confirmation(m) => m.get_header_mut(),
            Message::PositionMaintenanceRequest(m) => m.get_header_mut(),
            Message::PositionMaintenanceReport(m) => m.get_header_mut(),
            Message::RequestForPositions(m) => m.get_header_mut(),
            Message::RequestForPositionsAck(m) => m.get_header_mut(),
            Message::PositionReport(m) => m.get_header_mut(),
            Message::TradeCaptureReportRequestAck(m) => m.get_header_mut(),
            Message::TradeCaptureReportAck(m) => m.get_header_mut(),
            Message::AllocationReport(m) => m.get_header_mut(),
            Message::AllocationReportAck(m) => m.get_header_mut(),
            Message::ConfirmationAck(m) => m.get_header_mut(),
            Message::SettlementInstructionRequest(m) => m.get_header_mut(),
            Message::AssignmentReport(m) => m.get_header_mut(),
            Message::CollateralRequest(m) => m.get_header_mut(),
            Message::CollateralAssignment(m) => m.get_header_mut(),
            Message::CollateralResponse(m) => m.get_header_mut(),
            Message::CollateralReport(m) => m.get_header_mut(),
            Message::CollateralInquiry(m) => m.get_header_mut(),
            Message::NetworkCounterpartySystemStatusRequest(m) => m.get_header_mut(),
            Message::NetworkCounterpartySystemStatusResponse(m) => m.get_header_mut(),
            Message::UserRequest(m) => m.get_header_mut(),
            Message::UserResponse(m) => m.get_header_mut(),
            Message::CollateralInquiryAck(m) => m.get_header_mut(),
            Message::ConfirmationRequest(m) => m.get_header_mut(),
            Message::TradingSessionListRequest(m) => m.get_header_mut(),
            Message::TradingSessionList(m) => m.get_header_mut(),
            Message::SecurityListUpdateReport(m) => m.get_header_mut(),
            Message::AdjustedPositionReport(m) => m.get_header_mut(),
            Message::AllocationInstructionAlert(m) => m.get_header_mut(),
            Message::ExecutioNAcknowledgement(m) => m.get_header_mut(),
            Message::ContraryIntentionReport(m) => m.get_header_mut(),
            Message::SecurityDefinitionUpdateReport(m) => m.get_header_mut(),
            Message::SettlementObligationReport(m) => m.get_header_mut(),
            Message::DerivativeSecurityListUpdateReport(m) => m.get_header_mut(),
            Message::TradingSessionListUpdateReport(m) => m.get_header_mut(),
            Message::MarketDefinitionRequest(m) => m.get_header_mut(),
            Message::MarketDefinition(m) => m.get_header_mut(),
            Message::MarketDefinitionUpdateReport(m) => m.get_header_mut(),
            Message::ApplicationMessageRequest(m) => m.get_header_mut(),
            Message::ApplicationMessageRequestAck(m) => m.get_header_mut(),
            Message::ApplicationMessageReport(m) => m.get_header_mut(),
            Message::OrderMassActionReport(m) => m.get_header_mut(),
            Message::OrderMassActionRequest(m) => m.get_header_mut(),
            Message::UserNotification(m) => m.get_header_mut(),
            Message::StreamAssignmentRequest(m) => m.get_header_mut(),
            Message::StreamAssignmentReport(m) => m.get_header_mut(),
            Message::StreamAssignmentReportACK(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fixt11::messages::Heartbeat {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl HasHeader for fixt11::messages::TestRequest {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl HasHeader for fixt11::messages::ResendRequest {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl HasHeader for fixt11::messages::Reject {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl HasHeader for fixt11::messages::SequenceReset {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl HasHeader for fixt11::messages::Logout {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl HasHeader for fixt11::messages::Logon {
    fn get_header(&self) -> &Header {
        &self.header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum IndicationOfInterest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::indication_of_interest::Indication>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::indication_of_interest::Indication>),
}

impl Serialize for IndicationOfInterest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            IndicationOfInterest::FIX50(m) => m.serialize(serializer),
            IndicationOfInterest::FIX50SP1(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for IndicationOfInterest {
    fn get_header(&self) -> &Header {
        match self {
            IndicationOfInterest::FIX50(m) => m.get_header(),
            IndicationOfInterest::FIX50SP1(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            IndicationOfInterest::FIX50(m) => m.get_header_mut(),
            IndicationOfInterest::FIX50SP1(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::indication_of_interest::Indication {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::indication_of_interest::Indication {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum Advertisement {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::advertisement::Advertisement>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::advertisement::Advertisement>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::advertisement::Advertisement>),
}

impl Serialize for Advertisement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Advertisement::FIX50(m) => m.serialize(serializer),
            Advertisement::FIX50SP1(m) => m.serialize(serializer),
            Advertisement::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for Advertisement {
    fn get_header(&self) -> &Header {
        match self {
            Advertisement::FIX50(m) => m.get_header(),
            Advertisement::FIX50SP1(m) => m.get_header(),
            Advertisement::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            Advertisement::FIX50(m) => m.get_header_mut(),
            Advertisement::FIX50SP1(m) => m.get_header_mut(),
            Advertisement::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::advertisement::Advertisement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::advertisement::Advertisement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::advertisement::Advertisement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ExecutionReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::execution_report::Execution>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::execution_report::Execution>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::execution_report::ExecutionReport>),
}

impl Serialize for ExecutionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ExecutionReport::FIX50(m) => m.serialize(serializer),
            ExecutionReport::FIX50SP1(m) => m.serialize(serializer),
            ExecutionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ExecutionReport {
    fn get_header(&self) -> &Header {
        match self {
            ExecutionReport::FIX50(m) => m.get_header(),
            ExecutionReport::FIX50SP1(m) => m.get_header(),
            ExecutionReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ExecutionReport::FIX50(m) => m.get_header_mut(),
            ExecutionReport::FIX50SP1(m) => m.get_header_mut(),
            ExecutionReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::execution_report::Execution {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::execution_report::Execution {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::execution_report::ExecutionReport {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderCancelReject {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_cancel_reject::Order>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_cancel_reject::Order>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_cancel_reject::Order>),
}

impl Serialize for OrderCancelReject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrderCancelReject::FIX50(m) => m.serialize(serializer),
            OrderCancelReject::FIX50SP1(m) => m.serialize(serializer),
            OrderCancelReject::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for OrderCancelReject {
    fn get_header(&self) -> &Header {
        match self {
            OrderCancelReject::FIX50(m) => m.get_header(),
            OrderCancelReject::FIX50SP1(m) => m.get_header(),
            OrderCancelReject::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            OrderCancelReject::FIX50(m) => m.get_header_mut(),
            OrderCancelReject::FIX50SP1(m) => m.get_header_mut(),
            OrderCancelReject::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::order_cancel_reject::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::order_cancel_reject::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::order_cancel_reject::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum News {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::news::News>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::news::News>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::news::News>),
}

impl Serialize for News {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            News::FIX50(m) => m.serialize(serializer),
            News::FIX50SP1(m) => m.serialize(serializer),
            News::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for News {
    fn get_header(&self) -> &Header {
        match self {
            News::FIX50(m) => m.get_header(),
            News::FIX50SP1(m) => m.get_header(),
            News::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            News::FIX50(m) => m.get_header_mut(),
            News::FIX50SP1(m) => m.get_header_mut(),
            News::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::news::News {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::news::News {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::news::News {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum Email {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::email::Email>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::email::Email>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::email::Email>),
}

impl Serialize for Email {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Email::FIX50(m) => m.serialize(serializer),
            Email::FIX50SP1(m) => m.serialize(serializer),
            Email::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for Email {
    fn get_header(&self) -> &Header {
        match self {
            Email::FIX50(m) => m.get_header(),
            Email::FIX50SP1(m) => m.get_header(),
            Email::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            Email::FIX50(m) => m.get_header_mut(),
            Email::FIX50SP1(m) => m.get_header_mut(),
            Email::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::email::Email {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::email::Email {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::email::Email {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NewOrderSingle {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::new_order_single::New>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::new_order_single::New>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::new_order_single::NewOrderSingle>),
}

impl Serialize for NewOrderSingle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NewOrderSingle::FIX50(m) => m.serialize(serializer),
            NewOrderSingle::FIX50SP1(m) => m.serialize(serializer),
            NewOrderSingle::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for NewOrderSingle {
    fn get_header(&self) -> &Header {
        match self {
            NewOrderSingle::FIX50(m) => m.get_header(),
            NewOrderSingle::FIX50SP1(m) => m.get_header(),
            NewOrderSingle::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            NewOrderSingle::FIX50(m) => m.get_header_mut(),
            NewOrderSingle::FIX50SP1(m) => m.get_header_mut(),
            NewOrderSingle::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::new_order_single::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::new_order_single::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::new_order_single::NewOrderSingle {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NewOrderList {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::new_order_list::New>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::new_order_list::New>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::new_order_list::New>),
}

impl Serialize for NewOrderList {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NewOrderList::FIX50(m) => m.serialize(serializer),
            NewOrderList::FIX50SP1(m) => m.serialize(serializer),
            NewOrderList::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for NewOrderList {
    fn get_header(&self) -> &Header {
        match self {
            NewOrderList::FIX50(m) => m.get_header(),
            NewOrderList::FIX50SP1(m) => m.get_header(),
            NewOrderList::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            NewOrderList::FIX50(m) => m.get_header_mut(),
            NewOrderList::FIX50SP1(m) => m.get_header_mut(),
            NewOrderList::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::new_order_list::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::new_order_list::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::new_order_list::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderCancelRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_cancel_request::Order>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_cancel_request::Order>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_cancel_request::Order>),
}

impl Serialize for OrderCancelRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrderCancelRequest::FIX50(m) => m.serialize(serializer),
            OrderCancelRequest::FIX50SP1(m) => m.serialize(serializer),
            OrderCancelRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for OrderCancelRequest {
    fn get_header(&self) -> &Header {
        match self {
            OrderCancelRequest::FIX50(m) => m.get_header(),
            OrderCancelRequest::FIX50SP1(m) => m.get_header(),
            OrderCancelRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            OrderCancelRequest::FIX50(m) => m.get_header_mut(),
            OrderCancelRequest::FIX50SP1(m) => m.get_header_mut(),
            OrderCancelRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::order_cancel_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::order_cancel_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::order_cancel_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderCancelReplaceRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_cancel_replace_request::Order>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_cancel_replace_request::Order>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_cancel_replace_request::Order>),
}

impl Serialize for OrderCancelReplaceRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrderCancelReplaceRequest::FIX50(m) => m.serialize(serializer),
            OrderCancelReplaceRequest::FIX50SP1(m) => m.serialize(serializer),
            OrderCancelReplaceRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for OrderCancelReplaceRequest {
    fn get_header(&self) -> &Header {
        match self {
            OrderCancelReplaceRequest::FIX50(m) => m.get_header(),
            OrderCancelReplaceRequest::FIX50SP1(m) => m.get_header(),
            OrderCancelReplaceRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            OrderCancelReplaceRequest::FIX50(m) => m.get_header_mut(),
            OrderCancelReplaceRequest::FIX50SP1(m) => m.get_header_mut(),
            OrderCancelReplaceRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::order_cancel_replace_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::order_cancel_replace_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::order_cancel_replace_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderStatusRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_status_request::Order>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_status_request::Order>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_status_request::Order>),
}

impl Serialize for OrderStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrderStatusRequest::FIX50(m) => m.serialize(serializer),
            OrderStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            OrderStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for OrderStatusRequest {
    fn get_header(&self) -> &Header {
        match self {
            OrderStatusRequest::FIX50(m) => m.get_header(),
            OrderStatusRequest::FIX50SP1(m) => m.get_header(),
            OrderStatusRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            OrderStatusRequest::FIX50(m) => m.get_header_mut(),
            OrderStatusRequest::FIX50SP1(m) => m.get_header_mut(),
            OrderStatusRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::order_status_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::order_status_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::order_status_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationInstruction {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_instruction::Allocation>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_instruction::Allocation>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_instruction::AllocationInstruction>),
}

impl Serialize for AllocationInstruction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AllocationInstruction::FIX50(m) => m.serialize(serializer),
            AllocationInstruction::FIX50SP1(m) => m.serialize(serializer),
            AllocationInstruction::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for AllocationInstruction {
    fn get_header(&self) -> &Header {
        match self {
            AllocationInstruction::FIX50(m) => m.get_header(),
            AllocationInstruction::FIX50SP1(m) => m.get_header(),
            AllocationInstruction::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            AllocationInstruction::FIX50(m) => m.get_header_mut(),
            AllocationInstruction::FIX50SP1(m) => m.get_header_mut(),
            AllocationInstruction::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::allocation_instruction::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::allocation_instruction::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::allocation_instruction::AllocationInstruction {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListCancelRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_cancel_request::List>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_cancel_request::List>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_cancel_request::List>),
}

impl Serialize for ListCancelRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ListCancelRequest::FIX50(m) => m.serialize(serializer),
            ListCancelRequest::FIX50SP1(m) => m.serialize(serializer),
            ListCancelRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ListCancelRequest {
    fn get_header(&self) -> &Header {
        match self {
            ListCancelRequest::FIX50(m) => m.get_header(),
            ListCancelRequest::FIX50SP1(m) => m.get_header(),
            ListCancelRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ListCancelRequest::FIX50(m) => m.get_header_mut(),
            ListCancelRequest::FIX50SP1(m) => m.get_header_mut(),
            ListCancelRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::list_cancel_request::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::list_cancel_request::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::list_cancel_request::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListExecute {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_execute::List>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_execute::List>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_execute::List>),
}

impl Serialize for ListExecute {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ListExecute::FIX50(m) => m.serialize(serializer),
            ListExecute::FIX50SP1(m) => m.serialize(serializer),
            ListExecute::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ListExecute {
    fn get_header(&self) -> &Header {
        match self {
            ListExecute::FIX50(m) => m.get_header(),
            ListExecute::FIX50SP1(m) => m.get_header(),
            ListExecute::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ListExecute::FIX50(m) => m.get_header_mut(),
            ListExecute::FIX50SP1(m) => m.get_header_mut(),
            ListExecute::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::list_execute::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::list_execute::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::list_execute::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListStatusRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_status_request::List>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_status_request::List>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_status_request::List>),
}

impl Serialize for ListStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ListStatusRequest::FIX50(m) => m.serialize(serializer),
            ListStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            ListStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ListStatusRequest {
    fn get_header(&self) -> &Header {
        match self {
            ListStatusRequest::FIX50(m) => m.get_header(),
            ListStatusRequest::FIX50SP1(m) => m.get_header(),
            ListStatusRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ListStatusRequest::FIX50(m) => m.get_header_mut(),
            ListStatusRequest::FIX50SP1(m) => m.get_header_mut(),
            ListStatusRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::list_status_request::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::list_status_request::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::list_status_request::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListStatus {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_status::List>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_status::List>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_status::List>),
}

impl Serialize for ListStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ListStatus::FIX50(m) => m.serialize(serializer),
            ListStatus::FIX50SP1(m) => m.serialize(serializer),
            ListStatus::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ListStatus {
    fn get_header(&self) -> &Header {
        match self {
            ListStatus::FIX50(m) => m.get_header(),
            ListStatus::FIX50SP1(m) => m.get_header(),
            ListStatus::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ListStatus::FIX50(m) => m.get_header_mut(),
            ListStatus::FIX50SP1(m) => m.get_header_mut(),
            ListStatus::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::list_status::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::list_status::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::list_status::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationInstructionAck {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_instruction_ack::Allocation>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_instruction_ack::Allocation>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_instruction_ack::Allocation>),
}

impl Serialize for AllocationInstructionAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AllocationInstructionAck::FIX50(m) => m.serialize(serializer),
            AllocationInstructionAck::FIX50SP1(m) => m.serialize(serializer),
            AllocationInstructionAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for AllocationInstructionAck {
    fn get_header(&self) -> &Header {
        match self {
            AllocationInstructionAck::FIX50(m) => m.get_header(),
            AllocationInstructionAck::FIX50SP1(m) => m.get_header(),
            AllocationInstructionAck::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            AllocationInstructionAck::FIX50(m) => m.get_header_mut(),
            AllocationInstructionAck::FIX50SP1(m) => m.get_header_mut(),
            AllocationInstructionAck::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::allocation_instruction_ack::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::allocation_instruction_ack::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::allocation_instruction_ack::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum DontKnowTrade {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::dont_know_trade::Dont>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::dont_know_trade::Dont>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::dont_know_trade::Dont>),
}

impl Serialize for DontKnowTrade {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DontKnowTrade::FIX50(m) => m.serialize(serializer),
            DontKnowTrade::FIX50SP1(m) => m.serialize(serializer),
            DontKnowTrade::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for DontKnowTrade {
    fn get_header(&self) -> &Header {
        match self {
            DontKnowTrade::FIX50(m) => m.get_header(),
            DontKnowTrade::FIX50SP1(m) => m.get_header(),
            DontKnowTrade::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            DontKnowTrade::FIX50(m) => m.get_header_mut(),
            DontKnowTrade::FIX50SP1(m) => m.get_header_mut(),
            DontKnowTrade::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::dont_know_trade::Dont {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::dont_know_trade::Dont {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::dont_know_trade::Dont {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_request::Quote>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_request::Quote>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_request::Quote>),
}

impl Serialize for QuoteRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            QuoteRequest::FIX50(m) => m.serialize(serializer),
            QuoteRequest::FIX50SP1(m) => m.serialize(serializer),
            QuoteRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for QuoteRequest {
    fn get_header(&self) -> &Header {
        match self {
            QuoteRequest::FIX50(m) => m.get_header(),
            QuoteRequest::FIX50SP1(m) => m.get_header(),
            QuoteRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            QuoteRequest::FIX50(m) => m.get_header_mut(),
            QuoteRequest::FIX50SP1(m) => m.get_header_mut(),
            QuoteRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::quote_request::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::quote_request::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::quote_request::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum Quote {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote::Quote>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote::Quote>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote::Quote>),
}

impl Serialize for Quote {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Quote::FIX50(m) => m.serialize(serializer),
            Quote::FIX50SP1(m) => m.serialize(serializer),
            Quote::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for Quote {
    fn get_header(&self) -> &Header {
        match self {
            Quote::FIX50(m) => m.get_header(),
            Quote::FIX50SP1(m) => m.get_header(),
            Quote::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            Quote::FIX50(m) => m.get_header_mut(),
            Quote::FIX50SP1(m) => m.get_header_mut(),
            Quote::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::quote::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::quote::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::quote::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SettlementInstructions {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::settlement_instructions::Settlement>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::settlement_instructions::Settlement>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::settlement_instructions::Settlement>),
}

impl Serialize for SettlementInstructions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SettlementInstructions::FIX50(m) => m.serialize(serializer),
            SettlementInstructions::FIX50SP1(m) => m.serialize(serializer),
            SettlementInstructions::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SettlementInstructions {
    fn get_header(&self) -> &Header {
        match self {
            SettlementInstructions::FIX50(m) => m.get_header(),
            SettlementInstructions::FIX50SP1(m) => m.get_header(),
            SettlementInstructions::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SettlementInstructions::FIX50(m) => m.get_header_mut(),
            SettlementInstructions::FIX50SP1(m) => m.get_header_mut(),
            SettlementInstructions::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::settlement_instructions::Settlement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::settlement_instructions::Settlement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::settlement_instructions::Settlement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDataRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::market_data_request::Market>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_data_request::Market>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_data_request::Market>),
}

impl Serialize for MarketDataRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MarketDataRequest::FIX50(m) => m.serialize(serializer),
            MarketDataRequest::FIX50SP1(m) => m.serialize(serializer),
            MarketDataRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MarketDataRequest {
    fn get_header(&self) -> &Header {
        match self {
            MarketDataRequest::FIX50(m) => m.get_header(),
            MarketDataRequest::FIX50SP1(m) => m.get_header(),
            MarketDataRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MarketDataRequest::FIX50(m) => m.get_header_mut(),
            MarketDataRequest::FIX50SP1(m) => m.get_header_mut(),
            MarketDataRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::market_data_request::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::market_data_request::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::market_data_request::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDataSnapshotFullRefresh {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::market_data_snapshot_full_refresh::Market>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_data_snapshot_full_refresh::Market>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_data_snapshot_full_refresh::Market>),
}

impl Serialize for MarketDataSnapshotFullRefresh {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MarketDataSnapshotFullRefresh::FIX50(m) => m.serialize(serializer),
            MarketDataSnapshotFullRefresh::FIX50SP1(m) => m.serialize(serializer),
            MarketDataSnapshotFullRefresh::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MarketDataSnapshotFullRefresh {
    fn get_header(&self) -> &Header {
        match self {
            MarketDataSnapshotFullRefresh::FIX50(m) => m.get_header(),
            MarketDataSnapshotFullRefresh::FIX50SP1(m) => m.get_header(),
            MarketDataSnapshotFullRefresh::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MarketDataSnapshotFullRefresh::FIX50(m) => m.get_header_mut(),
            MarketDataSnapshotFullRefresh::FIX50SP1(m) => m.get_header_mut(),
            MarketDataSnapshotFullRefresh::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::market_data_snapshot_full_refresh::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::market_data_snapshot_full_refresh::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::market_data_snapshot_full_refresh::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDataIncrementalRefresh {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::market_data_incremental_refresh::Market>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_data_incremental_refresh::Market>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_data_incremental_refresh::Market>),
}

impl Serialize for MarketDataIncrementalRefresh {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MarketDataIncrementalRefresh::FIX50(m) => m.serialize(serializer),
            MarketDataIncrementalRefresh::FIX50SP1(m) => m.serialize(serializer),
            MarketDataIncrementalRefresh::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MarketDataIncrementalRefresh {
    fn get_header(&self) -> &Header {
        match self {
            MarketDataIncrementalRefresh::FIX50(m) => m.get_header(),
            MarketDataIncrementalRefresh::FIX50SP1(m) => m.get_header(),
            MarketDataIncrementalRefresh::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MarketDataIncrementalRefresh::FIX50(m) => m.get_header_mut(),
            MarketDataIncrementalRefresh::FIX50SP1(m) => m.get_header_mut(),
            MarketDataIncrementalRefresh::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::market_data_incremental_refresh::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::market_data_incremental_refresh::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::market_data_incremental_refresh::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDataRequestReject {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::market_data_request_reject::Market>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_data_request_reject::Market>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_data_request_reject::Market>),
}

impl Serialize for MarketDataRequestReject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MarketDataRequestReject::FIX50(m) => m.serialize(serializer),
            MarketDataRequestReject::FIX50SP1(m) => m.serialize(serializer),
            MarketDataRequestReject::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MarketDataRequestReject {
    fn get_header(&self) -> &Header {
        match self {
            MarketDataRequestReject::FIX50(m) => m.get_header(),
            MarketDataRequestReject::FIX50SP1(m) => m.get_header(),
            MarketDataRequestReject::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MarketDataRequestReject::FIX50(m) => m.get_header_mut(),
            MarketDataRequestReject::FIX50SP1(m) => m.get_header_mut(),
            MarketDataRequestReject::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::market_data_request_reject::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::market_data_request_reject::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::market_data_request_reject::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteCancel {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_cancel::Quote>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_cancel::Quote>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_cancel::Quote>),
}

impl Serialize for QuoteCancel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            QuoteCancel::FIX50(m) => m.serialize(serializer),
            QuoteCancel::FIX50SP1(m) => m.serialize(serializer),
            QuoteCancel::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for QuoteCancel {
    fn get_header(&self) -> &Header {
        match self {
            QuoteCancel::FIX50(m) => m.get_header(),
            QuoteCancel::FIX50SP1(m) => m.get_header(),
            QuoteCancel::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            QuoteCancel::FIX50(m) => m.get_header_mut(),
            QuoteCancel::FIX50SP1(m) => m.get_header_mut(),
            QuoteCancel::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::quote_cancel::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::quote_cancel::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::quote_cancel::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteStatusRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_status_request::Quote>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_status_request::Quote>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_status_request::Quote>),
}

impl Serialize for QuoteStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            QuoteStatusRequest::FIX50(m) => m.serialize(serializer),
            QuoteStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            QuoteStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for QuoteStatusRequest {
    fn get_header(&self) -> &Header {
        match self {
            QuoteStatusRequest::FIX50(m) => m.get_header(),
            QuoteStatusRequest::FIX50SP1(m) => m.get_header(),
            QuoteStatusRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            QuoteStatusRequest::FIX50(m) => m.get_header_mut(),
            QuoteStatusRequest::FIX50SP1(m) => m.get_header_mut(),
            QuoteStatusRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::quote_status_request::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::quote_status_request::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::quote_status_request::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MassQuoteAcknowledgement {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::mass_quote_acknowledgement::Mass>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::mass_quote_acknowledgement::Mass>),
}

impl Serialize for MassQuoteAcknowledgement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MassQuoteAcknowledgement::FIX50(m) => m.serialize(serializer),
            MassQuoteAcknowledgement::FIX50SP1(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MassQuoteAcknowledgement {
    fn get_header(&self) -> &Header {
        match self {
            MassQuoteAcknowledgement::FIX50(m) => m.get_header(),
            MassQuoteAcknowledgement::FIX50SP1(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MassQuoteAcknowledgement::FIX50(m) => m.get_header_mut(),
            MassQuoteAcknowledgement::FIX50SP1(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::mass_quote_acknowledgement::Mass {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::mass_quote_acknowledgement::Mass {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityDefinitionRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_definition_request::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_definition_request::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_definition_request::Security>),
}

impl Serialize for SecurityDefinitionRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityDefinitionRequest::FIX50(m) => m.serialize(serializer),
            SecurityDefinitionRequest::FIX50SP1(m) => m.serialize(serializer),
            SecurityDefinitionRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityDefinitionRequest {
    fn get_header(&self) -> &Header {
        match self {
            SecurityDefinitionRequest::FIX50(m) => m.get_header(),
            SecurityDefinitionRequest::FIX50SP1(m) => m.get_header(),
            SecurityDefinitionRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityDefinitionRequest::FIX50(m) => m.get_header_mut(),
            SecurityDefinitionRequest::FIX50SP1(m) => m.get_header_mut(),
            SecurityDefinitionRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_definition_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_definition_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_definition_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityDefinition {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_definition::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_definition::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_definition::Security>),
}

impl Serialize for SecurityDefinition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityDefinition::FIX50(m) => m.serialize(serializer),
            SecurityDefinition::FIX50SP1(m) => m.serialize(serializer),
            SecurityDefinition::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityDefinition {
    fn get_header(&self) -> &Header {
        match self {
            SecurityDefinition::FIX50(m) => m.get_header(),
            SecurityDefinition::FIX50SP1(m) => m.get_header(),
            SecurityDefinition::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityDefinition::FIX50(m) => m.get_header_mut(),
            SecurityDefinition::FIX50SP1(m) => m.get_header_mut(),
            SecurityDefinition::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_definition::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_definition::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_definition::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityStatusRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_status_request::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_status_request::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_status_request::Security>),
}

impl Serialize for SecurityStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityStatusRequest::FIX50(m) => m.serialize(serializer),
            SecurityStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            SecurityStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityStatusRequest {
    fn get_header(&self) -> &Header {
        match self {
            SecurityStatusRequest::FIX50(m) => m.get_header(),
            SecurityStatusRequest::FIX50SP1(m) => m.get_header(),
            SecurityStatusRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityStatusRequest::FIX50(m) => m.get_header_mut(),
            SecurityStatusRequest::FIX50SP1(m) => m.get_header_mut(),
            SecurityStatusRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_status_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_status_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_status_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityStatus {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_status::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_status::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_status::Security>),
}

impl Serialize for SecurityStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityStatus::FIX50(m) => m.serialize(serializer),
            SecurityStatus::FIX50SP1(m) => m.serialize(serializer),
            SecurityStatus::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityStatus {
    fn get_header(&self) -> &Header {
        match self {
            SecurityStatus::FIX50(m) => m.get_header(),
            SecurityStatus::FIX50SP1(m) => m.get_header(),
            SecurityStatus::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityStatus::FIX50(m) => m.get_header_mut(),
            SecurityStatus::FIX50SP1(m) => m.get_header_mut(),
            SecurityStatus::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_status::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_status::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_status::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionStatusRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trading_session_status_request::Trading>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_status_request::Trading>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_status_request::Trading>),
}

impl Serialize for TradingSessionStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TradingSessionStatusRequest::FIX50(m) => m.serialize(serializer),
            TradingSessionStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            TradingSessionStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for TradingSessionStatusRequest {
    fn get_header(&self) -> &Header {
        match self {
            TradingSessionStatusRequest::FIX50(m) => m.get_header(),
            TradingSessionStatusRequest::FIX50SP1(m) => m.get_header(),
            TradingSessionStatusRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            TradingSessionStatusRequest::FIX50(m) => m.get_header_mut(),
            TradingSessionStatusRequest::FIX50SP1(m) => m.get_header_mut(),
            TradingSessionStatusRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::trading_session_status_request::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::trading_session_status_request::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::trading_session_status_request::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionStatus {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trading_session_status::Trading>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_status::Trading>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_status::Trading>),
}

impl Serialize for TradingSessionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TradingSessionStatus::FIX50(m) => m.serialize(serializer),
            TradingSessionStatus::FIX50SP1(m) => m.serialize(serializer),
            TradingSessionStatus::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for TradingSessionStatus {
    fn get_header(&self) -> &Header {
        match self {
            TradingSessionStatus::FIX50(m) => m.get_header(),
            TradingSessionStatus::FIX50SP1(m) => m.get_header(),
            TradingSessionStatus::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            TradingSessionStatus::FIX50(m) => m.get_header_mut(),
            TradingSessionStatus::FIX50SP1(m) => m.get_header_mut(),
            TradingSessionStatus::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::trading_session_status::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::trading_session_status::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::trading_session_status::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MassQuote {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::mass_quote::Mass>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::mass_quote::Mass>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::mass_quote::Mass>),
}

impl Serialize for MassQuote {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MassQuote::FIX50(m) => m.serialize(serializer),
            MassQuote::FIX50SP1(m) => m.serialize(serializer),
            MassQuote::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MassQuote {
    fn get_header(&self) -> &Header {
        match self {
            MassQuote::FIX50(m) => m.get_header(),
            MassQuote::FIX50SP1(m) => m.get_header(),
            MassQuote::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MassQuote::FIX50(m) => m.get_header_mut(),
            MassQuote::FIX50SP1(m) => m.get_header_mut(),
            MassQuote::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::mass_quote::Mass {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::mass_quote::Mass {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::mass_quote::Mass {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum BusinessMessageReject {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::business_message_reject::Business>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::business_message_reject::Business>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::business_message_reject::Business>),
}

impl Serialize for BusinessMessageReject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            BusinessMessageReject::FIX50(m) => m.serialize(serializer),
            BusinessMessageReject::FIX50SP1(m) => m.serialize(serializer),
            BusinessMessageReject::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for BusinessMessageReject {
    fn get_header(&self) -> &Header {
        match self {
            BusinessMessageReject::FIX50(m) => m.get_header(),
            BusinessMessageReject::FIX50SP1(m) => m.get_header(),
            BusinessMessageReject::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            BusinessMessageReject::FIX50(m) => m.get_header_mut(),
            BusinessMessageReject::FIX50SP1(m) => m.get_header_mut(),
            BusinessMessageReject::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::business_message_reject::Business {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::business_message_reject::Business {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::business_message_reject::Business {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum BidRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::bid_request::Bid>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::bid_request::Bid>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::bid_request::Bid>),
}

impl Serialize for BidRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            BidRequest::FIX50(m) => m.serialize(serializer),
            BidRequest::FIX50SP1(m) => m.serialize(serializer),
            BidRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for BidRequest {
    fn get_header(&self) -> &Header {
        match self {
            BidRequest::FIX50(m) => m.get_header(),
            BidRequest::FIX50SP1(m) => m.get_header(),
            BidRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            BidRequest::FIX50(m) => m.get_header_mut(),
            BidRequest::FIX50SP1(m) => m.get_header_mut(),
            BidRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::bid_request::Bid {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::bid_request::Bid {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::bid_request::Bid {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum BidResponse {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::bid_response::Bid>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::bid_response::Bid>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::bid_response::Bid>),
}

impl Serialize for BidResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            BidResponse::FIX50(m) => m.serialize(serializer),
            BidResponse::FIX50SP1(m) => m.serialize(serializer),
            BidResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for BidResponse {
    fn get_header(&self) -> &Header {
        match self {
            BidResponse::FIX50(m) => m.get_header(),
            BidResponse::FIX50SP1(m) => m.get_header(),
            BidResponse::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            BidResponse::FIX50(m) => m.get_header_mut(),
            BidResponse::FIX50SP1(m) => m.get_header_mut(),
            BidResponse::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::bid_response::Bid {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::bid_response::Bid {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::bid_response::Bid {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ListStrikePrice {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::list_strike_price::List>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::list_strike_price::List>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::list_strike_price::List>),
}

impl Serialize for ListStrikePrice {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ListStrikePrice::FIX50(m) => m.serialize(serializer),
            ListStrikePrice::FIX50SP1(m) => m.serialize(serializer),
            ListStrikePrice::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ListStrikePrice {
    fn get_header(&self) -> &Header {
        match self {
            ListStrikePrice::FIX50(m) => m.get_header(),
            ListStrikePrice::FIX50SP1(m) => m.get_header(),
            ListStrikePrice::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ListStrikePrice::FIX50(m) => m.get_header_mut(),
            ListStrikePrice::FIX50SP1(m) => m.get_header_mut(),
            ListStrikePrice::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::list_strike_price::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::list_strike_price::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::list_strike_price::List {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum XMLMessage {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::xml_message::XML>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::xml_message::XML>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::xml_message::XML>),
}

impl Serialize for XMLMessage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            XMLMessage::FIX50(m) => m.serialize(serializer),
            XMLMessage::FIX50SP1(m) => m.serialize(serializer),
            XMLMessage::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for XMLMessage {
    fn get_header(&self) -> &Header {
        match self {
            XMLMessage::FIX50(m) => m.get_header(),
            XMLMessage::FIX50SP1(m) => m.get_header(),
            XMLMessage::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            XMLMessage::FIX50(m) => m.get_header_mut(),
            XMLMessage::FIX50SP1(m) => m.get_header_mut(),
            XMLMessage::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::xml_message::XML {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::xml_message::XML {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::xml_message::XML {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RegistrationInstructions {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::registration_instructions::Registration>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::registration_instructions::Registration>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::registration_instructions::Registration>),
}

impl Serialize for RegistrationInstructions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RegistrationInstructions::FIX50(m) => m.serialize(serializer),
            RegistrationInstructions::FIX50SP1(m) => m.serialize(serializer),
            RegistrationInstructions::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for RegistrationInstructions {
    fn get_header(&self) -> &Header {
        match self {
            RegistrationInstructions::FIX50(m) => m.get_header(),
            RegistrationInstructions::FIX50SP1(m) => m.get_header(),
            RegistrationInstructions::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            RegistrationInstructions::FIX50(m) => m.get_header_mut(),
            RegistrationInstructions::FIX50SP1(m) => m.get_header_mut(),
            RegistrationInstructions::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::registration_instructions::Registration {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::registration_instructions::Registration {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::registration_instructions::Registration {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RegistrationInstructionsResponse {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::registration_instructions_response::Registration>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::registration_instructions_response::Registration>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::registration_instructions_response::Registration>),
}

impl Serialize for RegistrationInstructionsResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RegistrationInstructionsResponse::FIX50(m) => m.serialize(serializer),
            RegistrationInstructionsResponse::FIX50SP1(m) => m.serialize(serializer),
            RegistrationInstructionsResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for RegistrationInstructionsResponse {
    fn get_header(&self) -> &Header {
        match self {
            RegistrationInstructionsResponse::FIX50(m) => m.get_header(),
            RegistrationInstructionsResponse::FIX50SP1(m) => m.get_header(),
            RegistrationInstructionsResponse::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            RegistrationInstructionsResponse::FIX50(m) => m.get_header_mut(),
            RegistrationInstructionsResponse::FIX50SP1(m) => m.get_header_mut(),
            RegistrationInstructionsResponse::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::registration_instructions_response::Registration {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::registration_instructions_response::Registration {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::registration_instructions_response::Registration {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassCancelRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_mass_cancel_request::Order>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_cancel_request::Order>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_cancel_request::Order>),
}

impl Serialize for OrderMassCancelRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrderMassCancelRequest::FIX50(m) => m.serialize(serializer),
            OrderMassCancelRequest::FIX50SP1(m) => m.serialize(serializer),
            OrderMassCancelRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for OrderMassCancelRequest {
    fn get_header(&self) -> &Header {
        match self {
            OrderMassCancelRequest::FIX50(m) => m.get_header(),
            OrderMassCancelRequest::FIX50SP1(m) => m.get_header(),
            OrderMassCancelRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            OrderMassCancelRequest::FIX50(m) => m.get_header_mut(),
            OrderMassCancelRequest::FIX50SP1(m) => m.get_header_mut(),
            OrderMassCancelRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::order_mass_cancel_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::order_mass_cancel_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::order_mass_cancel_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassCancelReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_mass_cancel_report::Order>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_cancel_report::Order>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_cancel_report::Order>),
}

impl Serialize for OrderMassCancelReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrderMassCancelReport::FIX50(m) => m.serialize(serializer),
            OrderMassCancelReport::FIX50SP1(m) => m.serialize(serializer),
            OrderMassCancelReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for OrderMassCancelReport {
    fn get_header(&self) -> &Header {
        match self {
            OrderMassCancelReport::FIX50(m) => m.get_header(),
            OrderMassCancelReport::FIX50SP1(m) => m.get_header(),
            OrderMassCancelReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            OrderMassCancelReport::FIX50(m) => m.get_header_mut(),
            OrderMassCancelReport::FIX50SP1(m) => m.get_header_mut(),
            OrderMassCancelReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::order_mass_cancel_report::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::order_mass_cancel_report::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::order_mass_cancel_report::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NewOrderCross {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::new_order_cross::New>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::new_order_cross::New>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::new_order_cross::New>),
}

impl Serialize for NewOrderCross {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NewOrderCross::FIX50(m) => m.serialize(serializer),
            NewOrderCross::FIX50SP1(m) => m.serialize(serializer),
            NewOrderCross::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for NewOrderCross {
    fn get_header(&self) -> &Header {
        match self {
            NewOrderCross::FIX50(m) => m.get_header(),
            NewOrderCross::FIX50SP1(m) => m.get_header(),
            NewOrderCross::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            NewOrderCross::FIX50(m) => m.get_header_mut(),
            NewOrderCross::FIX50SP1(m) => m.get_header_mut(),
            NewOrderCross::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::new_order_cross::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::new_order_cross::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::new_order_cross::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CrossOrderCancelReplaceRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::cross_order_cancel_replace_request::Cross>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::cross_order_cancel_replace_request::Cross>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::cross_order_cancel_replace_request::Cross>),
}

impl Serialize for CrossOrderCancelReplaceRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CrossOrderCancelReplaceRequest::FIX50(m) => m.serialize(serializer),
            CrossOrderCancelReplaceRequest::FIX50SP1(m) => m.serialize(serializer),
            CrossOrderCancelReplaceRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for CrossOrderCancelReplaceRequest {
    fn get_header(&self) -> &Header {
        match self {
            CrossOrderCancelReplaceRequest::FIX50(m) => m.get_header(),
            CrossOrderCancelReplaceRequest::FIX50SP1(m) => m.get_header(),
            CrossOrderCancelReplaceRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            CrossOrderCancelReplaceRequest::FIX50(m) => m.get_header_mut(),
            CrossOrderCancelReplaceRequest::FIX50SP1(m) => m.get_header_mut(),
            CrossOrderCancelReplaceRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::cross_order_cancel_replace_request::Cross {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::cross_order_cancel_replace_request::Cross {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::cross_order_cancel_replace_request::Cross {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CrossOrderCancelRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::cross_order_cancel_request::Cross>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::cross_order_cancel_request::Cross>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::cross_order_cancel_request::Cross>),
}

impl Serialize for CrossOrderCancelRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CrossOrderCancelRequest::FIX50(m) => m.serialize(serializer),
            CrossOrderCancelRequest::FIX50SP1(m) => m.serialize(serializer),
            CrossOrderCancelRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for CrossOrderCancelRequest {
    fn get_header(&self) -> &Header {
        match self {
            CrossOrderCancelRequest::FIX50(m) => m.get_header(),
            CrossOrderCancelRequest::FIX50SP1(m) => m.get_header(),
            CrossOrderCancelRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            CrossOrderCancelRequest::FIX50(m) => m.get_header_mut(),
            CrossOrderCancelRequest::FIX50SP1(m) => m.get_header_mut(),
            CrossOrderCancelRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::cross_order_cancel_request::Cross {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::cross_order_cancel_request::Cross {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::cross_order_cancel_request::Cross {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityTypeRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_type_request::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_type_request::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_type_request::Security>),
}

impl Serialize for SecurityTypeRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityTypeRequest::FIX50(m) => m.serialize(serializer),
            SecurityTypeRequest::FIX50SP1(m) => m.serialize(serializer),
            SecurityTypeRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityTypeRequest {
    fn get_header(&self) -> &Header {
        match self {
            SecurityTypeRequest::FIX50(m) => m.get_header(),
            SecurityTypeRequest::FIX50SP1(m) => m.get_header(),
            SecurityTypeRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityTypeRequest::FIX50(m) => m.get_header_mut(),
            SecurityTypeRequest::FIX50SP1(m) => m.get_header_mut(),
            SecurityTypeRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_type_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_type_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_type_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityTypes {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_types::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_types::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_types::Security>),
}

impl Serialize for SecurityTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityTypes::FIX50(m) => m.serialize(serializer),
            SecurityTypes::FIX50SP1(m) => m.serialize(serializer),
            SecurityTypes::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityTypes {
    fn get_header(&self) -> &Header {
        match self {
            SecurityTypes::FIX50(m) => m.get_header(),
            SecurityTypes::FIX50SP1(m) => m.get_header(),
            SecurityTypes::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityTypes::FIX50(m) => m.get_header_mut(),
            SecurityTypes::FIX50SP1(m) => m.get_header_mut(),
            SecurityTypes::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_types::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_types::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_types::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityListRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_list_request::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_list_request::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_list_request::Security>),
}

impl Serialize for SecurityListRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityListRequest::FIX50(m) => m.serialize(serializer),
            SecurityListRequest::FIX50SP1(m) => m.serialize(serializer),
            SecurityListRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityListRequest {
    fn get_header(&self) -> &Header {
        match self {
            SecurityListRequest::FIX50(m) => m.get_header(),
            SecurityListRequest::FIX50SP1(m) => m.get_header(),
            SecurityListRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityListRequest::FIX50(m) => m.get_header_mut(),
            SecurityListRequest::FIX50SP1(m) => m.get_header_mut(),
            SecurityListRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_list_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_list_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_list_request::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityList {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_list::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_list::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_list::Security>),
}

impl Serialize for SecurityList {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityList::FIX50(m) => m.serialize(serializer),
            SecurityList::FIX50SP1(m) => m.serialize(serializer),
            SecurityList::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityList {
    fn get_header(&self) -> &Header {
        match self {
            SecurityList::FIX50(m) => m.get_header(),
            SecurityList::FIX50SP1(m) => m.get_header(),
            SecurityList::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityList::FIX50(m) => m.get_header_mut(),
            SecurityList::FIX50SP1(m) => m.get_header_mut(),
            SecurityList::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_list::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_list::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_list::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum DerivativeSecurityListRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::derivative_security_list_request::Derivative>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::derivative_security_list_request::Derivative>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::derivative_security_list_request::Derivative>),
}

impl Serialize for DerivativeSecurityListRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DerivativeSecurityListRequest::FIX50(m) => m.serialize(serializer),
            DerivativeSecurityListRequest::FIX50SP1(m) => m.serialize(serializer),
            DerivativeSecurityListRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for DerivativeSecurityListRequest {
    fn get_header(&self) -> &Header {
        match self {
            DerivativeSecurityListRequest::FIX50(m) => m.get_header(),
            DerivativeSecurityListRequest::FIX50SP1(m) => m.get_header(),
            DerivativeSecurityListRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            DerivativeSecurityListRequest::FIX50(m) => m.get_header_mut(),
            DerivativeSecurityListRequest::FIX50SP1(m) => m.get_header_mut(),
            DerivativeSecurityListRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::derivative_security_list_request::Derivative {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::derivative_security_list_request::Derivative {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::derivative_security_list_request::Derivative {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum DerivativeSecurityList {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::derivative_security_list::Derivative>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::derivative_security_list::Derivative>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::derivative_security_list::Derivative>),
}

impl Serialize for DerivativeSecurityList {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DerivativeSecurityList::FIX50(m) => m.serialize(serializer),
            DerivativeSecurityList::FIX50SP1(m) => m.serialize(serializer),
            DerivativeSecurityList::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for DerivativeSecurityList {
    fn get_header(&self) -> &Header {
        match self {
            DerivativeSecurityList::FIX50(m) => m.get_header(),
            DerivativeSecurityList::FIX50SP1(m) => m.get_header(),
            DerivativeSecurityList::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            DerivativeSecurityList::FIX50(m) => m.get_header_mut(),
            DerivativeSecurityList::FIX50SP1(m) => m.get_header_mut(),
            DerivativeSecurityList::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::derivative_security_list::Derivative {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::derivative_security_list::Derivative {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::derivative_security_list::Derivative {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NewOrderMultileg {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::new_order_multileg::New>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::new_order_multileg::New>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::new_order_multileg::New>),
}

impl Serialize for NewOrderMultileg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NewOrderMultileg::FIX50(m) => m.serialize(serializer),
            NewOrderMultileg::FIX50SP1(m) => m.serialize(serializer),
            NewOrderMultileg::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for NewOrderMultileg {
    fn get_header(&self) -> &Header {
        match self {
            NewOrderMultileg::FIX50(m) => m.get_header(),
            NewOrderMultileg::FIX50SP1(m) => m.get_header(),
            NewOrderMultileg::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            NewOrderMultileg::FIX50(m) => m.get_header_mut(),
            NewOrderMultileg::FIX50SP1(m) => m.get_header_mut(),
            NewOrderMultileg::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::new_order_multileg::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::new_order_multileg::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::new_order_multileg::New {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MultilegOrderCancelReplace {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::multileg_order_cancel_replace_request::Multileg>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::multileg_order_cancel_replace_request::Multileg>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::multileg_order_cancel_replace_request::Multileg>),
}

impl Serialize for MultilegOrderCancelReplace {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MultilegOrderCancelReplace::FIX50(m) => m.serialize(serializer),
            MultilegOrderCancelReplace::FIX50SP1(m) => m.serialize(serializer),
            MultilegOrderCancelReplace::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MultilegOrderCancelReplace {
    fn get_header(&self) -> &Header {
        match self {
            MultilegOrderCancelReplace::FIX50(m) => m.get_header(),
            MultilegOrderCancelReplace::FIX50SP1(m) => m.get_header(),
            MultilegOrderCancelReplace::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MultilegOrderCancelReplace::FIX50(m) => m.get_header_mut(),
            MultilegOrderCancelReplace::FIX50SP1(m) => m.get_header_mut(),
            MultilegOrderCancelReplace::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::multileg_order_cancel_replace_request::Multileg {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::multileg_order_cancel_replace_request::Multileg {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::multileg_order_cancel_replace_request::Multileg {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradeCaptureReportRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trade_capture_report_request::Trade>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trade_capture_report_request::Trade>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trade_capture_report_request::Trade>),
}

impl Serialize for TradeCaptureReportRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TradeCaptureReportRequest::FIX50(m) => m.serialize(serializer),
            TradeCaptureReportRequest::FIX50SP1(m) => m.serialize(serializer),
            TradeCaptureReportRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for TradeCaptureReportRequest {
    fn get_header(&self) -> &Header {
        match self {
            TradeCaptureReportRequest::FIX50(m) => m.get_header(),
            TradeCaptureReportRequest::FIX50SP1(m) => m.get_header(),
            TradeCaptureReportRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            TradeCaptureReportRequest::FIX50(m) => m.get_header_mut(),
            TradeCaptureReportRequest::FIX50SP1(m) => m.get_header_mut(),
            TradeCaptureReportRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::trade_capture_report_request::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::trade_capture_report_request::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::trade_capture_report_request::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradeCaptureReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trade_capture_report::Trade>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trade_capture_report::Trade>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trade_capture_report::TradeCaptureReport>),
}

impl Serialize for TradeCaptureReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TradeCaptureReport::FIX50(m) => m.serialize(serializer),
            TradeCaptureReport::FIX50SP1(m) => m.serialize(serializer),
            TradeCaptureReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for TradeCaptureReport {
    fn get_header(&self) -> &Header {
        match self {
            TradeCaptureReport::FIX50(m) => m.get_header(),
            TradeCaptureReport::FIX50SP1(m) => m.get_header(),
            TradeCaptureReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            TradeCaptureReport::FIX50(m) => m.get_header_mut(),
            TradeCaptureReport::FIX50SP1(m) => m.get_header_mut(),
            TradeCaptureReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::trade_capture_report::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::trade_capture_report::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::trade_capture_report::TradeCaptureReport {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassStatusRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::order_mass_status_request::Order>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_status_request::Order>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_status_request::Order>),
}

impl Serialize for OrderMassStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrderMassStatusRequest::FIX50(m) => m.serialize(serializer),
            OrderMassStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            OrderMassStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for OrderMassStatusRequest {
    fn get_header(&self) -> &Header {
        match self {
            OrderMassStatusRequest::FIX50(m) => m.get_header(),
            OrderMassStatusRequest::FIX50SP1(m) => m.get_header(),
            OrderMassStatusRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            OrderMassStatusRequest::FIX50(m) => m.get_header_mut(),
            OrderMassStatusRequest::FIX50SP1(m) => m.get_header_mut(),
            OrderMassStatusRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::order_mass_status_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::order_mass_status_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::order_mass_status_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteRequestReject {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_request_reject::Quote>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_request_reject::Quote>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_request_reject::Quote>),
}

impl Serialize for QuoteRequestReject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            QuoteRequestReject::FIX50(m) => m.serialize(serializer),
            QuoteRequestReject::FIX50SP1(m) => m.serialize(serializer),
            QuoteRequestReject::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for QuoteRequestReject {
    fn get_header(&self) -> &Header {
        match self {
            QuoteRequestReject::FIX50(m) => m.get_header(),
            QuoteRequestReject::FIX50SP1(m) => m.get_header(),
            QuoteRequestReject::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            QuoteRequestReject::FIX50(m) => m.get_header_mut(),
            QuoteRequestReject::FIX50SP1(m) => m.get_header_mut(),
            QuoteRequestReject::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::quote_request_reject::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::quote_request_reject::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::quote_request_reject::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RFQRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::rfq_request::RFQ>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::rfq_request::RFQ>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::rfq_request::RFQ>),
}

impl Serialize for RFQRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RFQRequest::FIX50(m) => m.serialize(serializer),
            RFQRequest::FIX50SP1(m) => m.serialize(serializer),
            RFQRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for RFQRequest {
    fn get_header(&self) -> &Header {
        match self {
            RFQRequest::FIX50(m) => m.get_header(),
            RFQRequest::FIX50SP1(m) => m.get_header(),
            RFQRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            RFQRequest::FIX50(m) => m.get_header_mut(),
            RFQRequest::FIX50SP1(m) => m.get_header_mut(),
            RFQRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::rfq_request::RFQ {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::rfq_request::RFQ {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::rfq_request::RFQ {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteStatusReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_status_report::Quote>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_status_report::Quote>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_status_report::QuoteStatusReport>),
}

impl Serialize for QuoteStatusReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            QuoteStatusReport::FIX50(m) => m.serialize(serializer),
            QuoteStatusReport::FIX50SP1(m) => m.serialize(serializer),
            QuoteStatusReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for QuoteStatusReport {
    fn get_header(&self) -> &Header {
        match self {
            QuoteStatusReport::FIX50(m) => m.get_header(),
            QuoteStatusReport::FIX50SP1(m) => m.get_header(),
            QuoteStatusReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            QuoteStatusReport::FIX50(m) => m.get_header_mut(),
            QuoteStatusReport::FIX50SP1(m) => m.get_header_mut(),
            QuoteStatusReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::quote_status_report::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::quote_status_report::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::quote_status_report::QuoteStatusReport {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum QuoteResponse {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::quote_response::Quote>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::quote_response::Quote>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::quote_response::QuoteResponse>),
}

impl Serialize for QuoteResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            QuoteResponse::FIX50(m) => m.serialize(serializer),
            QuoteResponse::FIX50SP1(m) => m.serialize(serializer),
            QuoteResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for QuoteResponse {
    fn get_header(&self) -> &Header {
        match self {
            QuoteResponse::FIX50(m) => m.get_header(),
            QuoteResponse::FIX50SP1(m) => m.get_header(),
            QuoteResponse::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            QuoteResponse::FIX50(m) => m.get_header_mut(),
            QuoteResponse::FIX50SP1(m) => m.get_header_mut(),
            QuoteResponse::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::quote_response::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::quote_response::Quote {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::quote_response::QuoteResponse {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum Confirmation {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::confirmation::Confirmation>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::confirmation::Confirmation>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::confirmation::Confirmation>),
}

impl Serialize for Confirmation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Confirmation::FIX50(m) => m.serialize(serializer),
            Confirmation::FIX50SP1(m) => m.serialize(serializer),
            Confirmation::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for Confirmation {
    fn get_header(&self) -> &Header {
        match self {
            Confirmation::FIX50(m) => m.get_header(),
            Confirmation::FIX50SP1(m) => m.get_header(),
            Confirmation::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            Confirmation::FIX50(m) => m.get_header_mut(),
            Confirmation::FIX50SP1(m) => m.get_header_mut(),
            Confirmation::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::confirmation::Confirmation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::confirmation::Confirmation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::confirmation::Confirmation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum PositionMaintenanceRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::position_maintenance_request::Position>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::position_maintenance_request::Position>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::position_maintenance_request::Position>),
}

impl Serialize for PositionMaintenanceRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PositionMaintenanceRequest::FIX50(m) => m.serialize(serializer),
            PositionMaintenanceRequest::FIX50SP1(m) => m.serialize(serializer),
            PositionMaintenanceRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for PositionMaintenanceRequest {
    fn get_header(&self) -> &Header {
        match self {
            PositionMaintenanceRequest::FIX50(m) => m.get_header(),
            PositionMaintenanceRequest::FIX50SP1(m) => m.get_header(),
            PositionMaintenanceRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            PositionMaintenanceRequest::FIX50(m) => m.get_header_mut(),
            PositionMaintenanceRequest::FIX50SP1(m) => m.get_header_mut(),
            PositionMaintenanceRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::position_maintenance_request::Position {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::position_maintenance_request::Position {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::position_maintenance_request::Position {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum PositionMaintenanceReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::position_maintenance_report::Position>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::position_maintenance_report::Position>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::position_maintenance_report::Position>),
}

impl Serialize for PositionMaintenanceReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PositionMaintenanceReport::FIX50(m) => m.serialize(serializer),
            PositionMaintenanceReport::FIX50SP1(m) => m.serialize(serializer),
            PositionMaintenanceReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for PositionMaintenanceReport {
    fn get_header(&self) -> &Header {
        match self {
            PositionMaintenanceReport::FIX50(m) => m.get_header(),
            PositionMaintenanceReport::FIX50SP1(m) => m.get_header(),
            PositionMaintenanceReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            PositionMaintenanceReport::FIX50(m) => m.get_header_mut(),
            PositionMaintenanceReport::FIX50SP1(m) => m.get_header_mut(),
            PositionMaintenanceReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::position_maintenance_report::Position {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::position_maintenance_report::Position {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::position_maintenance_report::Position {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RequestForPositions {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::request_for_positions::Request>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::request_for_positions::Request>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::request_for_positions::Request>),
}

impl Serialize for RequestForPositions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RequestForPositions::FIX50(m) => m.serialize(serializer),
            RequestForPositions::FIX50SP1(m) => m.serialize(serializer),
            RequestForPositions::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for RequestForPositions {
    fn get_header(&self) -> &Header {
        match self {
            RequestForPositions::FIX50(m) => m.get_header(),
            RequestForPositions::FIX50SP1(m) => m.get_header(),
            RequestForPositions::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            RequestForPositions::FIX50(m) => m.get_header_mut(),
            RequestForPositions::FIX50SP1(m) => m.get_header_mut(),
            RequestForPositions::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::request_for_positions::Request {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::request_for_positions::Request {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::request_for_positions::Request {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum RequestForPositionsAck {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::request_for_positions_ack::Request>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::request_for_positions_ack::Request>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::request_for_positions_ack::Request>),
}

impl Serialize for RequestForPositionsAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RequestForPositionsAck::FIX50(m) => m.serialize(serializer),
            RequestForPositionsAck::FIX50SP1(m) => m.serialize(serializer),
            RequestForPositionsAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for RequestForPositionsAck {
    fn get_header(&self) -> &Header {
        match self {
            RequestForPositionsAck::FIX50(m) => m.get_header(),
            RequestForPositionsAck::FIX50SP1(m) => m.get_header(),
            RequestForPositionsAck::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            RequestForPositionsAck::FIX50(m) => m.get_header_mut(),
            RequestForPositionsAck::FIX50SP1(m) => m.get_header_mut(),
            RequestForPositionsAck::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::request_for_positions_ack::Request {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::request_for_positions_ack::Request {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::request_for_positions_ack::Request {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum PositionReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::position_report::Position>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::position_report::Position>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::position_report::Position>),
}

impl Serialize for PositionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PositionReport::FIX50(m) => m.serialize(serializer),
            PositionReport::FIX50SP1(m) => m.serialize(serializer),
            PositionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for PositionReport {
    fn get_header(&self) -> &Header {
        match self {
            PositionReport::FIX50(m) => m.get_header(),
            PositionReport::FIX50SP1(m) => m.get_header(),
            PositionReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            PositionReport::FIX50(m) => m.get_header_mut(),
            PositionReport::FIX50SP1(m) => m.get_header_mut(),
            PositionReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::position_report::Position {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::position_report::Position {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::position_report::Position {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradeCaptureReportRequestAck {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trade_capture_report_request_ack::Trade>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trade_capture_report_request_ack::Trade>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trade_capture_report_request_ack::Trade>),
}

impl Serialize for TradeCaptureReportRequestAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TradeCaptureReportRequestAck::FIX50(m) => m.serialize(serializer),
            TradeCaptureReportRequestAck::FIX50SP1(m) => m.serialize(serializer),
            TradeCaptureReportRequestAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for TradeCaptureReportRequestAck {
    fn get_header(&self) -> &Header {
        match self {
            TradeCaptureReportRequestAck::FIX50(m) => m.get_header(),
            TradeCaptureReportRequestAck::FIX50SP1(m) => m.get_header(),
            TradeCaptureReportRequestAck::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            TradeCaptureReportRequestAck::FIX50(m) => m.get_header_mut(),
            TradeCaptureReportRequestAck::FIX50SP1(m) => m.get_header_mut(),
            TradeCaptureReportRequestAck::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::trade_capture_report_request_ack::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::trade_capture_report_request_ack::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::trade_capture_report_request_ack::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradeCaptureReportAck {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trade_capture_report_ack::Trade>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trade_capture_report_ack::Trade>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trade_capture_report_ack::TradeCaptureReportAck>),
}

impl Serialize for TradeCaptureReportAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TradeCaptureReportAck::FIX50(m) => m.serialize(serializer),
            TradeCaptureReportAck::FIX50SP1(m) => m.serialize(serializer),
            TradeCaptureReportAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for TradeCaptureReportAck {
    fn get_header(&self) -> &Header {
        match self {
            TradeCaptureReportAck::FIX50(m) => m.get_header(),
            TradeCaptureReportAck::FIX50SP1(m) => m.get_header(),
            TradeCaptureReportAck::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            TradeCaptureReportAck::FIX50(m) => m.get_header_mut(),
            TradeCaptureReportAck::FIX50SP1(m) => m.get_header_mut(),
            TradeCaptureReportAck::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::trade_capture_report_ack::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::trade_capture_report_ack::Trade {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::trade_capture_report_ack::TradeCaptureReportAck {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_report::Allocation>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_report::Allocation>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_report::AllocationReport>),
}

impl Serialize for AllocationReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AllocationReport::FIX50(m) => m.serialize(serializer),
            AllocationReport::FIX50SP1(m) => m.serialize(serializer),
            AllocationReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for AllocationReport {
    fn get_header(&self) -> &Header {
        match self {
            AllocationReport::FIX50(m) => m.get_header(),
            AllocationReport::FIX50SP1(m) => m.get_header(),
            AllocationReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            AllocationReport::FIX50(m) => m.get_header_mut(),
            AllocationReport::FIX50SP1(m) => m.get_header_mut(),
            AllocationReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::allocation_report::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::allocation_report::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::allocation_report::AllocationReport {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationReportAck {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_report_ack::Allocation>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_report_ack::Allocation>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_report_ack::Allocation>),
}

impl Serialize for AllocationReportAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AllocationReportAck::FIX50(m) => m.serialize(serializer),
            AllocationReportAck::FIX50SP1(m) => m.serialize(serializer),
            AllocationReportAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for AllocationReportAck {
    fn get_header(&self) -> &Header {
        match self {
            AllocationReportAck::FIX50(m) => m.get_header(),
            AllocationReportAck::FIX50SP1(m) => m.get_header(),
            AllocationReportAck::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            AllocationReportAck::FIX50(m) => m.get_header_mut(),
            AllocationReportAck::FIX50SP1(m) => m.get_header_mut(),
            AllocationReportAck::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::allocation_report_ack::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::allocation_report_ack::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::allocation_report_ack::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ConfirmationAck {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::confirmation_ack::Confirmation>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::confirmation_ack::Confirmation>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::confirmation_ack::Confirmation>),
}

impl Serialize for ConfirmationAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ConfirmationAck::FIX50(m) => m.serialize(serializer),
            ConfirmationAck::FIX50SP1(m) => m.serialize(serializer),
            ConfirmationAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ConfirmationAck {
    fn get_header(&self) -> &Header {
        match self {
            ConfirmationAck::FIX50(m) => m.get_header(),
            ConfirmationAck::FIX50SP1(m) => m.get_header(),
            ConfirmationAck::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ConfirmationAck::FIX50(m) => m.get_header_mut(),
            ConfirmationAck::FIX50SP1(m) => m.get_header_mut(),
            ConfirmationAck::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::confirmation_ack::Confirmation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::confirmation_ack::Confirmation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::confirmation_ack::Confirmation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SettlementInstructionRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::settlement_instruction_request::Settlement>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::settlement_instruction_request::Settlement>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::settlement_instruction_request::Settlement>),
}

impl Serialize for SettlementInstructionRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SettlementInstructionRequest::FIX50(m) => m.serialize(serializer),
            SettlementInstructionRequest::FIX50SP1(m) => m.serialize(serializer),
            SettlementInstructionRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SettlementInstructionRequest {
    fn get_header(&self) -> &Header {
        match self {
            SettlementInstructionRequest::FIX50(m) => m.get_header(),
            SettlementInstructionRequest::FIX50SP1(m) => m.get_header(),
            SettlementInstructionRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SettlementInstructionRequest::FIX50(m) => m.get_header_mut(),
            SettlementInstructionRequest::FIX50SP1(m) => m.get_header_mut(),
            SettlementInstructionRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::settlement_instruction_request::Settlement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::settlement_instruction_request::Settlement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::settlement_instruction_request::Settlement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AssignmentReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::assignment_report::Assignment>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::assignment_report::Assignment>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::assignment_report::Assignment>),
}

impl Serialize for AssignmentReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AssignmentReport::FIX50(m) => m.serialize(serializer),
            AssignmentReport::FIX50SP1(m) => m.serialize(serializer),
            AssignmentReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for AssignmentReport {
    fn get_header(&self) -> &Header {
        match self {
            AssignmentReport::FIX50(m) => m.get_header(),
            AssignmentReport::FIX50SP1(m) => m.get_header(),
            AssignmentReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            AssignmentReport::FIX50(m) => m.get_header_mut(),
            AssignmentReport::FIX50SP1(m) => m.get_header_mut(),
            AssignmentReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::assignment_report::Assignment {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::assignment_report::Assignment {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::assignment_report::Assignment {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_request::Collateral>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_request::Collateral>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_request::Collateral>),
}

impl Serialize for CollateralRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CollateralRequest::FIX50(m) => m.serialize(serializer),
            CollateralRequest::FIX50SP1(m) => m.serialize(serializer),
            CollateralRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for CollateralRequest {
    fn get_header(&self) -> &Header {
        match self {
            CollateralRequest::FIX50(m) => m.get_header(),
            CollateralRequest::FIX50SP1(m) => m.get_header(),
            CollateralRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            CollateralRequest::FIX50(m) => m.get_header_mut(),
            CollateralRequest::FIX50SP1(m) => m.get_header_mut(),
            CollateralRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::collateral_request::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::collateral_request::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::collateral_request::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralAssignment {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_assignment::Collateral>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_assignment::Collateral>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_assignment::Collateral>),
}

impl Serialize for CollateralAssignment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CollateralAssignment::FIX50(m) => m.serialize(serializer),
            CollateralAssignment::FIX50SP1(m) => m.serialize(serializer),
            CollateralAssignment::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for CollateralAssignment {
    fn get_header(&self) -> &Header {
        match self {
            CollateralAssignment::FIX50(m) => m.get_header(),
            CollateralAssignment::FIX50SP1(m) => m.get_header(),
            CollateralAssignment::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            CollateralAssignment::FIX50(m) => m.get_header_mut(),
            CollateralAssignment::FIX50SP1(m) => m.get_header_mut(),
            CollateralAssignment::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::collateral_assignment::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::collateral_assignment::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::collateral_assignment::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralResponse {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_response::Collateral>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_response::Collateral>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_response::Collateral>),
}

impl Serialize for CollateralResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CollateralResponse::FIX50(m) => m.serialize(serializer),
            CollateralResponse::FIX50SP1(m) => m.serialize(serializer),
            CollateralResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for CollateralResponse {
    fn get_header(&self) -> &Header {
        match self {
            CollateralResponse::FIX50(m) => m.get_header(),
            CollateralResponse::FIX50SP1(m) => m.get_header(),
            CollateralResponse::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            CollateralResponse::FIX50(m) => m.get_header_mut(),
            CollateralResponse::FIX50SP1(m) => m.get_header_mut(),
            CollateralResponse::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::collateral_response::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::collateral_response::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::collateral_response::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_report::Collateral>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_report::Collateral>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_report::Collateral>),
}

impl Serialize for CollateralReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CollateralReport::FIX50(m) => m.serialize(serializer),
            CollateralReport::FIX50SP1(m) => m.serialize(serializer),
            CollateralReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for CollateralReport {
    fn get_header(&self) -> &Header {
        match self {
            CollateralReport::FIX50(m) => m.get_header(),
            CollateralReport::FIX50SP1(m) => m.get_header(),
            CollateralReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            CollateralReport::FIX50(m) => m.get_header_mut(),
            CollateralReport::FIX50SP1(m) => m.get_header_mut(),
            CollateralReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::collateral_report::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::collateral_report::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::collateral_report::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralInquiry {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_inquiry::Collateral>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_inquiry::Collateral>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_inquiry::Collateral>),
}

impl Serialize for CollateralInquiry {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CollateralInquiry::FIX50(m) => m.serialize(serializer),
            CollateralInquiry::FIX50SP1(m) => m.serialize(serializer),
            CollateralInquiry::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for CollateralInquiry {
    fn get_header(&self) -> &Header {
        match self {
            CollateralInquiry::FIX50(m) => m.get_header(),
            CollateralInquiry::FIX50SP1(m) => m.get_header(),
            CollateralInquiry::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            CollateralInquiry::FIX50(m) => m.get_header_mut(),
            CollateralInquiry::FIX50SP1(m) => m.get_header_mut(),
            CollateralInquiry::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::collateral_inquiry::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::collateral_inquiry::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::collateral_inquiry::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NetworkCounterpartySystemStatusRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::network_counterparty_system_status_request::Network>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::network_counterparty_system_status_request::Network>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::network_counterparty_system_status_request::Network>),
}

impl Serialize for NetworkCounterpartySystemStatusRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NetworkCounterpartySystemStatusRequest::FIX50(m) => m.serialize(serializer),
            NetworkCounterpartySystemStatusRequest::FIX50SP1(m) => m.serialize(serializer),
            NetworkCounterpartySystemStatusRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for NetworkCounterpartySystemStatusRequest {
    fn get_header(&self) -> &Header {
        match self {
            NetworkCounterpartySystemStatusRequest::FIX50(m) => m.get_header(),
            NetworkCounterpartySystemStatusRequest::FIX50SP1(m) => m.get_header(),
            NetworkCounterpartySystemStatusRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            NetworkCounterpartySystemStatusRequest::FIX50(m) => m.get_header_mut(),
            NetworkCounterpartySystemStatusRequest::FIX50SP1(m) => m.get_header_mut(),
            NetworkCounterpartySystemStatusRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::network_counterparty_system_status_request::Network {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::network_counterparty_system_status_request::Network {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::network_counterparty_system_status_request::Network {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum NetworkCounterpartySystemStatusResponse {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::network_counterparty_system_status_response::Network>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::network_counterparty_system_status_response::Network>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::network_counterparty_system_status_response::Network>),
}

impl Serialize for NetworkCounterpartySystemStatusResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NetworkCounterpartySystemStatusResponse::FIX50(m) => m.serialize(serializer),
            NetworkCounterpartySystemStatusResponse::FIX50SP1(m) => m.serialize(serializer),
            NetworkCounterpartySystemStatusResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for NetworkCounterpartySystemStatusResponse {
    fn get_header(&self) -> &Header {
        match self {
            NetworkCounterpartySystemStatusResponse::FIX50(m) => m.get_header(),
            NetworkCounterpartySystemStatusResponse::FIX50SP1(m) => m.get_header(),
            NetworkCounterpartySystemStatusResponse::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            NetworkCounterpartySystemStatusResponse::FIX50(m) => m.get_header_mut(),
            NetworkCounterpartySystemStatusResponse::FIX50SP1(m) => m.get_header_mut(),
            NetworkCounterpartySystemStatusResponse::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::network_counterparty_system_status_response::Network {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::network_counterparty_system_status_response::Network {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::network_counterparty_system_status_response::Network {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum UserRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::user_request::User>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::user_request::User>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::user_request::User>),
}

impl Serialize for UserRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            UserRequest::FIX50(m) => m.serialize(serializer),
            UserRequest::FIX50SP1(m) => m.serialize(serializer),
            UserRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for UserRequest {
    fn get_header(&self) -> &Header {
        match self {
            UserRequest::FIX50(m) => m.get_header(),
            UserRequest::FIX50SP1(m) => m.get_header(),
            UserRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            UserRequest::FIX50(m) => m.get_header_mut(),
            UserRequest::FIX50SP1(m) => m.get_header_mut(),
            UserRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::user_request::User {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::user_request::User {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::user_request::User {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum UserResponse {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::user_response::User>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::user_response::User>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::user_response::User>),
}

impl Serialize for UserResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            UserResponse::FIX50(m) => m.serialize(serializer),
            UserResponse::FIX50SP1(m) => m.serialize(serializer),
            UserResponse::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for UserResponse {
    fn get_header(&self) -> &Header {
        match self {
            UserResponse::FIX50(m) => m.get_header(),
            UserResponse::FIX50SP1(m) => m.get_header(),
            UserResponse::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            UserResponse::FIX50(m) => m.get_header_mut(),
            UserResponse::FIX50SP1(m) => m.get_header_mut(),
            UserResponse::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::user_response::User {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::user_response::User {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::user_response::User {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum CollateralInquiryAck {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::collateral_inquiry_ack::Collateral>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::collateral_inquiry_ack::Collateral>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::collateral_inquiry_ack::Collateral>),
}

impl Serialize for CollateralInquiryAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CollateralInquiryAck::FIX50(m) => m.serialize(serializer),
            CollateralInquiryAck::FIX50SP1(m) => m.serialize(serializer),
            CollateralInquiryAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for CollateralInquiryAck {
    fn get_header(&self) -> &Header {
        match self {
            CollateralInquiryAck::FIX50(m) => m.get_header(),
            CollateralInquiryAck::FIX50SP1(m) => m.get_header(),
            CollateralInquiryAck::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            CollateralInquiryAck::FIX50(m) => m.get_header_mut(),
            CollateralInquiryAck::FIX50SP1(m) => m.get_header_mut(),
            CollateralInquiryAck::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::collateral_inquiry_ack::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::collateral_inquiry_ack::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::collateral_inquiry_ack::Collateral {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ConfirmationRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::confirmation_request::Confirmation>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::confirmation_request::Confirmation>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::confirmation_request::Confirmation>),
}

impl Serialize for ConfirmationRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ConfirmationRequest::FIX50(m) => m.serialize(serializer),
            ConfirmationRequest::FIX50SP1(m) => m.serialize(serializer),
            ConfirmationRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ConfirmationRequest {
    fn get_header(&self) -> &Header {
        match self {
            ConfirmationRequest::FIX50(m) => m.get_header(),
            ConfirmationRequest::FIX50SP1(m) => m.get_header(),
            ConfirmationRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ConfirmationRequest::FIX50(m) => m.get_header_mut(),
            ConfirmationRequest::FIX50SP1(m) => m.get_header_mut(),
            ConfirmationRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::confirmation_request::Confirmation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::confirmation_request::Confirmation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::confirmation_request::Confirmation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionListRequest {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trading_session_list_request::Trading>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_list_request::Trading>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_list_request::Trading>),
}

impl Serialize for TradingSessionListRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TradingSessionListRequest::FIX50(m) => m.serialize(serializer),
            TradingSessionListRequest::FIX50SP1(m) => m.serialize(serializer),
            TradingSessionListRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for TradingSessionListRequest {
    fn get_header(&self) -> &Header {
        match self {
            TradingSessionListRequest::FIX50(m) => m.get_header(),
            TradingSessionListRequest::FIX50SP1(m) => m.get_header(),
            TradingSessionListRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            TradingSessionListRequest::FIX50(m) => m.get_header_mut(),
            TradingSessionListRequest::FIX50SP1(m) => m.get_header_mut(),
            TradingSessionListRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::trading_session_list_request::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::trading_session_list_request::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::trading_session_list_request::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionList {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::trading_session_list::Trading>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_list::Trading>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_list::Trading>),
}

impl Serialize for TradingSessionList {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TradingSessionList::FIX50(m) => m.serialize(serializer),
            TradingSessionList::FIX50SP1(m) => m.serialize(serializer),
            TradingSessionList::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for TradingSessionList {
    fn get_header(&self) -> &Header {
        match self {
            TradingSessionList::FIX50(m) => m.get_header(),
            TradingSessionList::FIX50SP1(m) => m.get_header(),
            TradingSessionList::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            TradingSessionList::FIX50(m) => m.get_header_mut(),
            TradingSessionList::FIX50SP1(m) => m.get_header_mut(),
            TradingSessionList::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::trading_session_list::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::trading_session_list::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::trading_session_list::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityListUpdateReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_list_update_report::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_list_update_report::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_list_update_report::Security>),
}

impl Serialize for SecurityListUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityListUpdateReport::FIX50(m) => m.serialize(serializer),
            SecurityListUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            SecurityListUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityListUpdateReport {
    fn get_header(&self) -> &Header {
        match self {
            SecurityListUpdateReport::FIX50(m) => m.get_header(),
            SecurityListUpdateReport::FIX50SP1(m) => m.get_header(),
            SecurityListUpdateReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityListUpdateReport::FIX50(m) => m.get_header_mut(),
            SecurityListUpdateReport::FIX50SP1(m) => m.get_header_mut(),
            SecurityListUpdateReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_list_update_report::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_list_update_report::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_list_update_report::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AdjustedPositionReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::adjusted_position_report::Adjusted>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::adjusted_position_report::Adjusted>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::adjusted_position_report::Adjusted>),
}

impl Serialize for AdjustedPositionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AdjustedPositionReport::FIX50(m) => m.serialize(serializer),
            AdjustedPositionReport::FIX50SP1(m) => m.serialize(serializer),
            AdjustedPositionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for AdjustedPositionReport {
    fn get_header(&self) -> &Header {
        match self {
            AdjustedPositionReport::FIX50(m) => m.get_header(),
            AdjustedPositionReport::FIX50SP1(m) => m.get_header(),
            AdjustedPositionReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            AdjustedPositionReport::FIX50(m) => m.get_header_mut(),
            AdjustedPositionReport::FIX50SP1(m) => m.get_header_mut(),
            AdjustedPositionReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::adjusted_position_report::Adjusted {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::adjusted_position_report::Adjusted {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::adjusted_position_report::Adjusted {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum AllocationInstructionAlert {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::allocation_instruction_alert::Allocation>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::allocation_instruction_alert::Allocation>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::allocation_instruction_alert::AllocationInstructionAlert>),
}

impl Serialize for AllocationInstructionAlert {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AllocationInstructionAlert::FIX50(m) => m.serialize(serializer),
            AllocationInstructionAlert::FIX50SP1(m) => m.serialize(serializer),
            AllocationInstructionAlert::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for AllocationInstructionAlert {
    fn get_header(&self) -> &Header {
        match self {
            AllocationInstructionAlert::FIX50(m) => m.get_header(),
            AllocationInstructionAlert::FIX50SP1(m) => m.get_header(),
            AllocationInstructionAlert::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            AllocationInstructionAlert::FIX50(m) => m.get_header_mut(),
            AllocationInstructionAlert::FIX50SP1(m) => m.get_header_mut(),
            AllocationInstructionAlert::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::allocation_instruction_alert::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::allocation_instruction_alert::Allocation {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::allocation_instruction_alert::AllocationInstructionAlert {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ExecutioNAcknowledgement {
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::execution_ack::ExecutionAck>),
}

impl Serialize for ExecutioNAcknowledgement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ExecutioNAcknowledgement::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ExecutioNAcknowledgement {
    fn get_header(&self) -> &Header {
        match self {
            ExecutioNAcknowledgement::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ExecutioNAcknowledgement::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp2::messages::execution_ack::ExecutionAck {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ContraryIntentionReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::contrary_intention_report::Contrary>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::contrary_intention_report::Contrary>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::contrary_intention_report::Contrary>),
}

impl Serialize for ContraryIntentionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ContraryIntentionReport::FIX50(m) => m.serialize(serializer),
            ContraryIntentionReport::FIX50SP1(m) => m.serialize(serializer),
            ContraryIntentionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ContraryIntentionReport {
    fn get_header(&self) -> &Header {
        match self {
            ContraryIntentionReport::FIX50(m) => m.get_header(),
            ContraryIntentionReport::FIX50SP1(m) => m.get_header(),
            ContraryIntentionReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ContraryIntentionReport::FIX50(m) => m.get_header_mut(),
            ContraryIntentionReport::FIX50SP1(m) => m.get_header_mut(),
            ContraryIntentionReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::contrary_intention_report::Contrary {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::contrary_intention_report::Contrary {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::contrary_intention_report::Contrary {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SecurityDefinitionUpdateReport {
    /// FIX50
    #[serde(rename = "7")]
    FIX50(Box<fix50::messages::security_definition_update_report::Security>),
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::security_definition_update_report::Security>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::security_definition_update_report::Security>),
}

impl Serialize for SecurityDefinitionUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SecurityDefinitionUpdateReport::FIX50(m) => m.serialize(serializer),
            SecurityDefinitionUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            SecurityDefinitionUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SecurityDefinitionUpdateReport {
    fn get_header(&self) -> &Header {
        match self {
            SecurityDefinitionUpdateReport::FIX50(m) => m.get_header(),
            SecurityDefinitionUpdateReport::FIX50SP1(m) => m.get_header(),
            SecurityDefinitionUpdateReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SecurityDefinitionUpdateReport::FIX50(m) => m.get_header_mut(),
            SecurityDefinitionUpdateReport::FIX50SP1(m) => m.get_header_mut(),
            SecurityDefinitionUpdateReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50::messages::security_definition_update_report::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp1::messages::security_definition_update_report::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::security_definition_update_report::Security {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum SettlementObligationReport {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::settlement_obligation_report::Settlement>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::settlement_obligation_report::Settlement>),
}

impl Serialize for SettlementObligationReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SettlementObligationReport::FIX50SP1(m) => m.serialize(serializer),
            SettlementObligationReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for SettlementObligationReport {
    fn get_header(&self) -> &Header {
        match self {
            SettlementObligationReport::FIX50SP1(m) => m.get_header(),
            SettlementObligationReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            SettlementObligationReport::FIX50SP1(m) => m.get_header_mut(),
            SettlementObligationReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::settlement_obligation_report::Settlement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::settlement_obligation_report::Settlement {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum DerivativeSecurityListUpdateReport {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::derivative_security_list_update_report::Derivative>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::derivative_security_list_update_report::Derivative>),
}

impl Serialize for DerivativeSecurityListUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DerivativeSecurityListUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            DerivativeSecurityListUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for DerivativeSecurityListUpdateReport {
    fn get_header(&self) -> &Header {
        match self {
            DerivativeSecurityListUpdateReport::FIX50SP1(m) => m.get_header(),
            DerivativeSecurityListUpdateReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            DerivativeSecurityListUpdateReport::FIX50SP1(m) => m.get_header_mut(),
            DerivativeSecurityListUpdateReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::derivative_security_list_update_report::Derivative {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::derivative_security_list_update_report::Derivative {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum TradingSessionListUpdateReport {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::trading_session_list_update_report::Trading>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::trading_session_list_update_report::Trading>),
}

impl Serialize for TradingSessionListUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TradingSessionListUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            TradingSessionListUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for TradingSessionListUpdateReport {
    fn get_header(&self) -> &Header {
        match self {
            TradingSessionListUpdateReport::FIX50SP1(m) => m.get_header(),
            TradingSessionListUpdateReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            TradingSessionListUpdateReport::FIX50SP1(m) => m.get_header_mut(),
            TradingSessionListUpdateReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::trading_session_list_update_report::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::trading_session_list_update_report::Trading {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDefinitionRequest {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_definition_request::Market>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_definition_request::Market>),
}

impl Serialize for MarketDefinitionRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MarketDefinitionRequest::FIX50SP1(m) => m.serialize(serializer),
            MarketDefinitionRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MarketDefinitionRequest {
    fn get_header(&self) -> &Header {
        match self {
            MarketDefinitionRequest::FIX50SP1(m) => m.get_header(),
            MarketDefinitionRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MarketDefinitionRequest::FIX50SP1(m) => m.get_header_mut(),
            MarketDefinitionRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::market_definition_request::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::market_definition_request::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDefinition {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_definition::Market>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_definition::MarketDefinition>),
}

impl Serialize for MarketDefinition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MarketDefinition::FIX50SP1(m) => m.serialize(serializer),
            MarketDefinition::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MarketDefinition {
    fn get_header(&self) -> &Header {
        match self {
            MarketDefinition::FIX50SP1(m) => m.get_header(),
            MarketDefinition::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MarketDefinition::FIX50SP1(m) => m.get_header_mut(),
            MarketDefinition::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::market_definition::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::market_definition::MarketDefinition {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum MarketDefinitionUpdateReport {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::market_definition_update_report::Market>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::market_definition_update_report::Market>),
}

impl Serialize for MarketDefinitionUpdateReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MarketDefinitionUpdateReport::FIX50SP1(m) => m.serialize(serializer),
            MarketDefinitionUpdateReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for MarketDefinitionUpdateReport {
    fn get_header(&self) -> &Header {
        match self {
            MarketDefinitionUpdateReport::FIX50SP1(m) => m.get_header(),
            MarketDefinitionUpdateReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            MarketDefinitionUpdateReport::FIX50SP1(m) => m.get_header_mut(),
            MarketDefinitionUpdateReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::market_definition_update_report::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::market_definition_update_report::Market {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ApplicationMessageRequest {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::application_message_request::Application>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::application_message_request::Application>),
}

impl Serialize for ApplicationMessageRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ApplicationMessageRequest::FIX50SP1(m) => m.serialize(serializer),
            ApplicationMessageRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ApplicationMessageRequest {
    fn get_header(&self) -> &Header {
        match self {
            ApplicationMessageRequest::FIX50SP1(m) => m.get_header(),
            ApplicationMessageRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ApplicationMessageRequest::FIX50SP1(m) => m.get_header_mut(),
            ApplicationMessageRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::application_message_request::Application {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::application_message_request::Application {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ApplicationMessageRequestAck {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::application_message_request_ack::Application>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::application_message_request_ack::Application>),
}

impl Serialize for ApplicationMessageRequestAck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ApplicationMessageRequestAck::FIX50SP1(m) => m.serialize(serializer),
            ApplicationMessageRequestAck::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ApplicationMessageRequestAck {
    fn get_header(&self) -> &Header {
        match self {
            ApplicationMessageRequestAck::FIX50SP1(m) => m.get_header(),
            ApplicationMessageRequestAck::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ApplicationMessageRequestAck::FIX50SP1(m) => m.get_header_mut(),
            ApplicationMessageRequestAck::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::application_message_request_ack::Application {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::application_message_request_ack::Application {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum ApplicationMessageReport {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::application_message_report::Application>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::application_message_report::Application>),
}

impl Serialize for ApplicationMessageReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ApplicationMessageReport::FIX50SP1(m) => m.serialize(serializer),
            ApplicationMessageReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for ApplicationMessageReport {
    fn get_header(&self) -> &Header {
        match self {
            ApplicationMessageReport::FIX50SP1(m) => m.get_header(),
            ApplicationMessageReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            ApplicationMessageReport::FIX50SP1(m) => m.get_header_mut(),
            ApplicationMessageReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::application_message_report::Application {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::application_message_report::Application {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassActionReport {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_action_report::Order>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_action_report::Order>),
}

impl Serialize for OrderMassActionReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrderMassActionReport::FIX50SP1(m) => m.serialize(serializer),
            OrderMassActionReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for OrderMassActionReport {
    fn get_header(&self) -> &Header {
        match self {
            OrderMassActionReport::FIX50SP1(m) => m.get_header(),
            OrderMassActionReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            OrderMassActionReport::FIX50SP1(m) => m.get_header_mut(),
            OrderMassActionReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::order_mass_action_report::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::order_mass_action_report::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum OrderMassActionRequest {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::order_mass_action_request::Order>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::order_mass_action_request::Order>),
}

impl Serialize for OrderMassActionRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            OrderMassActionRequest::FIX50SP1(m) => m.serialize(serializer),
            OrderMassActionRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for OrderMassActionRequest {
    fn get_header(&self) -> &Header {
        match self {
            OrderMassActionRequest::FIX50SP1(m) => m.get_header(),
            OrderMassActionRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            OrderMassActionRequest::FIX50SP1(m) => m.get_header_mut(),
            OrderMassActionRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::order_mass_action_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::order_mass_action_request::Order {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum UserNotification {
    /// FIX50SP1
    #[serde(rename = "8")]
    FIX50SP1(Box<fix50sp1::messages::user_notification::User>),
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::user_notification::User>),
}

impl Serialize for UserNotification {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            UserNotification::FIX50SP1(m) => m.serialize(serializer),
            UserNotification::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for UserNotification {
    fn get_header(&self) -> &Header {
        match self {
            UserNotification::FIX50SP1(m) => m.get_header(),
            UserNotification::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            UserNotification::FIX50SP1(m) => m.get_header_mut(),
            UserNotification::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp1::messages::user_notification::User {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

impl HasHeader for fix50sp2::messages::user_notification::User {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum StreamAssignmentRequest {
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::stream_assignment_request::Stream>),
}

impl Serialize for StreamAssignmentRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StreamAssignmentRequest::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for StreamAssignmentRequest {
    fn get_header(&self) -> &Header {
        match self {
            StreamAssignmentRequest::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            StreamAssignmentRequest::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp2::messages::stream_assignment_request::Stream {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum StreamAssignmentReport {
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::stream_assignment_report::Stream>),
}

impl Serialize for StreamAssignmentReport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StreamAssignmentReport::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for StreamAssignmentReport {
    fn get_header(&self) -> &Header {
        match self {
            StreamAssignmentReport::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            StreamAssignmentReport::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp2::messages::stream_assignment_report::Stream {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "1128")]
pub enum StreamAssignmentReportACK {
    /// FIX50SP2
    #[serde(rename = "9")]
    FIX50SP2(Box<fix50sp2::messages::stream_assignment_report_ack::Stream>),
}

impl Serialize for StreamAssignmentReportACK {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StreamAssignmentReportACK::FIX50SP2(m) => m.serialize(serializer),
        }
    }
}

impl HasHeader for StreamAssignmentReportACK {
    fn get_header(&self) -> &Header {
        match self {
            StreamAssignmentReportACK::FIX50SP2(m) => m.get_header(),
        }
    }
    fn get_header_mut(&mut self) -> &mut Header {
        match self {
            StreamAssignmentReportACK::FIX50SP2(m) => m.get_header_mut(),
        }
    }
}

impl HasHeader for fix50sp2::messages::stream_assignment_report_ack::Stream {
    fn get_header(&self) -> &Header {
        &self.standard_message_header
    }
    fn get_header_mut(&mut self) -> &mut Header {
        &mut self.standard_message_header
    }
}

#[cfg(test)]
mod test {
    use super::Message;
    use fixt11::header::{MsgType, HasHeader};

    #[test]
    fn logon() {
        let msg = "8=FIXT.1.1\u{1}9=111\u{1}35=A\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}212=10\u{1}213=0123456789\u{1}369=1\u{1}98=0\u{1}108=1\u{1}789=1\u{1}1137=0\u{1}10=073\u{1}";
        let mut obj = dbg!(serde_fix::from_str_checked::<Message>(msg)).unwrap();
        obj.get_header_mut().msg_type = Some(MsgType::Logon);
        obj.get_header_mut().body_length = 0;
        assert_eq!(serde_fix::to_string_checked(&obj), Ok(msg.to_owned()));
    }
}
