
use serde::{Deserialize, Serialize};

use fix_common::ApplVerID;

use crate::has_header::*;

pub use fix42::*;

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "35")]
pub enum Message {
	/// Heartbeat
	#[serde(rename = "0")]
	Heartbeat(Box<fix42::messages::heartbeat::Heartbeat>),
	/// Test Request
	#[serde(rename = "1")]
	TestRequest(Box<fix42::messages::test_request::TestRequest>),
	/// Resend Request
	#[serde(rename = "2")]
	ResendRequest(Box<fix42::messages::resend_request::ResendRequest>),
	/// Reject
	#[serde(rename = "3")]
	Reject(Box<fix42::messages::reject::Reject>),
	/// Sequence Reset
	#[serde(rename = "4")]
	SequenceReset(Box<fix42::messages::sequence_reset::SequenceReset>),
	/// Logout
	#[serde(rename = "5")]
	Logout(Box<fix42::messages::logout::Logout>),
	/// Indication of Interest
	#[serde(rename = "6")]
	IndicationOfInterest(Box<fix42::messages::indication_of_interest::IndicationOfInterest>),
	/// Advertisement
	#[serde(rename = "7")]
	Advertisement(Box<fix42::messages::advertisement::Advertisement>),
	/// Execution Report
	#[serde(rename = "8")]
	ExecutionReport(Box<fix42::messages::execution_report::ExecutionReport>),
	/// Order Cancel Reject
	#[serde(rename = "9")]
	OrderCancelReject(Box<fix42::messages::order_cancel_reject::OrderCancelReject>),
	/// Logon
	#[serde(rename = "A")]
	Logon(Box<fix42::messages::logon::Logon>),
	/// News
	#[serde(rename = "B")]
	News(Box<fix42::messages::news::News>),
	/// Email
	#[serde(rename = "C")]
	Email(Box<fix42::messages::email::Email>),
	/// New Order - Single
	#[serde(rename = "D")]
	NewOrderSingle(Box<fix42::messages::new_order_single::NewOrderSingle>),
	/// New Order - List
	#[serde(rename = "E")]
	NewOrderList(Box<fix42::messages::new_order_list::NewOrderList>),
	/// Order Cancel Request
	#[serde(rename = "F")]
	OrderCancelRequest(Box<fix42::messages::order_cancel_request::OrderCancelRequest>),
	/// Order Cancel/Replace Request
	#[serde(rename = "G")]
	OrderCancelReplaceRequest(Box<fix42::messages::order_cancel_replace_request::OrderCancelReplaceRequest>),
	/// Order Status Request
	#[serde(rename = "H")]
	OrderStatusRequest(Box<fix42::messages::order_status_request::OrderStatusRequest>),
	/// Allocation
	#[serde(rename = "J")]
	Allocation(Box<fix42::messages::allocation::Allocation>),
	/// List Cancel Request
	#[serde(rename = "K")]
	ListCancelRequest(Box<fix42::messages::list_cancel_request::ListCancelRequest>),
	/// List Execute
	#[serde(rename = "L")]
	ListExecute(Box<fix42::messages::list_execute::ListExecute>),
	/// List Status Request
	#[serde(rename = "M")]
	ListStatusRequest(Box<fix42::messages::list_status_request::ListStatusRequest>),
	/// List Status
	#[serde(rename = "N")]
	ListStatus(Box<fix42::messages::list_status::ListStatus>),
	/// Allocation ACK
	#[serde(rename = "P")]
	AllocationAck(Box<fix42::messages::allocation_ack::AllocationAck>),
	/// Don't Know Trade
	#[serde(rename = "Q")]
	DontKnowTrade(Box<fix42::messages::dont_know_trade::DontKnowTrade>),
	/// Quote Request
	#[serde(rename = "R")]
	QuoteRequest(Box<fix42::messages::quote_request::QuoteRequest>),
	/// Quote
	#[serde(rename = "S")]
	Quote(Box<fix42::messages::quote::Quote>),
	/// Settlement Instructions
	#[serde(rename = "T")]
	SettlementInstructions(Box<fix42::messages::settlement_instructions::SettlementInstructions>),
	/// Market Data Request
	#[serde(rename = "V")]
	MarketDataRequest(Box<fix42::messages::market_data_request::MarketDataRequest>),
	/// Market Data - Snapshot/Full Refresh
	#[serde(rename = "W")]
	MarketDataSnapshotFullRefresh(Box<fix42::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh>),
	/// Market Data - Incremental Refresh
	#[serde(rename = "X")]
	MarketDataIncrementalRefresh(Box<fix42::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh>),
	/// Market Data Request Reject
	#[serde(rename = "Y")]
	MarketDataRequestReject(Box<fix42::messages::market_data_request_reject::MarketDataRequestReject>),
	/// Quote Cancel
	#[serde(rename = "Z")]
	QuoteCancel(Box<fix42::messages::quote_cancel::QuoteCancel>),
	/// Quote Status Request
	#[serde(rename = "a")]
	QuoteStatusRequest(Box<fix42::messages::quote_status_request::QuoteStatusRequest>),
	/// Quote Acknowledgement
	#[serde(rename = "b")]
	QuoteAcknowledgement(Box<fix42::messages::quote_acknowledgement::QuoteAcknowledgement>),
	/// Security Definition Request
	#[serde(rename = "c")]
	SecurityDefinitionRequest(Box<fix42::messages::security_definition_request::SecurityDefinitionRequest>),
	/// Security Definition
	#[serde(rename = "d")]
	SecurityDefinition(Box<fix42::messages::security_definition::SecurityDefinition>),
	/// Security Status Request
	#[serde(rename = "e")]
	SecurityStatusRequest(Box<fix42::messages::security_status_request::SecurityStatusRequest>),
	/// Security Status
	#[serde(rename = "f")]
	SecurityStatus(Box<fix42::messages::security_status::SecurityStatus>),
	/// Trading Session Status Request
	#[serde(rename = "g")]
	TradingSessionStatusRequest(Box<fix42::messages::trading_session_status_request::TradingSessionStatusRequest>),
	/// Trading Session Status
	#[serde(rename = "h")]
	TradingSessionStatus(Box<fix42::messages::trading_session_status::TradingSessionStatus>),
	/// Mass Quote
	#[serde(rename = "i")]
	MassQuote(Box<fix42::messages::mass_quote::MassQuote>),
	/// Business Message Reject
	#[serde(rename = "j")]
	BusinessMessageReject(Box<fix42::messages::business_message_reject::BusinessMessageReject>),
	/// Bid Request
	#[serde(rename = "k")]
	BidRequest(Box<fix42::messages::bid_request::BidRequest>),
	/// Bid Response
	#[serde(rename = "l")]
	BidResponse(Box<fix42::messages::bid_response::BidResponse>),
	/// List Strike Price
	#[serde(rename = "m")]
	ListStrikePrice(Box<fix42::messages::list_strike_price::ListStrikePrice>),
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
            Message::IndicationOfInterest(m) => Box::new(m.get_header()),
            Message::Advertisement(m) => Box::new(m.get_header()),
            Message::ExecutionReport(m) => Box::new(m.get_header()),
            Message::OrderCancelReject(m) => Box::new(m.get_header()),
            Message::Logon(m) => Box::new(m.get_header()),
            Message::News(m) => Box::new(m.get_header()),
            Message::Email(m) => Box::new(m.get_header()),
            Message::NewOrderSingle(m) => Box::new(m.get_header()),
            Message::NewOrderList(m) => Box::new(m.get_header()),
            Message::OrderCancelRequest(m) => Box::new(m.get_header()),
            Message::OrderCancelReplaceRequest(m) => Box::new(m.get_header()),
            Message::OrderStatusRequest(m) => Box::new(m.get_header()),
            Message::Allocation(m) => Box::new(m.get_header()),
            Message::ListCancelRequest(m) => Box::new(m.get_header()),
            Message::ListExecute(m) => Box::new(m.get_header()),
            Message::ListStatusRequest(m) => Box::new(m.get_header()),
            Message::ListStatus(m) => Box::new(m.get_header()),
            Message::AllocationAck(m) => Box::new(m.get_header()),
            Message::DontKnowTrade(m) => Box::new(m.get_header()),
            Message::QuoteRequest(m) => Box::new(m.get_header()),
            Message::Quote(m) => Box::new(m.get_header()),
            Message::SettlementInstructions(m) => Box::new(m.get_header()),
            Message::MarketDataRequest(m) => Box::new(m.get_header()),
            Message::MarketDataSnapshotFullRefresh(m) => Box::new(m.get_header()),
            Message::MarketDataIncrementalRefresh(m) => Box::new(m.get_header()),
            Message::MarketDataRequestReject(m) => Box::new(m.get_header()),
            Message::QuoteCancel(m) => Box::new(m.get_header()),
            Message::QuoteStatusRequest(m) => Box::new(m.get_header()),
            Message::QuoteAcknowledgement(m) => Box::new(m.get_header()),
            Message::SecurityDefinitionRequest(m) => Box::new(m.get_header()),
            Message::SecurityDefinition(m) => Box::new(m.get_header()),
            Message::SecurityStatusRequest(m) => Box::new(m.get_header()),
            Message::SecurityStatus(m) => Box::new(m.get_header()),
            Message::TradingSessionStatusRequest(m) => Box::new(m.get_header()),
            Message::TradingSessionStatus(m) => Box::new(m.get_header()),
            Message::MassQuote(m) => Box::new(m.get_header()),
            Message::BusinessMessageReject(m) => Box::new(m.get_header()),
            Message::BidRequest(m) => Box::new(m.get_header()),
            Message::BidResponse(m) => Box::new(m.get_header()),
            Message::ListStrikePrice(m) => Box::new(m.get_header()),
        }
    }
}

impl<const T: char> crate::header::Header for fix42::standard_message_header::StandardMessageHeader<T> {
    fn get_sender_comp_id(&self) -> &str {
        &self.sender_comp_id
    }
    fn get_target_comp_id(&self) -> &str {
        &self.target_comp_id
    }
    fn get_msg_seq_num(&self) -> u32 {
        self.msg_seq_num
    }
}

impl<const T: char> crate::header::HeaderExt for fix42::standard_message_header::StandardMessageHeader<T> {
    fn get_appl_ver_id<const V: u8>(&self) -> ApplVerID<V> {
        ApplVerID::default()
    }
    fn reply<H: crate::header::HeaderExt>(&mut self, other: &H) {
        self.sender_comp_id = other.get_target_comp_id().to_owned();
        self.target_comp_id = other.get_sender_comp_id().to_owned();
        self.msg_seq_num = other.get_msg_seq_num();
    }
}

impl HasHeader for fix42::messages::advertisement::Advertisement {
    type Output = fix42::standard_message_header::StandardMessageHeader<'7'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::allocation_ack::AllocationAck {
    type Output = fix42::standard_message_header::StandardMessageHeader<'P'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::allocation::Allocation {
    type Output = fix42::standard_message_header::StandardMessageHeader<'J'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::bid_request::BidRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'k'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::bid_response::BidResponse {
    type Output = fix42::standard_message_header::StandardMessageHeader<'l'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::business_message_reject::BusinessMessageReject {
    type Output = fix42::standard_message_header::StandardMessageHeader<'j'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::dont_know_trade::DontKnowTrade {
    type Output = fix42::standard_message_header::StandardMessageHeader<'Q'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::email::Email {
    type Output = fix42::standard_message_header::StandardMessageHeader<'C'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::execution_report::ExecutionReport {
    type Output = fix42::standard_message_header::StandardMessageHeader<'8'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::heartbeat::Heartbeat {
    type Output = fix42::standard_message_header::StandardMessageHeader<'0'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::indication_of_interest::IndicationOfInterest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'6'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::list_cancel_request::ListCancelRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'K'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::list_execute::ListExecute {
    type Output = fix42::standard_message_header::StandardMessageHeader<'L'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::list_status_request::ListStatusRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'M'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::list_status::ListStatus {
    type Output = fix42::standard_message_header::StandardMessageHeader<'N'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::list_strike_price::ListStrikePrice {
    type Output = fix42::standard_message_header::StandardMessageHeader<'m'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::logon::Logon {
    type Output = fix42::standard_message_header::StandardMessageHeader<'A'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::logout::Logout {
    type Output = fix42::standard_message_header::StandardMessageHeader<'5'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::market_data_incremental_refresh::MarketDataIncrementalRefresh {
    type Output = fix42::standard_message_header::StandardMessageHeader<'X'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::market_data_request_reject::MarketDataRequestReject {
    type Output = fix42::standard_message_header::StandardMessageHeader<'Y'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::market_data_request::MarketDataRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'V'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh {
    type Output = fix42::standard_message_header::StandardMessageHeader<'W'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::mass_quote::MassQuote {
    type Output = fix42::standard_message_header::StandardMessageHeader<'i'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::new_order_list::NewOrderList {
    type Output = fix42::standard_message_header::StandardMessageHeader<'E'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::new_order_single::NewOrderSingle {
    type Output = fix42::standard_message_header::StandardMessageHeader<'D'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::news::News {
    type Output = fix42::standard_message_header::StandardMessageHeader<'B'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::order_cancel_reject::OrderCancelReject {
    type Output = fix42::standard_message_header::StandardMessageHeader<'9'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::order_cancel_replace_request::OrderCancelReplaceRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'G'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::order_cancel_request::OrderCancelRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'F'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::order_status_request::OrderStatusRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'H'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::quote_acknowledgement::QuoteAcknowledgement {
    type Output = fix42::standard_message_header::StandardMessageHeader<'b'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::quote_cancel::QuoteCancel {
    type Output = fix42::standard_message_header::StandardMessageHeader<'Z'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::quote_request::QuoteRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'R'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::quote::Quote {
    type Output = fix42::standard_message_header::StandardMessageHeader<'S'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::quote_status_request::QuoteStatusRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'a'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::reject::Reject {
    type Output = fix42::standard_message_header::StandardMessageHeader<'3'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::resend_request::ResendRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'2'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::security_definition_request::SecurityDefinitionRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'c'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::security_definition::SecurityDefinition {
    type Output = fix42::standard_message_header::StandardMessageHeader<'d'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::security_status_request::SecurityStatusRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'e'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::security_status::SecurityStatus {
    type Output = fix42::standard_message_header::StandardMessageHeader<'f'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::sequence_reset::SequenceReset {
    type Output = fix42::standard_message_header::StandardMessageHeader<'4'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::settlement_instructions::SettlementInstructions {
    type Output = fix42::standard_message_header::StandardMessageHeader<'T'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::test_request::TestRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'1'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::trading_session_status_request::TradingSessionStatusRequest {
    type Output = fix42::standard_message_header::StandardMessageHeader<'g'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix42::messages::trading_session_status::TradingSessionStatus {
    type Output = fix42::standard_message_header::StandardMessageHeader<'h'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

#[cfg(test)]
mod test {
    use crate::Message;

    #[test]
    fn logon() {
        let msg = "8=FIX.4.2\u{1}9=98\u{1}35=A\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}212=10\u{1}213=0123456789\u{1}369=1\u{1}98=0\u{1}108=1\u{1}10=134\u{1}";
        let mut obj = dbg!(serde_fix::from_str_checked::<Message>(msg)).unwrap();
        match obj {
            Message::FIX42(super::Message::Logon(ref mut l)) => {
                l.standard_message_header.body_length = 0;
            },
            _ => panic!("Deserialized message is not of type Logon"),
        }
        assert_eq!(serde_fix::to_string_checked(&obj), Ok(msg.to_owned()));
    }
}
