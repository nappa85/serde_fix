
use serde::{Deserialize, Serialize};

use fix_common::ApplVerID;

use crate::has_header::*;

pub use fix40::*;

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "35")]
pub enum Message {
	/// Heartbeat
	#[serde(rename = "0")]
	Heartbeat(Box<fix40::messages::heartbeat::Heartbeat>),
	/// Test Request
	#[serde(rename = "1")]
	TestRequest(Box<fix40::messages::test_request::TestRequest>),
	/// Resend Request
	#[serde(rename = "2")]
	ResendRequest(Box<fix40::messages::resend_request::ResendRequest>),
	/// Reject
	#[serde(rename = "3")]
	Reject(Box<fix40::messages::reject::Reject>),
	/// Sequence Reset
	#[serde(rename = "4")]
	SequenceReset(Box<fix40::messages::sequence_reset::SequenceReset>),
	/// Logout
	#[serde(rename = "5")]
	Logout(Box<fix40::messages::logout::Logout>),
	/// Indication of Interest
	#[serde(rename = "6")]
	IndicationOfInterest(Box<fix40::messages::indication_of_interest::IndicationOfInterest>),
	/// Advertisement
	#[serde(rename = "7")]
	Advertisement(Box<fix40::messages::advertisement::Advertisement>),
	/// Execution Report
	#[serde(rename = "8")]
	ExecutionReport(Box<fix40::messages::execution_report::ExecutionReport>),
	/// Order Cancel Reject
	#[serde(rename = "9")]
	OrderCancelReject(Box<fix40::messages::order_cancel_reject::OrderCancelReject>),
	/// Logon
	#[serde(rename = "A")]
	Logon(Box<fix40::messages::logon::Logon>),
	/// News
	#[serde(rename = "B")]
	News(Box<fix40::messages::news::News>),
	/// Email
	#[serde(rename = "C")]
	Email(Box<fix40::messages::email::Email>),
	/// New Order - Single
	#[serde(rename = "D")]
	NewOrderSingle(Box<fix40::messages::new_order_single::NewOrderSingle>),
	/// New Order - List
	#[serde(rename = "E")]
	NewOrderList(Box<fix40::messages::new_order_list::NewOrderList>),
	/// Order Cancel Request
	#[serde(rename = "F")]
	OrderCancelRequest(Box<fix40::messages::order_cancel_request::OrderCancelRequest>),
	/// Order Cancel/Replace Request
	#[serde(rename = "G")]
	OrderCancelReplaceRequest(Box<fix40::messages::order_cancel_replace_request::OrderCancelReplaceRequest>),
	/// Order Status Request
	#[serde(rename = "H")]
	OrderStatusRequest(Box<fix40::messages::order_status_request::OrderStatusRequest>),
	/// Allocation
	#[serde(rename = "J")]
	Allocation(Box<fix40::messages::allocation::Allocation>),
	/// List Cancel Request
	#[serde(rename = "K")]
	ListCancelRequest(Box<fix40::messages::list_cancel_request::ListCancelRequest>),
	/// List Execute
	#[serde(rename = "L")]
	ListExecute(Box<fix40::messages::list_execute::ListExecute>),
	/// List Status Request
	#[serde(rename = "M")]
	ListStatusRequest(Box<fix40::messages::list_status_request::ListStatusRequest>),
	/// List Status
	#[serde(rename = "N")]
	ListStatus(Box<fix40::messages::list_status::ListStatus>),
	/// Allocation ACK
	#[serde(rename = "P")]
	AllocationAck(Box<fix40::messages::allocation_ack::AllocationAck>),
	/// Don't Know Trade
	#[serde(rename = "Q")]
	DontKnowTrade(Box<fix40::messages::dont_know_trade::DontKnowTrade>),
	/// Quote Request
	#[serde(rename = "R")]
	QuoteRequest(Box<fix40::messages::quote_request::QuoteRequest>),
	/// Quote
	#[serde(rename = "S")]
	Quote(Box<fix40::messages::quote::Quote>),
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
        }
    }
}

impl<const T: char> crate::header::Header for fix40::standard_message_header::StandardMessageHeader<T> {
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

impl<const T: char> crate::header::HeaderExt for fix40::standard_message_header::StandardMessageHeader<T> {
    fn get_appl_ver_id<const V: u8>(&self) -> ApplVerID<V> {
        ApplVerID::default()
    }
    fn reply<H: crate::header::HeaderExt>(&mut self, other: &H) {
        self.sender_comp_id = other.get_target_comp_id().to_owned();
        self.target_comp_id = other.get_sender_comp_id().to_owned();
        self.msg_seq_num = other.get_msg_seq_num();
    }
}

impl HasHeader for fix40::messages::advertisement::Advertisement {
    type Output = fix40::standard_message_header::StandardMessageHeader<'7'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::allocation_ack::AllocationAck {
    type Output = fix40::standard_message_header::StandardMessageHeader<'P'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::allocation::Allocation {
    type Output = fix40::standard_message_header::StandardMessageHeader<'J'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::dont_know_trade::DontKnowTrade {
    type Output = fix40::standard_message_header::StandardMessageHeader<'Q'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::email::Email {
    type Output = fix40::standard_message_header::StandardMessageHeader<'C'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::execution_report::ExecutionReport {
    type Output = fix40::standard_message_header::StandardMessageHeader<'8'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::heartbeat::Heartbeat {
    type Output = fix40::standard_message_header::StandardMessageHeader<'0'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::indication_of_interest::IndicationOfInterest {
    type Output = fix40::standard_message_header::StandardMessageHeader<'6'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::list_cancel_request::ListCancelRequest {
    type Output = fix40::standard_message_header::StandardMessageHeader<'K'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::list_execute::ListExecute {
    type Output = fix40::standard_message_header::StandardMessageHeader<'L'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::list_status_request::ListStatusRequest {
    type Output = fix40::standard_message_header::StandardMessageHeader<'M'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::list_status::ListStatus {
    type Output = fix40::standard_message_header::StandardMessageHeader<'N'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::logon::Logon {
    type Output = fix40::standard_message_header::StandardMessageHeader<'A'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::logout::Logout {
    type Output = fix40::standard_message_header::StandardMessageHeader<'5'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::new_order_list::NewOrderList {
    type Output = fix40::standard_message_header::StandardMessageHeader<'E'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::new_order_single::NewOrderSingle {
    type Output = fix40::standard_message_header::StandardMessageHeader<'D'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::news::News {
    type Output = fix40::standard_message_header::StandardMessageHeader<'B'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::order_cancel_reject::OrderCancelReject {
    type Output = fix40::standard_message_header::StandardMessageHeader<'9'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::order_cancel_replace_request::OrderCancelReplaceRequest {
    type Output = fix40::standard_message_header::StandardMessageHeader<'G'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::order_cancel_request::OrderCancelRequest {
    type Output = fix40::standard_message_header::StandardMessageHeader<'F'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::order_status_request::OrderStatusRequest {
    type Output = fix40::standard_message_header::StandardMessageHeader<'H'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::quote_request::QuoteRequest {
    type Output = fix40::standard_message_header::StandardMessageHeader<'R'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::quote::Quote {
    type Output = fix40::standard_message_header::StandardMessageHeader<'S'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::reject::Reject {
    type Output = fix40::standard_message_header::StandardMessageHeader<'3'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::resend_request::ResendRequest {
    type Output = fix40::standard_message_header::StandardMessageHeader<'2'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::sequence_reset::SequenceReset {
    type Output = fix40::standard_message_header::StandardMessageHeader<'4'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

impl HasHeader for fix40::messages::test_request::TestRequest {
    type Output = fix40::standard_message_header::StandardMessageHeader<'1'>;
    fn get_header(&self) -> &Self::Output {
        &self.standard_message_header
    }
}

#[cfg(test)]
mod test {
    use crate::Message;

    #[test]
    fn logon() {
        let msg = "8=FIX.4.0\u{1}9=70\u{1}35=A\u{1}49=CLIENT1\u{1}56=EXECUTOR\u{1}34=17\u{1}52=20210310-16:38:01.821\u{1}98=0\u{1}108=1\u{1}10=084\u{1}";
        let mut obj = dbg!(serde_fix::from_str_checked::<Message>(msg)).unwrap();
        match obj {
            Message::FIX40(super::Message::Logon(ref mut l)) => {
                l.standard_message_header.body_length = 0;
            },
            _ => panic!("Deserialized message is not of type Logon"),
        }
        assert_eq!(serde_fix::to_string_checked(&obj), Ok(msg.to_owned()));
    }
}
