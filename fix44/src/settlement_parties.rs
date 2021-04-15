
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlementParties {
	/// Repeating group below should contain unique combinations of <a href="tag_782_SettlPartyID.html" target="bottom">SettlPartyID&nbsp;(782)</a> , <a href="tag_783_SettlPartyIDSource.html" target="bottom">SettlPartyIDSource&nbsp;(783)</a> , and <a href="tag_784_SettlPartyRole.html" target="bottom">SettlPartyRole&nbsp;(784)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "781")]
	pub settl_party_i_ds: Option<fix_common::RepeatingValues<SettlPartyID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettlPartyID {
	/// Used to identify source of SettlPartyID. Required if <a href="tag_783_SettlPartyIDSource.html" target="bottom">SettlPartyIDSource&nbsp;(783)</a> is specified. Required if <a href="tag_781_NoSettlPartyIDs.html" target="bottom">NoSettlPartyIDs&nbsp;(781)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "782")]
	pub settl_party_id: Option<String>,
	/// Used to identify class source of SettlPartyID value (e.g. BIC). Required if <a href="tag_782_SettlPartyID.html" target="bottom">SettlPartyID&nbsp;(782)</a> is specified. Required if <a href="tag_781_NoSettlPartyIDs.html" target="bottom">NoSettlPartyIDs&nbsp;(781)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "783")]
	pub settl_party_id_source: Option<SettlPartyIDSource>,
	/// Identifies the type of SettlPartyID (e.g. Executing Broker). Required if <a href="tag_781_NoSettlPartyIDs.html" target="bottom">NoSettlPartyIDs&nbsp;(781)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "784")]
	pub settl_party_role: Option<SettlPartyRole>,
	/// Repeating group of SettlParty sub-identifiers.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "801")]
	pub no_settl_party_sub_i_ds: Option<usize>,
	/// Sub-identifier (e.g. Clearing Acct for <a href="tag_782_SettlPartyID.html" target="bottom">SettlPartyID&nbsp;(782)</a> =Clearing Firm) if applicable. Required if <a href="tag_801_NoSettlPartySubIDs.html" target="bottom">NoSettlPartySubIDs&nbsp;(801)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "785")]
	pub settl_party_sub_id: Option<String>,
	/// Type of Sub-identifier. Required if <a href="tag_801_NoSettlPartySubIDs.html" target="bottom">NoSettlPartySubIDs&nbsp;(801)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "786")]
	pub settl_party_sub_id_type: Option<SettlPartySubIDType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlPartyIDSource {
	/// BIC (Bank Identification Code-Swift managed) code (ISO 9362)
	#[serde(rename = "B")]
	BicCode,
	/// Generally accepted market participant identifier (e.g. NASD mnemonic)
	#[serde(rename = "C")]
	GenerallyAcceptedMarketParticipantIdentifier,
	/// Proprietary/Custom code
	#[serde(rename = "D")]
	ProprietaryCustomCode,
	/// ISO Country Code
	#[serde(rename = "E")]
	IsoCountryCode,
	/// Settlement Entity Location (note if Local Market Settlement use "E = ISO Country Code")
	#[serde(rename = "F")]
	SettlementEntityLocation,
	/// MIC (ISO 10383 - Market Identifier Code) (See " <a xmlns="http://www.b2bits.com/FIXProtocol" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" href="../appendices/appendix_6-c.html" target="_blank">Appendix 6-C</a> " of FIX Specification)
	#[serde(rename = "G")]
	Mic,
	/// CSD participant/member code (e.g. Euroclear, DTC, CREST or Kassenverein number)
	#[serde(rename = "H")]
	CsdParticipantMemberCode,
	/// Korean Investor ID
	#[serde(rename = "1")]
	KoreanInvestorId,
	/// Taiwanese Qualified Foreign Investor ID QFII / FID
	#[serde(rename = "2")]
	TaiwaneseQualifiedForeignInvestorIdQfiiFid,
	/// Taiwanese Trading Account
	#[serde(rename = "3")]
	TaiwaneseTradingAccount,
	/// Malaysian Central Depository (MCD) number
	#[serde(rename = "4")]
	MalaysianCentralDepositoryNumber,
	/// Chinese B Share (Shezhen and Shanghai)
	#[serde(rename = "5")]
	ChineseBShare,
	/// UK National Insurance or Pension Number
	#[serde(rename = "6")]
	UkNationalInsuranceOrPensionNumber,
	/// US Social Security Number
	#[serde(rename = "7")]
	UsSocialSecurityNumber,
	/// US Employer Identification Number
	#[serde(rename = "8")]
	UsEmployerIdentificationNumber,
	/// Australian Business Number
	#[serde(rename = "9")]
	AustralianBusinessNumber,
	/// Australian Tax File Number
	#[serde(rename = "A")]
	AustralianTaxFileNumber,
	/// Directed broker three character acronym as defined in ISITC 'ETC Best Practice' guidelines document
	#[serde(rename = "I")]
	DirectedBrokerThreeCharacterAcronymAsDefinedInIsitcEtcBestPracticeGuidelinesDocument,
}

impl Default for SettlPartyIDSource {
	fn default() -> Self {
		SettlPartyIDSource::BicCode
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlPartyRole {
	/// Executing Firm (formerly FIX 4.2 ExecBroker)
	#[serde(rename = "1")]
	ExecutingFirm,
	/// Broker of Credit (formerly FIX 4.2 BrokerOfCredit)
	#[serde(rename = "2")]
	BrokerOfCredit,
	/// Client ID (formerly FIX 4.2 ClientID)
	#[serde(rename = "3")]
	ClientId,
	/// Clearing Firm (formerly FIX 4.2 ClearingFirm)
	#[serde(rename = "4")]
	ClearingFirm,
	/// Investor ID
	#[serde(rename = "5")]
	InvestorId,
	/// Introducing Firm
	#[serde(rename = "6")]
	IntroducingFirm,
	/// Entering Firm
	#[serde(rename = "7")]
	EnteringFirm,
	/// Locate/Lending Firm (for short-sales)
	#[serde(rename = "8")]
	LocateLendingFirm,
	/// Fund manager Client ID (for CIV)
	#[serde(rename = "9")]
	FundManagerClientId,
	/// Settlement Location (formerly FIX 4.2 SettlLocation)
	#[serde(rename = "10")]
	SettlementLocation,
	/// Order Origination Trader (associated with Order Origination Firm - e.g. trader who initiates/submits the order)
	#[serde(rename = "11")]
	OrderOriginationTrader,
	/// Executing Trader (associated with Executing Firm - actually executes)
	#[serde(rename = "12")]
	ExecutingTrader,
	/// Order Origination Firm (e.g. buyside firm)
	#[serde(rename = "13")]
	OrderOriginationFirm,
	/// Giveup Clearing Firm (firm to which trade is given up)
	#[serde(rename = "14")]
	GiveupClearingFirm,
	/// Correspondant Clearing Firm
	#[serde(rename = "15")]
	CorrespondantClearingFirm,
	/// Executing System
	#[serde(rename = "16")]
	ExecutingSystem,
	/// Contra Firm
	#[serde(rename = "17")]
	ContraFirm,
	/// Contra Clearing Firm
	#[serde(rename = "18")]
	ContraClearingFirm,
	/// Sponsoring Firm
	#[serde(rename = "19")]
	SponsoringFirm,
	/// Underlying Contra Firm
	#[serde(rename = "20")]
	UnderlyingContraFirm,
	/// Clearing Organization
	#[serde(rename = "21")]
	ClearingOrganization,
	/// Exchange
	#[serde(rename = "22")]
	Exchange,
	/// Customer Account
	#[serde(rename = "24")]
	CustomerAccount,
	/// Correspondent Clearing Organization
	#[serde(rename = "25")]
	CorrespondentClearingOrganization,
	/// Correspondent Broker
	#[serde(rename = "26")]
	CorrespondentBroker,
	/// Buyer/Seller (Receiver/Deliverer)
	#[serde(rename = "27")]
	BuyerSeller,
	/// Custodian
	#[serde(rename = "28")]
	Custodian,
	/// Intermediary
	#[serde(rename = "29")]
	Intermediary,
	/// Agent
	#[serde(rename = "30")]
	Agent,
	/// Sub custodian
	#[serde(rename = "31")]
	SubCustodian,
	/// Beneficiary
	#[serde(rename = "32")]
	Beneficiary,
	/// Interested party
	#[serde(rename = "33")]
	InterestedParty,
	/// Regulatory body
	#[serde(rename = "34")]
	RegulatoryBody,
	/// Liquidity provider
	#[serde(rename = "35")]
	LiquidityProvider,
	/// Entering trader
	#[serde(rename = "36")]
	EnteringTrader,
	/// Contra Trader
	#[serde(rename = "37")]
	ContraTrader,
	/// Position Account
	#[serde(rename = "38")]
	PositionAccount,
}

impl Default for SettlPartyRole {
	fn default() -> Self {
		SettlPartyRole::ExecutingFirm
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlPartySubIDType {
	/// Firm
	#[serde(rename = "1")]
	N1,
	/// Person
	#[serde(rename = "2")]
	N2,
	/// System
	#[serde(rename = "3")]
	N3,
	/// Application
	#[serde(rename = "4")]
	N4,
	/// Full legal name of firm
	#[serde(rename = "5")]
	N5,
	/// Postal address (inclusive of street address, location, and postal code)
	#[serde(rename = "6")]
	N6,
	/// Phone number
	#[serde(rename = "7")]
	N7,
	/// Email address
	#[serde(rename = "8")]
	N8,
	/// Contact name
	#[serde(rename = "9")]
	N9,
	/// Securities account number (for settlement instructions)
	#[serde(rename = "10")]
	N10,
	/// Registration number (for settlement instructions and confirmations)
	#[serde(rename = "11")]
	N11,
	/// Registered address (for confirmation purposes)
	#[serde(rename = "12")]
	N12,
	/// Regulatory status (for confirmation purposes)
	#[serde(rename = "13")]
	N13,
	/// Registration name (for settlement instructions)
	#[serde(rename = "14")]
	N14,
	/// Cash account number (for settlement instructions)
	#[serde(rename = "15")]
	N15,
	/// BIC
	#[serde(rename = "16")]
	N16,
	/// CSD participant/member code (e.g. Euroclear, DTC, CREST or Kassenverein number)
	#[serde(rename = "17")]
	N17,
	/// Registered address
	#[serde(rename = "18")]
	N18,
	/// Fund/account name
	#[serde(rename = "19")]
	N19,
	/// Telex number
	#[serde(rename = "20")]
	N20,
	/// Fax number
	#[serde(rename = "21")]
	N21,
	/// Securities account name
	#[serde(rename = "22")]
	N22,
	/// Cash account name
	#[serde(rename = "23")]
	N23,
	/// Department
	#[serde(rename = "24")]
	N24,
	/// Location / Desk
	#[serde(rename = "25")]
	N25,
	/// Position Account Type
	#[serde(rename = "26")]
	N26,
}

impl Default for SettlPartySubIDType {
	fn default() -> Self {
		SettlPartySubIDType::N1
	}
}
