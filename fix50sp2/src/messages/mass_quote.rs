
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Mass {
	/// MsgType = i
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Required when quote is in response to a <a href="message_Quote_Request_R.html" target="main">Quote Request&nbsp;(R)</a> message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "131")]
	pub quote_req_id: Option<String>,
	/// QuoteID
	#[serde(rename = "117")]
	pub quote_id: String,
	/// Type of Quote.Default is Indicative if not specified
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "537")]
	pub quote_type: Option<QuoteType>,
	/// Level of Response requested from receiver of quote messages.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "301")]
	pub quote_response_level: Option<QuoteResponseLevel>,
	/// Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Account
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1")]
	pub account: Option<String>,
	/// AcctIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "660")]
	pub acct_id_source: Option<AcctIDSource>,
	/// Type of account associated with the order (Origin)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "581")]
	pub account_type: Option<AccountType>,
	/// Default Bid Size for quote contained within this quote message - if not explicitly provided.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "293")]
	pub def_bid_size: Option<f64>,
	/// Default Offer Size for quotes contained within this quote message - if not explicitly provided.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "294")]
	pub def_offer_size: Option<f64>,
	/// The number of sets of quotes in the message
	#[serde(flatten)]
	pub quot_set_grp: super::super::quot_set_grp::QuotSetGrp,
	/// ThrottleInst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1685")]
	pub throttle_inst: Option<ThrottleInst>,
	/// QuoteModelType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2403")]
	pub quote_model_type: Option<QuoteModelType>,
	/// ComplianceID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "376")]
	pub compliance_id: Option<String>,
	/// ComplianceText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2404")]
	pub compliance_text: Option<String>,
	/// Must be set if EncodedComplianceText(2352) field is specified and must immediately precede it.
	#[serde(rename = "2351")]
	/// Encoded (non-ASCII characters) representation of the ComplianceText(2404) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "2352")]
	pub encoded_compliance_text: Option<fix_common::EncodedText<2352>>,
	/// SelfMatchPreventionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2362")]
	pub self_match_prevention_id: Option<String>,
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

impl Default for QuoteType {
	fn default() -> Self {
		QuoteType::Indicative
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteResponseLevel {
	/// No Acknowledgement (Default)
	#[serde(rename = "0")]
	NoAcknowledgement,
	/// Acknowledge only negative or erroneous quotes
	#[serde(rename = "1")]
	AcknowledgeOnlyNegativeOrErroneousQuotes,
	/// Acknowledge each quote messages
	#[serde(rename = "2")]
	AcknowledgeEachQuoteMessages,
	/// Summary Acknowledgement
	#[serde(rename = "3")]
	SummaryAcknowledgement,
}

impl Default for QuoteResponseLevel {
	fn default() -> Self {
		QuoteResponseLevel::NoAcknowledgement
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AcctIDSource {
	/// BIC
	#[serde(rename = "1")]
	Bic,
	/// SID code
	#[serde(rename = "2")]
	SidCode,
	/// TFM (GSPTA)
	#[serde(rename = "3")]
	Tfm,
	/// OMGEO (AlertID)
	#[serde(rename = "4")]
	Omgeo,
	/// DTCC code
	#[serde(rename = "5")]
	DtccCode,
	/// Special Segregated Account ID
	#[serde(rename = "6")]
	SpecialSegregatedAccountId,
	/// Other (custom or proprietary)
	#[serde(rename = "99")]
	Other,
}

impl Default for AcctIDSource {
	fn default() -> Self {
		AcctIDSource::Bic
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AccountType {
	/// Account is carried on customer Side of Books
	#[serde(rename = "1")]
	AccountIsCarriedOnCustomerSideOfBooks,
	/// Account is carried on non-Customer Side of books
	#[serde(rename = "2")]
	AccountIsCarriedOnNonCustomerSideOfBooks,
	/// House Trader
	#[serde(rename = "3")]
	HouseTrader,
	/// Floor Trader
	#[serde(rename = "4")]
	FloorTrader,
	/// Account is carried on non-customer side of books and is cross margined
	#[serde(rename = "6")]
	AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined,
	/// Account is house trader and is cross margined
	#[serde(rename = "7")]
	AccountIsHouseTraderAndIsCrossMargined,
	/// Joint Backoffice Account (JBO)
	#[serde(rename = "8")]
	JointBackofficeAccount,
	/// Equities specialist
	#[serde(rename = "9")]
	EquitiesSpecialist,
	/// Options market maker
	#[serde(rename = "10")]
	OptionsMarketMaker,
	/// Options firm account
	#[serde(rename = "11")]
	OptionsFirmAccount,
	/// Account for customer and non-customer orders
	#[serde(rename = "12")]
	AccountForCustomerAndNonCustomerOrders,
	/// Account for orders from multiple customers
	#[serde(rename = "13")]
	AccountForOrdersFromMultipleCustomers,
}

impl Default for AccountType {
	fn default() -> Self {
		AccountType::AccountIsCarriedOnCustomerSideOfBooks
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ThrottleInst {
	/// Reject if throttle limit exceeded
	#[serde(rename = "0")]
	RejectIfThrottleLimitExceeded,
	/// Queue if throttle limit exceeded
	#[serde(rename = "1")]
	QueueIfThrottleLimitExceeded,
}

impl Default for ThrottleInst {
	fn default() -> Self {
		ThrottleInst::RejectIfThrottleLimitExceeded
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuoteModelType {
	/// Quote Entry (New quote is entered or previously submitted quote is updated in full without regard to amount executed when
	/// a subsequent quote (e.g. with the same QuoteID reference) is received by the Recipient of the quote message.)
	#[serde(rename = "1")]
	QuoteEntryIsReceivedByTheRecipientOfTheQuoteMessage,
	/// Quote Modification (Previously submitted quote must be present and is updated, taking into consideration the amount already
	/// executed when a subsequent quote (e.g. with the same QuoteID reference) is received by the Recipient of the quote message.)
	#[serde(rename = "2")]
	QuoteModificationIsReceivedByTheRecipientOfTheQuoteMessage,
}

impl Default for QuoteModelType {
	fn default() -> Self {
		QuoteModelType::QuoteEntryIsReceivedByTheRecipientOfTheQuoteMessage
	}
}
