
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Mass {
	/// MsgType = b
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader,
	/// Required when acknowledgment is in response to a <a href="message_Quote_Request_R.html" target="main">Quote Request&nbsp;(R)</a> message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "131")]
	pub quote_req_id: Option<String>,
	/// Required when acknowledgment is in response to a Mass Quote, mass Quote Cancel or mass Quote Status Request message. Maps
	/// to: <a href="tag_117_QuoteID.html" target="bottom">QuoteID&nbsp;(117)</a> of a Mass Quote, <a href="tag_1166_QuoteMsgID.html" target="bottom">QuoteMsgID&nbsp;(1166)</a> of Quote Cancel, <a href="tag_649_QuoteStatusReqID.html" target="bottom">QuoteStatusReqID&nbsp;(649)</a> of Quote Status Request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "117")]
	pub quote_id: Option<String>,
	/// Status of the mass quote acknowledgement.
	#[serde(rename = "297")]
	pub quote_status: QuoteStatus,
	/// Reason quote was rejected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "300")]
	pub quote_reject_reason: Option<QuoteRejectReason>,
	/// Level of Response requested from receiver of quote messages. Is echoed back to the counterparty.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "301")]
	pub quote_response_level: Option<QuoteResponseLevel>,
	/// Type of Quote
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "537")]
	pub quote_type: Option<QuoteType>,
	/// QuoteCancelType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "298")]
	pub quote_cancel_type: Option<QuoteCancelType>,
	/// Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages".
	#[serde(flatten)]
	pub parties: Option<super::super::parties::Parties>,
	/// Should be populated if the Mass Quote Acknowledgement is acknowledging a mass quote cancellation by party.
	#[serde(flatten)]
	pub target_parties: Option<super::super::target_parties::TargetParties>,
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
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// EncodedTextLen
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "354")]
	pub encoded_text_len: Option<usize>,
	/// EncodedText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "355")]
	pub encoded_text: Option<String>,
	/// The number of sets of quotes in the message
	#[serde(flatten)]
	pub quot_set_ack_grp: Option<super::super::quot_set_ack_grp::QuotSetAckGrp>,
	/// ThrottleResponse
	#[serde(flatten)]
	pub throttle_response: Option<super::super::throttle_response::ThrottleResponse>,
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
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
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
pub enum QuoteStatus {
	/// Accepted
	#[serde(rename = "0")]
	Accepted,
	/// Canceled for Symbol(s)
	#[serde(rename = "1")]
	CanceledForSymbol,
	/// Canceled for Security Type(s)
	#[serde(rename = "2")]
	CanceledForSecurityType,
	/// Canceled for Underlying
	#[serde(rename = "3")]
	CanceledForUnderlying,
	/// Canceled All
	#[serde(rename = "4")]
	CanceledAll,
	/// Rejected
	#[serde(rename = "5")]
	Rejected,
	/// Removed from Market
	#[serde(rename = "6")]
	RemovedFromMarket,
	/// Expired
	#[serde(rename = "7")]
	Expired,
	/// Query
	#[serde(rename = "8")]
	Query,
	/// Quote Not Found
	#[serde(rename = "9")]
	QuoteNotFound,
	/// Pending
	#[serde(rename = "10")]
	Pending,
	/// Pass
	#[serde(rename = "11")]
	Pass,
	/// Locked Market Warning
	#[serde(rename = "12")]
	LockedMarketWarning,
	/// Cross Market Warning
	#[serde(rename = "13")]
	CrossMarketWarning,
	/// Canceled Due To Lock Market
	#[serde(rename = "14")]
	CanceledDueToLockMarket,
	/// Canceled due to crossed market
	#[serde(rename = "15")]
	CanceledDueToCrossedMarket,
	/// Active
	#[serde(rename = "16")]
	Active,
	/// Canceled
	#[serde(rename = "17")]
	Canceled,
	/// Unsolicited Quote Replenishment
	#[serde(rename = "18")]
	UnsolicitedQuoteReplenishment,
	/// Pending End Trade
	#[serde(rename = "19")]
	PendingEndTrade,
	/// Too Late to End
	#[serde(rename = "20")]
	TooLateToEnd,
	/// Traded
	#[serde(rename = "21")]
	Traded,
	/// Traded and removed
	#[serde(rename = "22")]
	TradedAndRemoved,
	/// Contract terminated
	#[serde(rename = "23")]
	ContractTerminated,
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
