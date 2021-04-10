
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Quote {
	/// MsgType = R
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// QuoteReqID
	#[serde(rename = "131")]
	pub quote_req_id: String,
	/// For tradeable quote model - used to indicate to which RFQ Request this Quote Request is in response.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "644")]
	pub rfq_req_id: Option<String>,
	/// Required only in two party models when <a href="tag_537_QuoteType.html" target="bottom">QuoteType(537)&nbsp;(537)</a> = '1' (Tradeable) and the <a href="tag_40_OrdType.html" target="bottom">OrdType(40)&nbsp;(40)</a> = '2' (Limit).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "11")]
	pub cl_ord_id: Option<String>,
	/// BookingType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "775")]
	pub booking_type: Option<BookingType>,
	/// OrderCapacity
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "528")]
	pub order_capacity: Option<OrderCapacity>,
	/// OrderRestrictions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "529")]
	pub order_restrictions: Option<fix_common::SeparatedValues<OrderRestrictions>>,
	/// Used to indicate whether a private negotiation is requested or if the response should be public. Only relevant in markets
	/// supporting both Private and Public quotes. If field is not provided in message, the model used must be bilaterally agreed..
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
	/// that applies to the whole message, not individual legs, sides, etc..
	#[serde(flatten)]
	pub root_parties: Option<super::super::root_parties::RootParties>,
	/// Number of related symbols (instruments) in Request
	#[serde(flatten)]
	pub quot_req_grp: super::super::quot_req_grp::QuotReqGrp,
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
	/// ComplianceID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "376")]
	pub compliance_id: Option<String>,
	/// ComplianceText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2404")]
	pub compliance_text: Option<String>,
	/// Must be set if EncodedComplianceText(2352) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2351")]
	pub encoded_compliance_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the ComplianceText(2404) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2352")]
	pub encoded_compliance_text: Option<String>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BookingType {
	/// Regular booking
	#[serde(rename = "0")]
	RegularBooking,
	/// CFD (Contract for difference)
	#[serde(rename = "1")]
	Cfd,
	/// Total Return Swap
	#[serde(rename = "2")]
	TotalReturnSwap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderCapacity {
	/// Agency
	#[serde(rename = "A")]
	Agency,
	/// Proprietary
	#[serde(rename = "G")]
	Proprietary,
	/// Individual
	#[serde(rename = "I")]
	Individual,
	/// Principal
	#[serde(rename = "P")]
	Principal,
	/// Riskless Principal
	#[serde(rename = "R")]
	RisklessPrincipal,
	/// Agent for Other Member
	#[serde(rename = "W")]
	AgentForOtherMember,
	/// Mixed capacity
	#[serde(rename = "M")]
	MixedCapacity,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderRestrictions {
	/// Program Trade
	#[serde(rename = "1")]
	ProgramTrade,
	/// Index Arbitrage
	#[serde(rename = "2")]
	IndexArbitrage,
	/// Non-Index Arbitrage
	#[serde(rename = "3")]
	NonIndexArbitrage,
	/// Competing Market Maker
	#[serde(rename = "4")]
	CompetingMarketMaker,
	/// Acting as Market Maker or Specialist in the security
	#[serde(rename = "5")]
	ActingAsMarketMakerOrSpecialistInTheSecurity,
	/// Acting as Market Maker or Specialist in the underlying security of a derivative security
	#[serde(rename = "6")]
	ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity,
	/// Foreign Entity (of foreign governmnet or regulatory jurisdiction)
	#[serde(rename = "7")]
	ForeignEntity,
	/// External Market Participant
	#[serde(rename = "8")]
	ExternalMarketParticipant,
	/// External Inter-connected Market Linkage
	#[serde(rename = "9")]
	ExternalInterConnectedMarketLinkage,
	/// Riskless Arbitrage
	#[serde(rename = "A")]
	RisklessArbitrage,
	/// Issuer Holding
	#[serde(rename = "B")]
	IssuerHolding,
	/// Issue Price Stabilization
	#[serde(rename = "C")]
	IssuePriceStabilization,
	/// Non-algorithmic
	#[serde(rename = "D")]
	NonAlgorithmic,
	/// Algorithmic
	#[serde(rename = "E")]
	Algorithmic,
	/// Cross
	#[serde(rename = "F")]
	Cross,
	/// Insider Account
	#[serde(rename = "G")]
	InsiderAccount,
	/// Significant Shareholder
	#[serde(rename = "H")]
	SignificantShareholder,
	/// Normal Course Issuer Bid (NCIB)
	#[serde(rename = "I")]
	NormalCourseIssuerBid,
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
