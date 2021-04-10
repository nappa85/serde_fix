
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteAck {
	/// MsgType = CW
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Contains the QuoteID(117) of a single Quote(35=S).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "117")]
	pub quote_id: Option<String>,
	/// Contains the QuoteMsgID(1166) of a single Quote(35=S) or QuoteCancel(35=Z).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1166")]
	pub quote_msg_id: Option<String>,
	/// QuoteReqID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "131")]
	pub quote_req_id: Option<String>,
	/// QuoteType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "537")]
	pub quote_type: Option<QuoteType>,
	/// QuoteCancelType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "298")]
	pub quote_cancel_type: Option<QuoteCancelType>,
	/// SecondaryQuoteID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1751")]
	pub secondary_quote_id: Option<String>,
	/// QuoteAckStatus
	#[serde(rename = "1865")]
	pub quote_ack_status: QuoteAckStatus,
	/// Conditionally required when QuoteAckStatus(1865) = 2(Rejected)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "300")]
	pub quote_reject_reason: Option<QuoteRejectReason>,
	/// RejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1328")]
	pub reject_text: Option<String>,
	/// EncodedRejectTextLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1664")]
	pub encoded_reject_text_len: Option<usize>,
	/// EncodedRejectText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1665")]
	pub encoded_reject_text: Option<String>,
	/// Parties
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// May be used by the quote receiver to inform quote provider of pre-trade transparency waiver determination in the context of
	/// MiFID II.
	#[serde(flatten)]
	pub quote_attribute_grp: Option<super::super::quote_attribute_grp::QuoteAttributeGrp>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteType {
	/// Indicative
	#[serde(rename = "0")]
	Indicative,
	/// Tradeable
	#[serde(rename = "1")]
	Tradeable,
	/// Restricted Tradeable
	#[serde(rename = "2")]
	RestrictedTradeable,
	/// Counter (tradeable)
	#[serde(rename = "3")]
	Counter,
	/// Initially tradeable
	#[serde(rename = "4")]
	InitiallyTradeable,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteCancelType {
	/// Cancel for Symbol(s)
	#[serde(rename = "1")]
	CancelForSymbol,
	/// Cancel for Security Type(s)
	#[serde(rename = "2")]
	CancelForSecurityType,
	/// Cancel for Underlying Symbol
	#[serde(rename = "3")]
	CancelForUnderlyingSymbol,
	/// Cancel All Quotes
	#[serde(rename = "4")]
	CancelAllQuotes,
	/// Cancel quote specified in QuoteID
	#[serde(rename = "5")]
	CancelQuoteSpecifiedInQuoteId,
	/// Cancel by QuoteType(537)
	#[serde(rename = "6")]
	CancelByQuoteType,
	/// Cancel for Security Issuer
	#[serde(rename = "7")]
	CancelForSecurityIssuer,
	/// Cancel for Issuer of Underlying Security
	#[serde(rename = "8")]
	CancelForIssuerOfUnderlyingSecurity,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteAckStatus {
	/// Received, not yet processed
	#[serde(rename = "0")]
	ReceivedNotYetProcessed,
	/// Accepted
	#[serde(rename = "1")]
	Accepted,
	/// Rejected
	#[serde(rename = "2")]
	Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteRejectReason {
	/// Unknown symbol (security)
	#[serde(rename = "1")]
	N1,
	/// Exhcnage (security) closed
	#[serde(rename = "2")]
	N2,
	/// Quote exceeds limit
	#[serde(rename = "3")]
	N3,
	/// Too late to enter
	#[serde(rename = "4")]
	N4,
	/// Unknown quote
	#[serde(rename = "5")]
	N5,
	/// Duplicate quote
	#[serde(rename = "6")]
	N6,
	/// Invalid bid/ask spread
	#[serde(rename = "7")]
	N7,
	/// Invalid price
	#[serde(rename = "8")]
	N8,
	/// Not authorized to quote security
	#[serde(rename = "9")]
	N9,
	/// Price exceeds current price band
	#[serde(rename = "10")]
	N10,
	/// Quote Locked - Unable to Update/Cancel
	#[serde(rename = "11")]
	N11,
	/// Invalid or unknown Security Issuer
	#[serde(rename = "12")]
	N12,
	/// Invalid or unknown Issuer of Underlying Security
	#[serde(rename = "13")]
	N13,
	/// Other
	#[serde(rename = "99")]
	N99,
	/// Notional value exceeds threshold
	#[serde(rename = "14")]
	N14,
	/// Price exceeds current price band
	#[serde(rename = "15")]
	N15,
	/// Reference price not available
	#[serde(rename = "16")]
	N16,
	/// Insufficient credit limit
	#[serde(rename = "17")]
	N17,
	/// Exceeded clip size limit
	#[serde(rename = "18")]
	N18,
	/// Exceeded maximum notional order amount
	#[serde(rename = "19")]
	N19,
	/// Exceeded DV01/PV01 limit
	#[serde(rename = "20")]
	N20,
	/// Exceeded CS01 limit
	#[serde(rename = "21")]
	N21,
}
