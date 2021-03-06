
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetParties {
	/// Repeating group below should contain unique combinations of <a href="tag_1462_TargetPartyID.html" target="bottom">TargetPartyID&nbsp;(1462)</a> , <a href="tag_1463_TargetPartyIDSource.html" target="bottom">TargetPartyIDSource&nbsp;(1463)</a> , and <a href="tag_1464_TargetPartyRole.html" target="bottom">TargetPartyRole&nbsp;(1464)</a> .
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1461")]
	pub target_party_i_ds: Option<fix_common::RepeatingValues<TargetPartyID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetPartyID {
	/// Required if NoTargetPartyIDs &gt; 0. Used to identify PartyID targeted for the action specified in the message.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1462")]
	pub target_party_id: Option<String>,
	/// Used to identify source of target party id.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1463")]
	pub target_party_id_source: Option<TargetPartyIDSource>,
	/// Used to identify the role of target party id
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1464")]
	pub target_party_role: Option<TargetPartyRole>,
	/// Used to further qualify the role of target party role.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1818")]
	pub target_party_role_qualifier: Option<TargetPartyRoleQualifier>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TargetPartyIDSource {
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
	/// Market Identifier Code (ISO 10383) MIC
	#[serde(rename = "G")]
	MarketIdentifierCodeMic,
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
	/// Tax ID
	#[serde(rename = "J")]
	TaxId,
	/// Australian Company Number
	#[serde(rename = "K")]
	AustralianCompanyNumber,
	/// Australian Registered Body Number
	#[serde(rename = "L")]
	AustralianRegisteredBodyNumber,
	/// CFTC reporting firm identifier
	#[serde(rename = "M")]
	CftcReportingFirmIdentifier,
	/// Legal Entity Identifier (ISO 17442) LEI
	#[serde(rename = "N")]
	LegalEntityIdentifierLei,
	/// Interim identifier (An interim entity identifier assigned by a regulatory agency prior to an LEI (ISO 17442) being assigned.
	#[serde(rename = "O")]
	InterimIdentifierBeingAssigned,
	/// Short code identifier
	#[serde(rename = "P")]
	ShortCodeIdentifier,
	/// National ID of natural person
	#[serde(rename = "Q")]
	NationalIdOfNaturalPerson,
	/// India Permanent Account Number (Also referred to as PAN ID. An identifier issued by the Income Tax Department of India)
	#[serde(rename = "R")]
	IndiaPermanentAccountNumber,
	/// Firm designated identifier
	#[serde(rename = "S")]
	FirmDesignatedIdentifier,
	/// Special Segregated Account ID
	#[serde(rename = "T")]
	SpecialSegregatedAccountId,
	/// Master Special Segregated Account ID
	#[serde(rename = "U")]
	MasterSpecialSegregatedAccountId,
}

impl Default for TargetPartyIDSource {
	fn default() -> Self {
		TargetPartyIDSource::BicCode
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TargetPartyRole {
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
	/// Reporting intermediary
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
	/// Desk ID
	#[serde(rename = "76")]
	DeskId,
	/// Market data market
	#[serde(rename = "77")]
	MarketDataMarket,
	/// Allocation Entity
	#[serde(rename = "78")]
	AllocationEntity,
	/// Prime Broker providing General Trade Services
	#[serde(rename = "79")]
	PrimeBrokerProvidingGeneralTradeServices,
	/// Step-Out Firm (Prime Broker)
	#[serde(rename = "80")]
	StepOutFirm,
	/// Broker client ID
	#[serde(rename = "81")]
	BrokerClientId,
	/// Central Registration Depository (CRD)
	#[serde(rename = "82")]
	CentralRegistrationDepository,
	/// Clearing Account
	#[serde(rename = "83")]
	ClearingAccount,
	/// Acceptable Settling Counterparty
	#[serde(rename = "84")]
	AcceptableSettlingCounterparty,
	/// Unacceptable Settling Counterparty
	#[serde(rename = "85")]
	UnacceptableSettlingCounterparty,
	/// CLS Member Bank
	#[serde(rename = "86")]
	ClsMemberBank,
	/// In Concert Group
	#[serde(rename = "87")]
	InConcertGroup,
	/// In Concert Controlling Entity
	#[serde(rename = "88")]
	InConcertControllingEntity,
	/// Large Positions Reporting Account
	#[serde(rename = "89")]
	LargePositionsReportingAccount,
	/// SettlementFirm
	#[serde(rename = "90")]
	SettlementFirm,
	/// SettlementAccount
	#[serde(rename = "91")]
	SettlementAccount,
	/// Reporting Market Center
	#[serde(rename = "92")]
	ReportingMarketCenter,
	/// Related Reporting Market Center
	#[serde(rename = "93")]
	RelatedReportingMarketCenter,
	/// Away Market
	#[serde(rename = "94")]
	AwayMarket,
	/// Give-up trading firm
	#[serde(rename = "95")]
	GiveUpTradingFirm,
	/// Takeup trading firm
	#[serde(rename = "96")]
	TakeupTradingFirm,
	/// Give-up clearing firm
	#[serde(rename = "97")]
	GiveUpClearingFirm,
	/// Take-up clearing firm
	#[serde(rename = "98")]
	TakeUpClearingFirm,
	/// Originating Market
	#[serde(rename = "99")]
	OriginatingMarket,
	/// Margin account (Elaboration: Also referred to as "performance bond account". The margin account is the calculated margin requirements.
	/// Typically represents the aggregation of one or more position accounts)
	#[serde(rename = "100")]
	MarginAccount,
	/// Collateral asset account (Elaboration: The account at which individual collateral assets are maintained. Typically, although
	/// not always, one-for-one with the settlement account)
	#[serde(rename = "101")]
	CollateralAssetAccount,
	/// Data repository
	#[serde(rename = "102")]
	DataRepository,
	/// Calculation agent
	#[serde(rename = "103")]
	CalculationAgent,
	/// Sender of exercise notice
	#[serde(rename = "104")]
	SenderOfExerciseNotice,
	/// Receiver of exercise notice
	#[serde(rename = "105")]
	ReceiverOfExerciseNotice,
	/// Rate reference bank
	#[serde(rename = "106")]
	RateReferenceBank,
	/// Correspondent
	#[serde(rename = "107")]
	Correspondent,
	/// Beneficiary's bank or depository institution
	#[serde(rename = "109")]
	BeneficiarySBankOrDepositoryInstitution,
	/// Borrower
	#[serde(rename = "110")]
	Borrower,
	/// Primary obligator
	#[serde(rename = "111")]
	PrimaryObligator,
	/// Guarantor
	#[serde(rename = "112")]
	Guarantor,
	/// Excluded reference entity
	#[serde(rename = "113")]
	ExcludedReferenceEntity,
	/// Determining party
	#[serde(rename = "114")]
	DeterminingParty,
	/// Hedging party
	#[serde(rename = "115")]
	HedgingParty,
	/// Reporting entity
	#[serde(rename = "116")]
	ReportingEntity,
	/// Sales person
	#[serde(rename = "117")]
	SalesPerson,
	/// Operator
	#[serde(rename = "118")]
	Operator,
	/// Central Securities Depository (CSD)
	#[serde(rename = "119")]
	CentralSecuritiesDepository,
	/// International Central Securities Depository (ICSD)
	#[serde(rename = "120")]
	InternationalCentralSecuritiesDepository,
	/// Trading sub-account
	#[serde(rename = "121")]
	TradingSubAccount,
	/// Investment decision maker
	#[serde(rename = "122")]
	InvestmentDecisionMaker,
	/// Publishing intermediary
	#[serde(rename = "123")]
	PublishingIntermediary,
	/// Central Securities Depository (CSD) Participant
	#[serde(rename = "124")]
	CentralSecuritiesDepositoryParticipant,
	/// Issuer
	#[serde(rename = "125")]
	Issuer,
	/// Contra Customer Account
	#[serde(rename = "126")]
	ContraCustomerAccount,
	/// Contra Investment Decision Maker
	#[serde(rename = "127")]
	ContraInvestmentDecisionMaker,
}

impl Default for TargetPartyRole {
	fn default() -> Self {
		TargetPartyRole::ExecutingFirm
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TargetPartyRoleQualifier {
	/// Agency
	#[serde(rename = "0")]
	Agency,
	/// Principal
	#[serde(rename = "1")]
	Principal,
	/// Riskless Principal
	#[serde(rename = "2")]
	RisklessPrincipal,
	/// General Clearing Member
	#[serde(rename = "3")]
	GeneralClearingMember,
	/// Individual Clearing Member
	#[serde(rename = "4")]
	IndividualClearingMember,
	/// Preferred Market Maker
	#[serde(rename = "5")]
	PreferredMarketMaker,
	/// Directed Market Maker
	#[serde(rename = "6")]
	DirectedMarketMaker,
	/// Bank
	#[serde(rename = "7")]
	Bank,
	/// Hub
	#[serde(rename = "8")]
	Hub,
	/// Primary trade repository
	#[serde(rename = "9")]
	PrimaryTradeRepository,
	/// Original trade repository
	#[serde(rename = "10")]
	OriginalTradeRepository,
	/// Additional international trade repository
	#[serde(rename = "11")]
	AdditionalInternationalTradeRepository,
	/// Additional domestic trade repository
	#[serde(rename = "12")]
	AdditionalDomesticTradeRepository,
	/// Related exchange
	#[serde(rename = "13")]
	RelatedExchange,
	/// Options exchange
	#[serde(rename = "14")]
	OptionsExchange,
	/// Specified exchange
	#[serde(rename = "15")]
	SpecifiedExchange,
	/// Constituent exchange
	#[serde(rename = "16")]
	ConstituentExchange,
	/// Exempt from trade reporting
	#[serde(rename = "17")]
	ExemptFromTradeReporting,
	/// Current
	#[serde(rename = "18")]
	Current,
	/// New
	#[serde(rename = "19")]
	New,
	/// Designated sponsor
	#[serde(rename = "20")]
	DesignatedSponsor,
	/// Specialist
	#[serde(rename = "21")]
	Specialist,
	/// Algorithm
	#[serde(rename = "22")]
	Algorithm,
	/// Firm or legal entity
	#[serde(rename = "23")]
	FirmOrLegalEntity,
	/// Natural person
	#[serde(rename = "24")]
	NaturalPerson,
	/// Regular trader
	#[serde(rename = "25")]
	RegularTrader,
	/// Head trader
	#[serde(rename = "26")]
	HeadTrader,
	/// Supervisor
	#[serde(rename = "27")]
	Supervisor,
	/// Tri-party
	#[serde(rename = "28")]
	TriParty,
	/// Lender
	#[serde(rename = "29")]
	Lender,
}

impl Default for TargetPartyRoleQualifier {
	fn default() -> Self {
		TargetPartyRoleQualifier::Agency
	}
}
