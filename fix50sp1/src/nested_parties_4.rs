
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NestedParties4 {
	/// Repeating group below should contain unique combinations of Nested4PartyID, Nested4PartyIDSource, and Nested4PartyRole
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1414")]
	pub nested_4_party_i_ds: Option<fix_common::RepeatingValues<Nested4PartyID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Nested4PartyID {
	/// Used to identify source of Nested4PartyID. Required if <a href="tag_1416_Nested4PartyIDSource.html" target="bottom">Nested4PartyIDSource&nbsp;(1416)</a> is specified. Required if <a href="tag_1414_NoNested4PartyIDs.html" target="bottom">NoNested4PartyIDs&nbsp;(1414)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1415")]
	pub nested_4_party_id: Option<String>,
	/// Used to identify class source of <a href="tag_1415_Nested4PartyID.html" target="bottom">Nested4PartyID&nbsp;(1415)</a> value (e.g. BIC). Required if <a href="tag_1415_Nested4PartyID.html" target="bottom">Nested4PartyID&nbsp;(1415)</a> is specified. Required if <a href="tag_1414_NoNested4PartyIDs.html" target="bottom">NoNested4PartyIDs&nbsp;(1414)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1416")]
	pub nested_4_party_id_source: Option<Nested4PartyIDSource>,
	/// Identifies the type of <a href="tag_1415_Nested4PartyID.html" target="bottom">Nested4PartyID&nbsp;(1415)</a> (e.g. Executing Broker). Required if <a href="tag_1414_NoNested4PartyIDs.html" target="bottom">NoNested4PartyIDs&nbsp;(1414)</a> &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1417")]
	pub nested_4_party_role: Option<Nested4PartyRole>,
	/// NoNested4PartySubIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1413")]
	pub no_nested_4_party_sub_i_ds: Option<usize>,
	/// Nested4PartySubID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1412")]
	pub nested_4_party_sub_id: Option<String>,
	/// Nested4PartySubIDType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1411")]
	pub nested_4_party_sub_id_type: Option<Nested4PartySubIDType>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Nested4PartyIDSource {
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

impl Default for Nested4PartyIDSource {
	fn default() -> Self {
		Nested4PartyIDSource::BicCode
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Nested4PartyRole {
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
	/// Sub-custodian
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
	/// Contra trader
	#[serde(rename = "37")]
	ContraTrader,
	/// Position account
	#[serde(rename = "38")]
	PositionAccount,
	/// Contra Investor ID
	#[serde(rename = "39")]
	ContraInvestorId,
	/// Transfer to Firm
	#[serde(rename = "40")]
	TransferToFirm,
	/// Contra Position Account
	#[serde(rename = "41")]
	ContraPositionAccount,
	/// Contra Exchange
	#[serde(rename = "42")]
	ContraExchange,
	/// Internal Carry Account
	#[serde(rename = "43")]
	InternalCarryAccount,
	/// Order Entry Operator ID
	#[serde(rename = "44")]
	OrderEntryOperatorId,
	/// Secondary Account Number
	#[serde(rename = "45")]
	SecondaryAccountNumber,
	/// Foriegn Firm
	#[serde(rename = "46")]
	ForiegnFirm,
	/// Third Party Allocation Firm
	#[serde(rename = "47")]
	ThirdPartyAllocationFirm,
	/// Claiming Account
	#[serde(rename = "48")]
	ClaimingAccount,
	/// Asset Manager
	#[serde(rename = "49")]
	AssetManager,
	/// Pledgor Account
	#[serde(rename = "50")]
	PledgorAccount,
	/// Pledgee Account
	#[serde(rename = "51")]
	PledgeeAccount,
	/// Large Trader Reportable Account
	#[serde(rename = "52")]
	LargeTraderReportableAccount,
	/// Trader mnemonic
	#[serde(rename = "53")]
	TraderMnemonic,
	/// Sender Location
	#[serde(rename = "54")]
	SenderLocation,
	/// Session ID
	#[serde(rename = "55")]
	SessionId,
	/// Acceptable Counterparty
	#[serde(rename = "56")]
	AcceptableCounterparty,
	/// Unacceptable Counterparty
	#[serde(rename = "57")]
	UnacceptableCounterparty,
	/// Entering Unit
	#[serde(rename = "58")]
	EnteringUnit,
	/// Executing Unit
	#[serde(rename = "59")]
	ExecutingUnit,
	/// Introducing Broker
	#[serde(rename = "60")]
	IntroducingBroker,
	/// Quote originator
	#[serde(rename = "61")]
	QuoteOriginator,
	/// Report originator
	#[serde(rename = "62")]
	ReportOriginator,
	/// Systematic internaliser (SI)
	#[serde(rename = "63")]
	SystematicInternaliser,
	/// Multilateral Trading Facility (MTF)
	#[serde(rename = "64")]
	MultilateralTradingFacility,
	/// Regulated Market (RM)
	#[serde(rename = "65")]
	RegulatedMarket,
	/// Market Maker
	#[serde(rename = "66")]
	MarketMaker,
	/// Investment Firm
	#[serde(rename = "67")]
	InvestmentFirm,
	/// Host Competent Authority (Host CA)
	#[serde(rename = "68")]
	HostCompetentAuthority,
	/// Home Competent Authority (Home CA)
	#[serde(rename = "69")]
	HomeCompetentAuthority,
	/// Competent Authority of the most relevant market in terms of liquidity (CAL)
	#[serde(rename = "70")]
	CompetentAuthorityOfTheMostRelevantMarketInTermsOfLiquidity,
	/// Competent Authority of the Transaction (Execution) Venue (CATV)
	#[serde(rename = "71")]
	CompetentAuthorityOfTheTransactionVenue,
	/// Reporting intermediary (medium/vendor via which report has been published)
	#[serde(rename = "72")]
	ReportingIntermediary,
	/// Execution Venue
	#[serde(rename = "73")]
	ExecutionVenue,
	/// Market data entry originator
	#[serde(rename = "74")]
	MarketDataEntryOriginator,
	/// Location ID
	#[serde(rename = "75")]
	LocationId,
	/// Prime Broker providing General Trade Services
	#[serde(rename = "79")]
	PrimeBrokerProvidingGeneralTradeServices,
	/// Step-Out Firm (Prime Broker)
	#[serde(rename = "80")]
	StepOutFirm,
	/// BrokerClearingID
	#[serde(rename = "81")]
	BrokerClearingId,
}

impl Default for Nested4PartyRole {
	fn default() -> Self {
		Nested4PartyRole::ExecutingFirm
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Nested4PartySubIDType {
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
	/// Postal address
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
	/// CSD participant member code
	#[serde(rename = "17")]
	N17,
	/// Registered address
	#[serde(rename = "18")]
	N18,
	/// Fund account name
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
	/// Location desk
	#[serde(rename = "25")]
	N25,
	/// Position account type
	#[serde(rename = "26")]
	N26,
	/// Security locate ID
	#[serde(rename = "27")]
	N27,
	/// Market maker
	#[serde(rename = "28")]
	N28,
	/// Eligible counterparty
	#[serde(rename = "29")]
	N29,
	/// Professional client
	#[serde(rename = "30")]
	N30,
	/// Location
	#[serde(rename = "31")]
	N31,
	/// Execution venue
	#[serde(rename = "32")]
	N32,
	/// Currency delivery identifier
	#[serde(rename = "33")]
	N33,
}

impl Default for Nested4PartySubIDType {
	fn default() -> Self {
		Nested4PartySubIDType::N1
	}
}
