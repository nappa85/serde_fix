
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct RFQ {
	/// MsgType = AH
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// RFQReqID
	#[serde(rename = "644")]
	pub rfq_req_id: String,
	/// Insert here the set of Parties (firm identification) fields defined in COMMON COMPONENTS OF APPLICATION MESSAGES
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Number of related symbols (instruments) in Request
	#[serde(flatten)]
	pub rfq_req_grp: super::super::rfq_req_grp::RFQReqGrp,
	/// Used to subscribe for <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a> Requests that are sent into a market
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "263")]
	pub subscription_request_type: Option<SubscriptionRequestType>,
	/// Used to indicate whether a private negotiation is requested or if the response should be public. Only relevant in markets
	/// supporting both Private and Public quotes. If field is not provided in message, the model used must be bilaterally agreed.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1171")]
	pub private_quote: Option<PrivateQuote>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum SubscriptionRequestType {
	/// Snapshot
	#[serde(rename = "0")]
	Snapshot,
	/// Snapshot + Updates (Subscribe)
	#[serde(rename = "1")]
	SnapshotUpdates,
	/// Disable previous Snapshot + Update Request (Unsubscribe)
	#[serde(rename = "2")]
	DisablePreviousSnapshotUpdateRequest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum PrivateQuote {
	/// Private Quote
	#[serde(rename = "Y")]
	PrivateQuote,
	/// Public Quote
	#[serde(rename = "N")]
	PublicQuote,
}
