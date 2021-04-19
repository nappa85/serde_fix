
use serde::{Deserialize, Serialize};

pub use fix43::*;

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "35")]
pub enum Message {
	/// Heartbeat
	#[serde(rename = "0")]
	Heartbeat(Box<fix43::messages::heartbeat::Heartbeat>),
	/// Test Request
	#[serde(rename = "1")]
	TestRequest(Box<fix43::messages::test_request::TestRequest>),
	/// Resend Request
	#[serde(rename = "2")]
	ResendRequest(Box<fix43::messages::resend_request::ResendRequest>),
	/// Reject
	#[serde(rename = "3")]
	Reject(Box<fix43::messages::reject::Reject>),
	/// Sequence Reset
	#[serde(rename = "4")]
	SequenceReset(Box<fix43::messages::sequence_reset::SequenceReset>),
	/// Logout
	#[serde(rename = "5")]
	Logout(Box<fix43::messages::logout::Logout>),
	/// Indication of Interest
	#[serde(rename = "6")]
	IndicationOfInterest(Box<fix43::messages::indication_of_interest::IndicationOfInterest>),
	/// Advertisement
	#[serde(rename = "7")]
	Advertisement(Box<fix43::messages::advertisement::Advertisement>),
	/// Execution Report
	#[serde(rename = "8")]
	ExecutionReport(Box<fix43::messages::execution_report::ExecutionReport>),
	/// Order Cancel Reject
	#[serde(rename = "9")]
	OrderCancelReject(Box<fix43::messages::order_cancel_reject::OrderCancelReject>),
	/// Logon
	#[serde(rename = "A")]
	Logon(Box<fix43::messages::logon::Logon>),
	/// News
	#[serde(rename = "B")]
	News(Box<fix43::messages::news::News>),
	/// Email
	#[serde(rename = "C")]
	Email(Box<fix43::messages::email::Email>),
	/// New Order - Single
	#[serde(rename = "D")]
	NewOrderSingle(Box<fix43::messages::new_order_single::NewOrderSingle>),
	/// New Order - List
	#[serde(rename = "E")]
	NewOrderList(Box<fix43::messages::new_order_list::NewOrderList>),
	/// Order Cancel Request
	#[serde(rename = "F")]
	OrderCancelRequest(Box<fix43::messages::order_cancel_request::OrderCancelRequest>),
	/// Order Cancel/Replace Request
	#[serde(rename = "G")]
	OrderCancelReplaceRequest(Box<fix43::messages::order_cancel_replace_request::OrderCancelReplaceRequest>),
	/// Order Status Request
	#[serde(rename = "H")]
	OrderStatusRequest(Box<fix43::messages::order_status_request::OrderStatusRequest>),
	/// Allocation
	#[serde(rename = "J")]
	Allocation(Box<fix43::messages::allocation::Allocation>),
	/// List Cancel Request
	#[serde(rename = "K")]
	ListCancelRequest(Box<fix43::messages::list_cancel_request::ListCancelRequest>),
	/// List Execute
	#[serde(rename = "L")]
	ListExecute(Box<fix43::messages::list_execute::ListExecute>),
	/// List Status Request
	#[serde(rename = "M")]
	ListStatusRequest(Box<fix43::messages::list_status_request::ListStatusRequest>),
	/// List Status
	#[serde(rename = "N")]
	ListStatus(Box<fix43::messages::list_status::ListStatus>),
	/// Allocation ACK
	#[serde(rename = "P")]
	AllocationAck(Box<fix43::messages::allocation_ack::AllocationAck>),
	/// Don't Know Trade
	#[serde(rename = "Q")]
	DontKnowTrade(Box<fix43::messages::dont_know_trade::DontKnowTrade>),
	/// Quote Request
	#[serde(rename = "R")]
	QuoteRequest(Box<fix43::messages::quote_request::QuoteRequest>),
	/// Quote
	#[serde(rename = "S")]
	Quote(Box<fix43::messages::quote::Quote>),
	/// Settlement Instructions
	#[serde(rename = "T")]
	SettlementInstructions(Box<fix43::messages::settlement_instructions::SettlementInstructions>),
	/// Market Data Request
	#[serde(rename = "V")]
	MarketDataRequest(Box<fix43::messages::market_data_request::MarketDataRequest>),
	/// Market Data - Snapshot/Full Refresh
	#[serde(rename = "W")]
	MarketDataSnapshotFullRefresh(Box<fix43::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh>),
	/// Market Data - Incremental Refresh
	#[serde(rename = "X")]
	MarketDataIncrementalRefresh(Box<fix43::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh>),
	/// Market Data Request Reject
	#[serde(rename = "Y")]
	MarketDataRequestReject(Box<fix43::messages::market_data_request_reject::MarketDataRequestReject>),
	/// Quote Cancel
	#[serde(rename = "Z")]
	QuoteCancel(Box<fix43::messages::quote_cancel::QuoteCancel>),
	/// Quote Status Request
	#[serde(rename = "a")]
	QuoteStatusRequest(Box<fix43::messages::quote_status_request::QuoteStatusRequest>),
	/// Mass Quote Acknowledgement
	#[serde(rename = "b")]
	QuoteAcknowledgement(Box<fix43::messages::mass_quote_acknowledgement::MassQuoteAcknowledgement>),
	/// Security Definition Request
	#[serde(rename = "c")]
	SecurityDefinitionRequest(Box<fix43::messages::security_definition_request::SecurityDefinitionRequest>),
	/// Security Definition
	#[serde(rename = "d")]
	SecurityDefinition(Box<fix43::messages::security_definition::SecurityDefinition>),
	/// Security Status Request
	#[serde(rename = "e")]
	SecurityStatusRequest(Box<fix43::messages::security_status_request::SecurityStatusRequest>),
	/// Security Status
	#[serde(rename = "f")]
	SecurityStatus(Box<fix43::messages::security_status::SecurityStatus>),
	/// Trading Session Status Request
	#[serde(rename = "g")]
	TradingSessionStatusRequest(Box<fix43::messages::trading_session_status_request::TradingSessionStatusRequest>),
	/// Trading Session Status
	#[serde(rename = "h")]
	TradingSessionStatus(Box<fix43::messages::trading_session_status::TradingSessionStatus>),
	/// Mass Quote
	#[serde(rename = "i")]
	MassQuote(Box<fix43::messages::mass_quote::MassQuote>),
	/// Business Message Reject
	#[serde(rename = "j")]
	BusinessMessageReject(Box<fix43::messages::business_message_reject::BusinessMessageReject>),
	/// Bid Request
	#[serde(rename = "k")]
	BidRequest(Box<fix43::messages::bid_request::BidRequest>),
	/// Bid Response
	#[serde(rename = "l")]
	BidResponse(Box<fix43::messages::bid_response::BidResponse>),
	/// List Strike Price
	#[serde(rename = "m")]
	ListStrikePrice(Box<fix43::messages::list_strike_price::ListStrikePrice>),
	/// XML message
	#[serde(rename = "n")]
	XmlMessage(Box<fix43::messages::xml_message::XmlMessage>),
	/// Registration Instructions
	#[serde(rename = "o")]
	RegistrationInstructions(Box<fix43::messages::registration_instructions::RegistrationInstructions>),
	/// Registration Instructions Response
	#[serde(rename = "p")]
	RegistrationInstructionsResponse(Box<fix43::messages::registration_instructions_response::RegistrationInstructionsResponse>),
	/// Order Mass Cancel Request
	#[serde(rename = "q")]
	OrderMassCancelRequest(Box<fix43::messages::order_mass_cancel_request::OrderMassCancelRequest>),
	/// Order Mass Cancel Report
	#[serde(rename = "r")]
	OrderMassCancelReport(Box<fix43::messages::order_mass_cancel_report::OrderMassCancelReport>),
	/// New Order - Cross
	#[serde(rename = "s")]
	NewOrderCross(Box<fix43::messages::new_order_cross::NewOrderCross>),
	/// Cross Order Cancel/Replace Request
	#[serde(rename = "t")]
	CrossOrderCancelReplaceRequest(Box<fix43::messages::cross_order_cancel_replace_request::CrossOrderCancelReplaceRequest>),
	/// Cross Order Cancel Request
	#[serde(rename = "u")]
	CrossOrderCancelRequest(Box<fix43::messages::cross_order_cancel_request::CrossOrderCancelRequest>),
	/// Security Type Request
	#[serde(rename = "v")]
	SecurityTypeRequest(Box<fix43::messages::security_type_request::SecurityTypeRequest>),
	/// Security Types
	#[serde(rename = "w")]
	SecurityTypes(Box<fix43::messages::security_types::SecurityTypes>),
	/// Security List Request
	#[serde(rename = "x")]
	SecurityListRequest(Box<fix43::messages::security_list_request::SecurityListRequest>),
	/// Security List
	#[serde(rename = "y")]
	SecurityList(Box<fix43::messages::security_list::SecurityList>),
	/// Derivative Security List Request
	#[serde(rename = "z")]
	DerivativeSecurityListRequest(Box<fix43::messages::derivative_security_list_request::DerivativeSecurityListRequest>),
	/// Derivative Security List
	#[serde(rename = "AA")]
	DerivativeSecurityList(Box<fix43::messages::derivative_security_list::DerivativeSecurityList>),
	/// New Order - Multileg
	#[serde(rename = "AB")]
	NewOrderMultileg(Box<fix43::messages::new_order_multileg::NewOrderMultileg>),
	/// Multileg Order Cancel/Replace Request
	#[serde(rename = "AC")]
	MultilegOrderCancelReplaceRequest(Box<fix43::messages::multileg_order_cancel_replace_request::MultilegOrderCancelReplaceRequest>),
	/// Trade Capture Report Request
	#[serde(rename = "AD")]
	TradeCaptureReportRequest(Box<fix43::messages::trade_capture_report_request::TradeCaptureReportRequest>),
	/// Trade Capture Report
	#[serde(rename = "AE")]
	TradeCaptureReport(Box<fix43::messages::trade_capture_report::TradeCaptureReport>),
	/// Order Mass Status Request
	#[serde(rename = "AF")]
	OrderMassStatusRequest(Box<fix43::messages::order_mass_status_request::OrderMassStatusRequest>),
	/// Quote Request Reject
	#[serde(rename = "AG")]
	QuoteRequestReject(Box<fix43::messages::quote_request_reject::QuoteRequestReject>),
	/// RFQ Request
	#[serde(rename = "AH")]
	RfqRequest(Box<fix43::messages::rfq_request::RfqRequest>),
	/// Quote Status Report
	#[serde(rename = "AI")]
	QuoteStatusReport(Box<fix43::messages::quote_status_report::QuoteStatusReport>),
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
            Message::Allocation(m) => m.serialize(serializer),
            Message::ListCancelRequest(m) => m.serialize(serializer),
            Message::ListExecute(m) => m.serialize(serializer),
            Message::ListStatusRequest(m) => m.serialize(serializer),
            Message::ListStatus(m) => m.serialize(serializer),
            Message::AllocationAck(m) => m.serialize(serializer),
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
            Message::QuoteAcknowledgement(m) => m.serialize(serializer),
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
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Message;

    #[test]
    fn logon() {
        let msg = "8=FIX.4.3\u{1}9=98\u{1}35=A\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}212=10\u{1}213=0123456789\u{1}369=1\u{1}98=0\u{1}108=1\u{1}10=135\u{1}";
        let mut obj = dbg!(serde_fix::from_str_checked::<Message>(msg)).unwrap();
        match obj {
            Message::FIX43(super::Message::Logon(ref mut l)) => {
                l.standard_message_header.body_length = 0;
            },
            _ => panic!("Deserialized message is not of type Logon"),
        }
        assert_eq!(serde_fix::to_string_checked(&obj), Ok(msg.to_owned()));
    }
}
