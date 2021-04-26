
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteCancel {
	/// MsgType = Z
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'Z', ' '>,
	/// Required when quote is in response to a <a href="message_Quote_Request_R.html" target="main">Quote Request&nbsp;(R)</a> message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "131")]
	pub quote_req_id: Option<String>,
	/// QuoteID
	#[serde(rename = "117")]
	pub quote_id: String,
	/// Identifies the type of <a href="message_Quote_Cancel_Z.html" target="main">Quote Cancel&nbsp;(Z)</a> request.
	#[serde(rename = "298")]
	pub quote_cancel_type: QuoteCancelType,
	/// Level of Response requested from receiver of <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a> messages.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "301")]
	pub quote_response_level: Option<QuoteResponseLevel>,
	/// Parties
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
	/// Type of account associated with the order (Origin).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "581")]
	pub account_type: Option<AccountType>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// TradingSessionSubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "625")]
	pub trading_session_sub_id: Option<String>,
	/// The number of securities (instruments) whose quotes are to be canceled. Not required when cancelling all quotes.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "295")]
	pub quote_entries: Option<fix_common::RepeatingValues<QuoteEntrie>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteEntrie {
	/// Number of underlyings
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "711")]
	pub underlyings: Option<fix_common::RepeatingValues<super::super::underlying_instrument::UnderlyingInstrument>>,
	/// Required for multileg quotes
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "555")]
	pub legs: Option<fix_common::RepeatingValues<super::super::instrument_leg::InstrumentLeg>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for QuoteCancelType {
	fn default() -> Self {
		QuoteCancelType::CancelForSymbol
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for QuoteResponseLevel {
	fn default() -> Self {
		QuoteResponseLevel::NoAcknowledgement
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
	/// Other (custom or proprietary)
	#[serde(rename = "99")]
	Other,
}

impl Default for AcctIDSource {
	fn default() -> Self {
		AcctIDSource::Bic
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for AccountType {
	fn default() -> Self {
		AccountType::AccountIsCarriedOnCustomerSideOfBooks
	}
}
