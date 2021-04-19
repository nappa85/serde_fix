
use serde::{Deserialize, Serialize};

pub use fix44::*;

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "35")]
pub enum Message {
	/// Heartbeat
	#[serde(rename = "0")]
	Heartbeat(Box<fix44::messages::heartbeat::Heartbeat>),
	/// Test Request
	#[serde(rename = "1")]
	TestRequest(Box<fix44::messages::test_request::TestRequest>),
	/// Resend Request
	#[serde(rename = "2")]
	ResendRequest(Box<fix44::messages::resend_request::ResendRequest>),
	/// Reject
	#[serde(rename = "3")]
	Reject(Box<fix44::messages::reject::Reject>),
	/// Sequence Reset
	#[serde(rename = "4")]
	SequenceReset(Box<fix44::messages::sequence_reset::SequenceReset>),
	/// Logout
	#[serde(rename = "5")]
	Logout(Box<fix44::messages::logout::Logout>),
	/// Indication of Interest
	#[serde(rename = "6")]
	IndicationOfInterest(Box<fix44::messages::indication_of_interest::IndicationOfInterest>),
	/// Advertisement
	#[serde(rename = "7")]
	Advertisement(Box<fix44::messages::advertisement::Advertisement>),
	/// Execution Report
	#[serde(rename = "8")]
	ExecutionReport(Box<fix44::messages::execution_report::ExecutionReport>),
	/// Order Cancel Reject
	#[serde(rename = "9")]
	OrderCancelReject(Box<fix44::messages::order_cancel_reject::OrderCancelReject>),
	/// Logon
	#[serde(rename = "A")]
	Logon(Box<fix44::messages::logon::Logon>),
	/// News
	#[serde(rename = "B")]
	News(Box<fix44::messages::news::News>),
	/// Email
	#[serde(rename = "C")]
	Email(Box<fix44::messages::email::Email>),
	/// New Order - Single
	#[serde(rename = "D")]
	NewOrderSingle(Box<fix44::messages::new_order_single::NewOrderSingle>),
	/// New Order - List
	#[serde(rename = "E")]
	NewOrderList(Box<fix44::messages::new_order_list::NewOrderList>),
	/// Order Cancel Request
	#[serde(rename = "F")]
	OrderCancelRequest(Box<fix44::messages::order_cancel_request::OrderCancelRequest>),
	/// Order Cancel/Replace Request
	#[serde(rename = "G")]
	OrderCancelReplaceRequest(Box<fix44::messages::order_cancel_replace_request::OrderCancelReplaceRequest>),
	/// Order Status Request
	#[serde(rename = "H")]
	OrderStatusRequest(Box<fix44::messages::order_status_request::OrderStatusRequest>),
	/// Allocation
	#[serde(rename = "J")]
	AllocationInstruction(Box<fix44::messages::allocation_instruction::AllocationInstruction>),
	/// List Cancel Request
	#[serde(rename = "K")]
	ListCancelRequest(Box<fix44::messages::list_cancel_request::ListCancelRequest>),
	/// List Execute
	#[serde(rename = "L")]
	ListExecute(Box<fix44::messages::list_execute::ListExecute>),
	/// List Status Request
	#[serde(rename = "M")]
	ListStatusRequest(Box<fix44::messages::list_status_request::ListStatusRequest>),
	/// List Status
	#[serde(rename = "N")]
	ListStatus(Box<fix44::messages::list_status::ListStatus>),
	/// Allocation ACK
	#[serde(rename = "P")]
	AllocationInstructionAck(Box<fix44::messages::allocation_instruction_ack::AllocationInstructionAck>),
	/// Don't Know Trade
	#[serde(rename = "Q")]
	DontKnowTrade(Box<fix44::messages::dont_know_trade::DontKnowTrade>),
	/// Quote Request
	#[serde(rename = "R")]
	QuoteRequest(Box<fix44::messages::quote_request::QuoteRequest>),
	/// Quote
	#[serde(rename = "S")]
	Quote(Box<fix44::messages::quote::Quote>),
	/// Settlement Instructions
	#[serde(rename = "T")]
	SettlementInstructions(Box<fix44::messages::settlement_instructions::SettlementInstructions>),
	/// Market Data Request
	#[serde(rename = "V")]
	MarketDataRequest(Box<fix44::messages::market_data_request::MarketDataRequest>),
	/// Market Data - Snapshot/Full Refresh
	#[serde(rename = "W")]
	MarketDataSnapshotFullRefresh(Box<fix44::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh>),
	/// Market Data - Incremental Refresh
	#[serde(rename = "X")]
	MarketDataIncrementalRefresh(Box<fix44::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh>),
	/// Market Data Request Reject
	#[serde(rename = "Y")]
	MarketDataRequestReject(Box<fix44::messages::market_data_request_reject::MarketDataRequestReject>),
	/// Quote Cancel
	#[serde(rename = "Z")]
	QuoteCancel(Box<fix44::messages::quote_cancel::QuoteCancel>),
	/// Quote Status Request
	#[serde(rename = "a")]
	QuoteStatusRequest(Box<fix44::messages::quote_status_request::QuoteStatusRequest>),
	/// Quote Acknowledgement
	#[serde(rename = "b")]
	MassQuoteAcknowledgement(Box<fix44::messages::mass_quote_acknowledgement::MassQuoteAcknowledgement>),
	/// Security Definition Request
	#[serde(rename = "c")]
	SecurityDefinitionRequest(Box<fix44::messages::security_definition_request::SecurityDefinitionRequest>),
	/// Security Definition
	#[serde(rename = "d")]
	SecurityDefinition(Box<fix44::messages::security_definition::SecurityDefinition>),
	/// Security Status Request
	#[serde(rename = "e")]
	SecurityStatusRequest(Box<fix44::messages::security_status_request::SecurityStatusRequest>),
	/// Security Status
	#[serde(rename = "f")]
	SecurityStatus(Box<fix44::messages::security_status::SecurityStatus>),
	/// Trading Session Status Request
	#[serde(rename = "g")]
	TradingSessionStatusRequest(Box<fix44::messages::trading_session_status_request::TradingSessionStatusRequest>),
	/// Trading Session Status
	#[serde(rename = "h")]
	TradingSessionStatus(Box<fix44::messages::trading_session_status::TradingSessionStatus>),
	/// Mass Quote
	#[serde(rename = "i")]
	MassQuote(Box<fix44::messages::mass_quote::MassQuote>),
	/// Business Message Reject
	#[serde(rename = "j")]
	BusinessMessageReject(Box<fix44::messages::business_message_reject::BusinessMessageReject>),
	/// Bid Request
	#[serde(rename = "k")]
	BidRequest(Box<fix44::messages::bid_request::BidRequest>),
	/// Bid Response
	#[serde(rename = "l")]
	BidResponse(Box<fix44::messages::bid_response::BidResponse>),
	/// List Strike Price
	#[serde(rename = "m")]
	ListStrikePrice(Box<fix44::messages::list_strike_price::ListStrikePrice>),
	/// XML message
	#[serde(rename = "n")]
	XmlMessage(Box<fix44::messages::xml_message::XmlMessage>),
	/// Registration Instructions
	#[serde(rename = "o")]
	RegistrationInstructions(Box<fix44::messages::registration_instructions::RegistrationInstructions>),
	/// Registration Instructions Response
	#[serde(rename = "p")]
	RegistrationInstructionsResponse(Box<fix44::messages::registration_instructions_response::RegistrationInstructionsResponse>),
	/// Order Mass Cancel Request
	#[serde(rename = "q")]
	OrderMassCancelRequest(Box<fix44::messages::order_mass_cancel_request::OrderMassCancelRequest>),
	/// Order Mass Cancel Report
	#[serde(rename = "r")]
	OrderMassCancelReport(Box<fix44::messages::order_mass_cancel_report::OrderMassCancelReport>),
	/// New Order - Cross
	#[serde(rename = "s")]
	NewOrderCross(Box<fix44::messages::new_order_cross::NewOrderCross>),
	/// Cross Order Cancel/Replace Request
	#[serde(rename = "t")]
	CrossOrderCancelReplaceRequest(Box<fix44::messages::cross_order_cancel_replace_request::CrossOrderCancelReplaceRequest>),
	/// Cross Order Cancel Request
	#[serde(rename = "u")]
	CrossOrderCancelRequest(Box<fix44::messages::cross_order_cancel_request::CrossOrderCancelRequest>),
	/// Security Type Request
	#[serde(rename = "v")]
	SecurityTypeRequest(Box<fix44::messages::security_type_request::SecurityTypeRequest>),
	/// Security Types
	#[serde(rename = "w")]
	SecurityTypes(Box<fix44::messages::security_types::SecurityTypes>),
	/// Security List Request
	#[serde(rename = "x")]
	SecurityListRequest(Box<fix44::messages::security_list_request::SecurityListRequest>),
	/// Security List
	#[serde(rename = "y")]
	SecurityList(Box<fix44::messages::security_list::SecurityList>),
	/// Derivative Security List Request
	#[serde(rename = "z")]
	DerivativeSecurityListRequest(Box<fix44::messages::derivative_security_list_request::DerivativeSecurityListRequest>),
	/// Derivative Security List
	#[serde(rename = "AA")]
	DerivativeSecurityList(Box<fix44::messages::derivative_security_list::DerivativeSecurityList>),
	/// New Order - Multileg
	#[serde(rename = "AB")]
	NewOrderMultileg(Box<fix44::messages::new_order_multileg::NewOrderMultileg>),
	/// Multileg Order Cancel/Replace Request
	#[serde(rename = "AC")]
	MultilegOrderCancelReplaceRequest(Box<fix44::messages::multileg_order_cancel_replace_request::MultilegOrderCancelReplaceRequest>),
	/// Trade Capture Report Request
	#[serde(rename = "AD")]
	TradeCaptureReportRequest(Box<fix44::messages::trade_capture_report_request::TradeCaptureReportRequest>),
	/// Trade Capture Report
	#[serde(rename = "AE")]
	TradeCaptureReport(Box<fix44::messages::trade_capture_report::TradeCaptureReport>),
	/// Order Mass Status Request
	#[serde(rename = "AF")]
	OrderMassStatusRequest(Box<fix44::messages::order_mass_status_request::OrderMassStatusRequest>),
	/// Quote Request Reject
	#[serde(rename = "AG")]
	QuoteRequestReject(Box<fix44::messages::quote_request_reject::QuoteRequestReject>),
	/// RFQ Request
	#[serde(rename = "AH")]
	RfqRequest(Box<fix44::messages::rfq_request::RfqRequest>),
	/// Quote Status Report
	#[serde(rename = "AI")]
	QuoteStatusReport(Box<fix44::messages::quote_status_report::QuoteStatusReport>),
	/// Quote Response
	#[serde(rename = "AJ")]
	QuoteResponse(Box<fix44::messages::quote_response::QuoteResponse>),
	/// Confirmation
	#[serde(rename = "AK")]
	Confirmation(Box<fix44::messages::confirmation::Confirmation>),
	/// Position Maintenance Request
	#[serde(rename = "AL")]
	PositionMaintenanceRequest(Box<fix44::messages::position_maintenance_request::PositionMaintenanceRequest>),
	/// Position Maintenance Report
	#[serde(rename = "AM")]
	PositionMaintenanceReport(Box<fix44::messages::position_maintenance_report::PositionMaintenanceReport>),
	/// Request for Positions
	#[serde(rename = "AN")]
	RequestForPositions(Box<fix44::messages::request_for_positions::RequestForPositions>),
	/// Request for Positions Ack
	#[serde(rename = "AO")]
	RequestForPositionsAck(Box<fix44::messages::request_for_positions_ack::RequestForPositionsAck>),
	/// Position Report
	#[serde(rename = "AP")]
	PositionReport(Box<fix44::messages::position_report::PositionReport>),
	/// Trade Capture Report Request Ack
	#[serde(rename = "AQ")]
	TradeCaptureReportRequestAck(Box<fix44::messages::trade_capture_report_request_ack::TradeCaptureReportRequestAck>),
	/// Trade Capture Report Ack
	#[serde(rename = "AR")]
	TradeCaptureReportAck(Box<fix44::messages::trade_capture_report_ack::TradeCaptureReportAck>),
	/// Allocation Report
	#[serde(rename = "AS")]
	AllocationReport(Box<fix44::messages::allocation_report::AllocationReport>),
	/// Allocation Report Ack
	#[serde(rename = "AT")]
	AllocationReportAck(Box<fix44::messages::allocation_report_ack::AllocationReportAck>),
	/// Confirmation Ack
	#[serde(rename = "AU")]
	ConfirmationAck(Box<fix44::messages::confirmation_ack::ConfirmationAck>),
	/// Settlement Instruction Request
	#[serde(rename = "AV")]
	SettlementInstructionRequest(Box<fix44::messages::settlement_instruction_request::SettlementInstructionRequest>),
	/// Assignment Report
	#[serde(rename = "AW")]
	AssignmentReport(Box<fix44::messages::assignment_report::AssignmentReport>),
	/// Collateral Request
	#[serde(rename = "AX")]
	CollateralRequest(Box<fix44::messages::collateral_request::CollateralRequest>),
	/// Collateral Assignment
	#[serde(rename = "AY")]
	CollateralAssignment(Box<fix44::messages::collateral_assignment::CollateralAssignment>),
	/// Collateral Response
	#[serde(rename = "AZ")]
	CollateralResponse(Box<fix44::messages::collateral_response::CollateralResponse>),
	/// Collateral Report
	#[serde(rename = "BA")]
	CollateralReport(Box<fix44::messages::collateral_report::CollateralReport>),
	/// Collateral Inquiry
	#[serde(rename = "BB")]
	CollateralInquiry(Box<fix44::messages::collateral_inquiry::CollateralInquiry>),
	/// Network Status Request
	#[serde(rename = "BC")]
	NetworkCounterpartySystemStatusRequest(Box<fix44::messages::network_counterparty_system_status_request::NetworkCounterpartySystemStatusRequest>),
	/// Network Status Response
	#[serde(rename = "BD")]
	NetworkCounterpartySystemStatusResponse(Box<fix44::messages::network_counterparty_system_status_response::NetworkCounterpartySystemStatusResponse>),
	/// User Request
	#[serde(rename = "BE")]
	UserRequest(Box<fix44::messages::user_request::UserRequest>),
	/// User Response
	#[serde(rename = "BF")]
	UserResponse(Box<fix44::messages::user_response::UserResponse>),
	/// Collateral Inquiry Ack
	#[serde(rename = "BG")]
	CollateralInquiryAck(Box<fix44::messages::collateral_inquiry_ack::CollateralInquiryAck>),
	/// Confirmation Request
	#[serde(rename = "BH")]
	ConfirmationRequest(Box<fix44::messages::confirmation_request::ConfirmationRequest>),
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
            Message::MultilegOrderCancelReplaceRequest(m) => m.serialize(serializer),
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
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Message;

    #[test]
    fn logon() {
        let msg = "8=FIX.4.4\u{1}9=104\u{1}35=A\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}212=10\u{1}213=0123456789\u{1}369=1\u{1}98=0\u{1}108=1\u{1}789=1\u{1}10=195\u{1}";
        let mut obj = dbg!(serde_fix::from_str_checked::<Message>(msg)).unwrap();
        match obj {
            Message::FIX44(super::Message::Logon(ref mut l)) => {
                l.standard_message_header.body_length = 0;
            },
            _ => panic!("Deserialized message is not of type Logon"),
        }
        assert_eq!(serde_fix::to_string_checked(&obj), Ok(msg.to_owned()));
    }
}
