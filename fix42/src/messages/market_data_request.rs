
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MarketDataRequest {
	/// MsgType = V
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'V'>,
	/// Must be unique, or the ID of previous Market Data Request to disable if <a href="tag_263_SubscriptionRequestType.html" target="bottom">SubscriptionRequestType&nbsp;(263)</a> = Disable previous Snapshot + Updates Request (2).
	#[serde(rename = "262")]
	pub md_req_id: String,
	/// <a href="tag_263_SubscriptionRequestType.html" target="bottom">SubscriptionRequestType&nbsp;(263)</a> indicates to the other party what type of response is expected. A snapshot request only asks for current information. A subscribe
	/// request asks for updates as the status changes. Unsubscribe will cancel any future update messages from the counter party.
	#[serde(rename = "263")]
	pub subscription_request_type: SubscriptionRequestType,
	/// MarketDepth
	#[serde(deserialize_with = "fix_common::workarounds::from_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(rename = "264")]
	pub market_depth: u32,
	/// Required if <a href="tag_263_SubscriptionRequestType.html" target="bottom">SubscriptionRequestType&nbsp;(263)</a> = Snapshot + Updates (1).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "265")]
	pub md_update_type: Option<MDUpdateType>,
	/// AggregatedBook
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "266")]
	pub aggregated_book: Option<AggregatedBook>,
	/// Number of <a href="tag_269_MDEntryType.html" target="bottom">MDEntryType&nbsp;(269)</a> fields requested.
	#[serde(rename = "267")]
	pub md_entry_types: fix_common::RepeatingValues<MDEntryType>,
	/// Number of symbols requested.
	#[serde(rename = "146")]
	pub related_sym: fix_common::RepeatingValues<RelatedSy>,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MDEntryType {
	/// Must be the first field in this repeating group. This is a list of all the types of Market Data Entries that the firm requesting
	/// the Market Data is interested in receiving.
	#[serde(rename = "269")]
	pub md_entry_type_item: MDEntryTypeItem,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RelatedSy {
	/// Must be the first field in the repeating group.
	#[serde(rename = "55")]
	pub symbol: String,
	/// Can be repeated multiple times if message is related to multiple symbols.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "65")]
	pub symbol_sfx: Option<String>,
	/// Can be repeated multiple times if message is related to multiple symbols.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "48")]
	pub security_id: Option<String>,
	/// Can be repeated multiple times if message is related to multiple symbols.
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
	/// Can be repeated multiple times if message is related to multiple symbols.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "106")]
	pub issuer: Option<String>,
	/// Must be set if <a href="tag_349_EncodedIssuer.html" target="bottom">EncodedIssuer&nbsp;(349)</a> field is specified and must immediately precede it.
	#[serde(rename = "348")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_106_Issuer.html" target="bottom">Issuer&nbsp;(106)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "349")]
	pub encoded_issuer: Option<fix_common::EncodedText<349>>,
	/// Can be repeated multiple times if message is related to multiple symbols.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "107")]
	pub security_desc: Option<String>,
	/// Must be set if <a href="tag_351_EncodedSecurityDesc.html" target="bottom">EncodedSecurityDesc&nbsp;(351)</a> field is specified and must immediately precede it.
	#[serde(rename = "350")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_107_SecurityDesc.html" target="bottom">SecurityDesc&nbsp;(107)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "351")]
	pub encoded_security_desc: Option<fix_common::EncodedText<351>>,
	/// TradingSessionID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "336")]
	pub trading_session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for SubscriptionRequestType {
	fn default() -> Self {
		SubscriptionRequestType::Snapshot
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDUpdateType {
	/// Full Refresh
	#[serde(rename = "0")]
	FullRefresh,
	/// Incremental Refresh
	#[serde(rename = "1")]
	IncrementalRefresh,
}

impl Default for MDUpdateType {
	fn default() -> Self {
		MDUpdateType::FullRefresh
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum AggregatedBook {
	/// one book entry per side per price
	#[serde(rename = "Y")]
	OneBookEntryPerSidePerPrice,
	/// Multiple entries per side per price allowed
	#[serde(rename = "N")]
	MultipleEntriesPerSidePerPriceAllowed,
}

impl Default for AggregatedBook {
	fn default() -> Self {
		AggregatedBook::OneBookEntryPerSidePerPrice
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MDEntryTypeItem {
	/// Bid
	#[serde(rename = "0")]
	Bid,
	/// Offer
	#[serde(rename = "1")]
	Offer,
	/// Trade
	#[serde(rename = "2")]
	Trade,
	/// Index Value
	#[serde(rename = "3")]
	IndexValue,
	/// Opening Price
	#[serde(rename = "4")]
	OpeningPrice,
	/// Closing Price
	#[serde(rename = "5")]
	ClosingPrice,
	/// Settlement Price
	#[serde(rename = "6")]
	SettlementPrice,
	/// Trading Session High Price
	#[serde(rename = "7")]
	TradingSessionHighPrice,
	/// Trading Session Low Price
	#[serde(rename = "8")]
	TradingSessionLowPrice,
	/// Trading Session VWAP Price
	#[serde(rename = "9")]
	TradingSessionVwapPrice,
}

impl Default for MDEntryTypeItem {
	fn default() -> Self {
		MDEntryTypeItem::Bid
	}
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
