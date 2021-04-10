
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Quote {
	/// MsgType = AG
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// QuoteReqID
	#[serde(rename = "131")]
	pub quote_req_id: String,
	/// For tradeable quote model - used to indicate to which <a href="message_RFQ_Request_AH.html" target="main">RFQ Request&nbsp;(AH)</a> this <a href="message_Quote_Request_R.html" target="main">Quote Request&nbsp;(R)</a> is in response.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "644")]
	pub rfq_req_id: Option<String>,
	/// Reason <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a> was rejected
	#[serde(rename = "658")]
	pub quote_request_reject_reason: QuoteRequestRejectReason,
	/// Used to indicate whether a private negotiation is requested or if the response should be public. Only relevant in markets
	/// supporting both Private and Public quotes.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1171")]
	pub private_quote: Option<PrivateQuote>,
	/// RespondentType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1172")]
	pub respondent_type: Option<RespondentType>,
	/// PreTradeAnonymity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1091")]
	pub pre_trade_anonymity: Option<fix_common::Boolean>,
	/// Insert here the set of "Root Parties" fields defined in "common components of application messages". Used for acting parties
	/// that applies to the whole message, not individual legs, sides, etc.
	#[serde(flatten)]
	pub root_parties: Option<super::super::root_parties::RootParties>,
	/// Number of related symbols (instruments) in Request.
	#[serde(flatten)]
	pub quot_req_rjct_grp: super::super::quot_req_rjct_grp::QuotReqRjctGrp,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// Must be set if <a href="tag_355_EncodedText.html" target="bottom">EncodedText&nbsp;(355)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_58_Text.html" target="bottom">Text&nbsp;(58)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteRequestRejectReason {
	/// Unknown symbol (Security)
	#[serde(rename = "1")]
	UnknownSymbol,
	/// Exchange(Security) closed
	#[serde(rename = "2")]
	ExchangeClosed,
	/// Quote request exceeds limit
	#[serde(rename = "3")]
	QuoteRequestExceedsLimit,
	/// Too late to enter
	#[serde(rename = "4")]
	TooLateToEnter,
	/// Invalid price
	#[serde(rename = "5")]
	InvalidPrice,
	/// Not authorized to request quote
	#[serde(rename = "6")]
	NotAuthorizedToRequestQuote,
	/// No Match For Inquiry
	#[serde(rename = "7")]
	NoMatchForInquiry,
	/// No Market For Instrument
	#[serde(rename = "8")]
	NoMarketForInstrument,
	/// No Inventory
	#[serde(rename = "9")]
	NoInventory,
	/// Pass
	#[serde(rename = "10")]
	Pass,
	/// Insufficient credit
	#[serde(rename = "11")]
	InsufficientCredit,
	/// Other
	#[serde(rename = "99")]
	Other,
	/// Exceeded clip size limit
	#[serde(rename = "12")]
	ExceededClipSizeLimit,
	/// Exceeded maximum notional order amount
	#[serde(rename = "13")]
	ExceededMaximumNotionalOrderAmount,
	/// Exceeded DV01/PV01 limit
	#[serde(rename = "14")]
	ExceededDv01Pv01Limit,
	/// Exceeded CS01 limit
	#[serde(rename = "15")]
	ExceededCs01Limit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PrivateQuote {
	/// Private Quote
	#[serde(rename = "Y")]
	PrivateQuote,
	/// Public Quote
	#[serde(rename = "N")]
	PublicQuote,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RespondentType {
	/// All market participants
	#[serde(rename = "1")]
	AllMarketParticipants,
	/// Specified market participants
	#[serde(rename = "2")]
	SpecifiedMarketParticipants,
	/// All Market Makers
	#[serde(rename = "3")]
	AllMarketMakers,
	/// Primary Market Maker(s)
	#[serde(rename = "4")]
	PrimaryMarketMaker,
}
