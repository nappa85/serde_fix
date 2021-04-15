
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteAcknowledgement {
	/// MsgType = b
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'b'>,
	/// Required when acknowledgment is in response to a <a href="message_Quote_Request_R.html" target="main">Quote Request&nbsp;(R)</a> message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "131")]
	pub quote_req_id: Option<String>,
	/// Required when acknowledgment is in response to a <a href="message_Quote_S.html" target="main">Quote&nbsp;(S)</a> message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "117")]
	pub quote_id: Option<String>,
	/// Status of the <a href="message_Quote_Acknowledgement_b.html" target="main">quote acknowledgement&nbsp;(b)</a> .
	#[serde(rename = "297")]
	pub quote_ack_status: QuoteAckStatus,
	/// Reason quote was rejected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "300")]
	pub quote_reject_reason: Option<QuoteRejectReason>,
	/// Level of Response requested from receiver of quote messages. Is echoed back to the counterparty.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "301")]
	pub quote_response_level: Option<QuoteResponseLevel>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
	/// Text
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "58")]
	pub text: Option<String>,
	/// The number of sets of quotes in the message
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "296")]
	pub quote_sets: Option<fix_common::RepeatingValues<QuoteSet>>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuoteSet {
	/// First field in repeating group. Required if <a href="tag_296_NoQuoteSets.html" target="bottom">NoQuoteSets&nbsp;(296)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "302")]
	pub quote_set_id: Option<String>,
	/// Required if <a href="tag_296_NoQuoteSets.html" target="bottom">NoQuoteSets&nbsp;(296)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "311")]
	pub underlying_symbol: Option<String>,
	/// UnderlyingSymbolSfx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "312")]
	pub underlying_symbol_sfx: Option<String>,
	/// UnderlyingSecurityID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "309")]
	pub underlying_security_id: Option<String>,
	/// UnderlyingIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "305")]
	pub underlying_id_source: Option<UnderlyingIDSource>,
	/// UnderlyingSecurityType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "310")]
	pub underlying_security_type: Option<UnderlyingSecurityType>,
	/// Required if <a href="tag_314_UnderlyingMaturityDay.html" target="bottom">UnderlyingMaturityDay&nbsp;(314)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "313")]
	pub underlying_maturity_month_year: Option<fix_common::MonthYear>,
	/// UnderlyingMaturityDay
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "314")]
	pub underlying_maturity_day: Option<u8>,
	/// UnderlyingPutOrCall
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "315")]
	pub underlying_put_or_call: Option<UnderlyingPutOrCall>,
	/// UnderlyingStrikePrice
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "316")]
	pub underlying_strike_price: Option<f64>,
	/// UnderlyingOptAttribute
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "317")]
	pub underlying_opt_attribute: Option<UnderlyingOptAttribute>,
	/// For Fixed Income, Convertible Bonds, Derivatives, etc.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "436")]
	pub underlying_contract_multiplier: Option<f64>,
	/// UnderlyingCouponRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "435")]
	pub underlying_coupon_rate: Option<f64>,
	/// UnderlyingSecurityExchange
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "308")]
	pub underlying_security_exchange: Option<String>,
	/// UnderlyingIssuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "306")]
	pub underlying_issuer: Option<String>,
	/// Must be set if <a href="tag_363_EncodedUnderlyingIssuer.html" target="bottom">EncodedUnderlyingIssuer&nbsp;(363)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "362")]
	pub encoded_underlying_issuer_len: Option<i32>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_306_UnderlyingIssuer.html" target="bottom">UnderlyingIssuer&nbsp;(306)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "363")]
	pub encoded_underlying_issuer: Option<String>,
	/// UnderlyingSecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "307")]
	pub underlying_security_desc: Option<String>,
	/// Must be set if <a href="tag_365_EncodedUnderlyingSecurityDesc.html" target="bottom">EncodedUnderlyingSecurityDesc&nbsp;(365)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "364")]
	pub encoded_underlying_security_desc_len: Option<i32>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_307_UnderlyingSecurityDesc.html" target="bottom">UnderlyingSecurityDesc&nbsp;(307)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "365")]
	pub encoded_underlying_security_desc: Option<String>,
	/// Total number of quotes for the QuoteSet across all messages. Should be the sum of all <a href="tag_295_NoQuoteEntries.html" target="bottom">NoQuoteEntries&nbsp;(295)</a> in each message that has repeating quotes that are part of the same QuoteSet. Required if <a href="tag_295_NoQuoteEntries.html" target="bottom">NoQuoteEntries&nbsp;(295)</a> &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "304")]
	pub tot_quote_entries: Option<TotQuoteEntries>,
	/// NoQuoteEntries
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "295")]
	pub no_quote_entries: Option<i32>,
	/// Uniquely identifies the quote as part of a QuoteSet. First field in repeating group. Required if <a href="tag_295_NoQuoteEntries.html" target="bottom">NoQuoteEntries&nbsp;(295)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "299")]
	pub quote_entry_id: Option<String>,
	/// Symbol
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "55")]
	pub symbol: Option<String>,
	/// SymbolSfx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "65")]
	pub symbol_sfx: Option<String>,
	/// SecurityID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "48")]
	pub security_id: Option<String>,
	/// IDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "22")]
	pub id_source: Option<IDSource>,
	/// Must be specified if a Future or Option. If a Future: <a href="tag_55_Symbol.html" target="bottom">Symbol&nbsp;(55)</a> , <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> , and <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> are required. If an Option: <a href="tag_55_Symbol.html" target="bottom">Symbol&nbsp;(55)</a> , <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> , <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> , <a href="tag_201_PutOrCall.html" target="bottom">PutOrCall&nbsp;(201)</a> , and <a href="tag_202_StrikePrice.html" target="bottom">StrikePrice&nbsp;(202)</a> are required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// Specifies the month and year of maturity. Required if <a href="tag_205_MaturityDay.html" target="bottom">MaturityDay&nbsp;(205)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "200")]
	pub maturity_month_year: Option<fix_common::MonthYear>,
	/// Can be used in conjunction with <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> to specify a particular maturity date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "205")]
	pub maturity_day: Option<u8>,
	/// For Options.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "201")]
	pub put_or_call: Option<PutOrCall>,
	/// For Options.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "202")]
	pub strike_price: Option<f64>,
	/// For Options.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "206")]
	pub opt_attribute: Option<OptAttribute>,
	/// For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g.
	/// contracts vs. shares) amount.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "231")]
	pub contract_multiplier: Option<f64>,
	/// For Fixed Income.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "223")]
	pub coupon_rate: Option<f64>,
	/// Can be used to identify the security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "207")]
	pub security_exchange: Option<String>,
	/// Issuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "106")]
	pub issuer: Option<String>,
	/// Must be set if <a href="tag_349_EncodedIssuer.html" target="bottom">EncodedIssuer&nbsp;(349)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "348")]
	pub encoded_issuer_len: Option<i32>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_106_Issuer.html" target="bottom">Issuer&nbsp;(106)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "349")]
	pub encoded_issuer: Option<String>,
	/// SecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "107")]
	pub security_desc: Option<String>,
	/// Must be set if <a href="tag_351_EncodedSecurityDesc.html" target="bottom">EncodedSecurityDesc&nbsp;(351)</a> field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "350")]
	pub encoded_security_desc_len: Option<i32>,
	/// Encoded (non-ASCII characters) representation of the <a href="tag_107_SecurityDesc.html" target="bottom">SecurityDesc&nbsp;(107)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "351")]
	pub encoded_security_desc: Option<String>,
	/// Reason Quote Entry was rejected.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "368")]
	pub quote_entry_reject_reason: Option<QuoteEntryRejectReason>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuoteAckStatus {
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
}

impl Default for QuoteAckStatus {
	fn default() -> Self {
		QuoteAckStatus::Accepted
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuoteRejectReason {
	/// Unknown symbol (Security)
	#[serde(rename = "1")]
	UnknownSymbol,
	/// Exchange(Security) closed
	#[serde(rename = "2")]
	ExchangeClosed,
	/// Quote exceeds limit
	#[serde(rename = "3")]
	QuoteExceedsLimit,
	/// Too late to enter
	#[serde(rename = "4")]
	TooLateToEnter,
	/// Unknown Quote
	#[serde(rename = "5")]
	UnknownQuote,
	/// Duplicate Quote
	#[serde(rename = "6")]
	DuplicateQuote,
	/// Invalid bid/ask spread
	#[serde(rename = "7")]
	InvalidBidAskSpread,
	/// Invalid price
	#[serde(rename = "8")]
	InvalidPrice,
	/// Not authorized to quote security
	#[serde(rename = "9")]
	NotAuthorizedToQuoteSecurity,
}

impl Default for QuoteRejectReason {
	fn default() -> Self {
		QuoteRejectReason::UnknownSymbol
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
pub enum UnderlyingIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN number
	#[serde(rename = "4")]
	IsinNumber,
	/// RIC code
	#[serde(rename = "5")]
	RicCode,
	/// ISO Currency Code
	#[serde(rename = "6")]
	IsoCurrencyCode,
	/// ISO Country Code
	#[serde(rename = "7")]
	IsoCountryCode,
	/// Exchange Symbol
	#[serde(rename = "8")]
	ExchangeSymbol,
	/// Consolidated Tape Association (CTA) Symbol (SIAC CTS/CQS line format)
	#[serde(rename = "9")]
	ConsolidatedTapeAssociationSymbol,
}

impl Default for UnderlyingIDSource {
	fn default() -> Self {
		UnderlyingIDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingSecurityType {
	/// Bankers Acceptance
	#[serde(rename = "BA")]
	BankersAcceptance,
	/// Convertible Bond (Note not part of ISITC spec)
	#[serde(rename = "CB")]
	ConvertibleBond,
	/// Certificate Of Deposit
	#[serde(rename = "CD")]
	CertificateOfDeposit,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	CollateralizedMortgageObligation,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	CorporateBond,
	/// Commercial Paper
	#[serde(rename = "CP")]
	CommercialPaper,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	CorporatePrivatePlacement,
	/// Common Stock
	#[serde(rename = "CS")]
	CommonStock,
	/// Federal Housing Authority
	#[serde(rename = "FHA")]
	FederalHousingAuthority,
	/// Federal Home Loan
	#[serde(rename = "FHL")]
	FederalHomeLoan,
	/// Federal National Mortgage Association
	#[serde(rename = "FN")]
	FederalNationalMortgageAssociation,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	ForeignExchangeContract,
	/// Future
	#[serde(rename = "FUT")]
	Future,
	/// Government National Mortgage Association
	#[serde(rename = "GN")]
	GovernmentNationalMortgageAssociation,
	/// Treasuries + Agency DebentureI
	#[serde(rename = "GOVT")]
	TreasuriesAgencyDebentureI,
	/// Mutual Fund
	#[serde(rename = "ET Mortgage IOETTEMF")]
	MutualFund,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	MortgageInterestOnly,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	MortgagePrincipalOnly,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	MortgagePrivatePlacement,
	/// Miscellaneous Pass-Thru
	#[serde(rename = "MPT")]
	MiscellaneousPassThru,
	/// Municipal Bond
	#[serde(rename = "MUNI")]
	MunicipalBond,
	/// No ISITC Security Type
	#[serde(rename = "NONE")]
	NoIsitcSecurityType,
	/// Option
	#[serde(rename = "OPT")]
	Option,
	/// Preferred Stock
	#[serde(rename = "PS")]
	PreferredStock,
	/// Repurchase Agreement
	#[serde(rename = "RP")]
	RepurchaseAgreement,
	/// Reverse Repurchase Agreement
	#[serde(rename = "RVRP")]
	ReverseRepurchaseAgreement,
	/// Student Loan Marketing Association
	#[serde(rename = "SL")]
	StudentLoanMarketingAssociation,
	/// Time Deposit
	#[serde(rename = "TD")]
	TimeDeposit,
	/// US Treasury Bill
	#[serde(rename = "USTB")]
	UsTreasuryBill,
	/// Warrant
	#[serde(rename = "WAR")]
	Warrant,
	/// Cats, Tigers &amp; Lions (a real code: US Treasury Receipts)
	#[serde(rename = "ZOO")]
	CatsTigersAmpLions,
	/// 'Wildcard' entry (used on Security Definition Request message)
	#[serde(rename = "?")]
	WildcardEntry,
}

impl Default for UnderlyingSecurityType {
	fn default() -> Self {
		UnderlyingSecurityType::BankersAcceptance
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingPutOrCall {
	/// Put
	#[serde(rename = "0")]
	Put,
	/// Call
	#[serde(rename = "1")]
	Call,
}

impl Default for UnderlyingPutOrCall {
	fn default() -> Self {
		UnderlyingPutOrCall::Put
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingOptAttribute {
	/// Long (a.k.a. 'American')
	#[serde(rename = "L")]
	Long,
	/// Short (a.k.a. 'European')
	#[serde(rename = "S")]
	Short,
}

impl Default for UnderlyingOptAttribute {
	fn default() -> Self {
		UnderlyingOptAttribute::Long
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TotQuoteEntries {
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum IDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN number
	#[serde(rename = "4")]
	IsinNumber,
	/// RIC code
	#[serde(rename = "5")]
	RicCode,
	/// ISO Currency Code
	#[serde(rename = "6")]
	IsoCurrencyCode,
	/// ISO Country Code
	#[serde(rename = "7")]
	IsoCountryCode,
	/// Exchange Symbol
	#[serde(rename = "8")]
	ExchangeSymbol,
	/// Consolidated Tape Association (CTA) Symbol (SIAC CTS/CQS line format)
	#[serde(rename = "9")]
	ConsolidatedTapeAssociationSymbol,
}

impl Default for IDSource {
	fn default() -> Self {
		IDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityType {
	/// Bankers Acceptance
	#[serde(rename = "BA")]
	BankersAcceptance,
	/// Convertible Bond (Note not part of ISITC spec)
	#[serde(rename = "CB")]
	ConvertibleBond,
	/// Certificate Of Deposit
	#[serde(rename = "CD")]
	CertificateOfDeposit,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	CollateralizedMortgageObligation,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	CorporateBond,
	/// Commercial Paper
	#[serde(rename = "CP")]
	CommercialPaper,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	CorporatePrivatePlacement,
	/// Common Stock
	#[serde(rename = "CS")]
	CommonStock,
	/// Federal Housing Authority
	#[serde(rename = "FHA")]
	FederalHousingAuthority,
	/// Federal Home Loan
	#[serde(rename = "FHL")]
	FederalHomeLoan,
	/// Federal National Mortgage Association
	#[serde(rename = "FN")]
	FederalNationalMortgageAssociation,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	ForeignExchangeContract,
	/// Future
	#[serde(rename = "FUT")]
	Future,
	/// Government National Mortgage Association
	#[serde(rename = "GN")]
	GovernmentNationalMortgageAssociation,
	/// Treasuries + Agency DebentureI
	#[serde(rename = "GOVT")]
	TreasuriesAgencyDebentureI,
	/// Mutual Fund
	#[serde(rename = "ET Mortgage IOETTEMF")]
	MutualFund,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	MortgageInterestOnly,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	MortgagePrincipalOnly,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	MortgagePrivatePlacement,
	/// Miscellaneous Pass-Thru
	#[serde(rename = "MPT")]
	MiscellaneousPassThru,
	/// Municipal Bond
	#[serde(rename = "MUNI")]
	MunicipalBond,
	/// No ISITC Security Type
	#[serde(rename = "NONE")]
	NoIsitcSecurityType,
	/// Option
	#[serde(rename = "OPT")]
	Option,
	/// Preferred Stock
	#[serde(rename = "PS")]
	PreferredStock,
	/// Repurchase Agreement
	#[serde(rename = "RP")]
	RepurchaseAgreement,
	/// Reverse Repurchase Agreement
	#[serde(rename = "RVRP")]
	ReverseRepurchaseAgreement,
	/// Student Loan Marketing Association
	#[serde(rename = "SL")]
	StudentLoanMarketingAssociation,
	/// Time Deposit
	#[serde(rename = "TD")]
	TimeDeposit,
	/// US Treasury Bill
	#[serde(rename = "USTB")]
	UsTreasuryBill,
	/// Warrant
	#[serde(rename = "WAR")]
	Warrant,
	/// Cats, Tigers &amp; Lions (a real code: US Treasury Receipts)
	#[serde(rename = "ZOO")]
	CatsTigersAmpLions,
	/// 'Wildcard' entry (used on Security Definition Request message)
	#[serde(rename = "?")]
	WildcardEntry,
}

impl Default for SecurityType {
	fn default() -> Self {
		SecurityType::BankersAcceptance
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PutOrCall {
	/// Put
	#[serde(rename = "0")]
	Put,
	/// Call
	#[serde(rename = "1")]
	Call,
}

impl Default for PutOrCall {
	fn default() -> Self {
		PutOrCall::Put
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum OptAttribute {
	/// Long (a.k.a. 'American')
	#[serde(rename = "L")]
	Long,
	/// Short (a.k.a. 'European')
	#[serde(rename = "S")]
	Short,
}

impl Default for OptAttribute {
	fn default() -> Self {
		OptAttribute::Long
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum QuoteEntryRejectReason {
	/// Unknown symbol (Security)
	#[serde(rename = "1")]
	UnknownSymbol,
	/// Exchange(Security) closed
	#[serde(rename = "2")]
	ExchangeClosed,
	/// Quote exceeds limit
	#[serde(rename = "3")]
	QuoteExceedsLimit,
	/// Too late to enter
	#[serde(rename = "4")]
	TooLateToEnter,
	/// Unknown Quote
	#[serde(rename = "5")]
	UnknownQuote,
	/// Duplicate Quote
	#[serde(rename = "6")]
	DuplicateQuote,
	/// Invalid bid/ask spread
	#[serde(rename = "7")]
	InvalidBidAskSpread,
	/// Invalid price
	#[serde(rename = "8")]
	InvalidPrice,
	/// Not authorized to quote security
	#[serde(rename = "9")]
	NotAuthorizedToQuoteSecurity,
}

impl Default for QuoteEntryRejectReason {
	fn default() -> Self {
		QuoteEntryRejectReason::UnknownSymbol
	}
}
