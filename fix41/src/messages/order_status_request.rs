
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderStatusRequest {
	/// MsgType = H
	#[serde(flatten)]
	pub standard_message_header: super::super::standard_message_header::StandardMessageHeader<'H'>,
	/// OrderID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "37")]
	pub order_id: Option<char>,
	/// ClOrdID
	#[serde(rename = "11")]
	pub cl_ord_id: String,
	/// Used for firm identification in third-party transactions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "109")]
	pub client_id: Option<char>,
	/// Used for firm identification in third-party transactions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "76")]
	pub exec_broker: Option<char>,
	/// Symbol
	#[serde(rename = "55")]
	pub symbol: String,
	/// SymbolSfx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "65")]
	pub symbol_sfx: Option<char>,
	/// SecurityID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "48")]
	pub security_id: Option<char>,
	/// IDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "22")]
	pub id_source: Option<IDSource>,
	/// Must be specified if a Future or Option. If a Future: <a href="tag_55_Symbol.html" target="bottom">Symbol&nbsp;(55)</a> , <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> , and <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> are required. If an Option: <a href="tag_55_Symbol.html" target="bottom">Symbol&nbsp;(55)</a> , <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> , <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> , <a href="tag_201_PutOrCall.html" target="bottom">PutOrCall&nbsp;(201)</a> , and <a href="tag_202_StrikePrice.html" target="bottom">StrikePrice&nbsp;(202)</a> are required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// For Options or Futures to specify the month and year of maturity.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "200")]
	pub maturity_month_year: Option<fix_common::MonthYear>,
	/// For Options or Futures and can be used in conjunction with <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> to specify a particular maturity date.
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
	/// Can be used to identify the security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "207")]
	pub security_exchange: Option<char>,
	/// Issuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "106")]
	pub issuer: Option<char>,
	/// SecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "107")]
	pub security_desc: Option<char>,
	/// Side
	#[serde(rename = "54")]
	pub side: Side,
	/// Standard Message Trailer
	#[serde(flatten)]
	pub standard_message_trailer: super::super::standard_message_trailer::StandardMessageTrailer,
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
	/// Treasuries + Agency Debenture
	#[serde(rename = "GOVT")]
	TreasuriesAgencyDebenture,
	/// IOETTE Mortgage
	#[serde(rename = "IET")]
	IoetteMortgage,
	/// Mutual Fund
	#[serde(rename = "MF")]
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
pub enum Side {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
	/// Buy minus
	#[serde(rename = "3")]
	BuyMinus,
	/// Sell plus
	#[serde(rename = "4")]
	SellPlus,
	/// Sell short
	#[serde(rename = "5")]
	SellShort,
	/// Sell short exempt
	#[serde(rename = "6")]
	SellShortExempt,
	/// Undisclosed (valid for <a href="message_Indication_of_Interest_6.html" target="main">Indication of Interest&nbsp;(6)</a> and <a href="message_New_Order_List_E.html" target="main">List Order&nbsp;(E)</a> messages only)
	#[serde(rename = "7")]
	UndisclosedAAndAHrefMessageNewOrderListEHtmlTargetMainListOrderNbspAMessagesOnly,
	/// Cross (orders where counterparty is an exchange, valid for all messages except IOIs)
	#[serde(rename = "8")]
	Cross,
}

impl Default for Side {
	fn default() -> Self {
		Side::Buy
	}
}
