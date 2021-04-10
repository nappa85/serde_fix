
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionParties {
	/// NoProvisionPartyIDs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40174")]
	pub provision_party_i_ds: Option<crate::entities::RepeatingValues<ProvisionPartyID>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionPartyID {
	/// Required if NoProvisionPartyIDs(40174) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40175")]
	pub provision_party_id: Option<String>,
	/// Required if NoProvisionPartyIDs(40174) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40176")]
	pub provision_party_id_source: Option<ProvisionPartyIDSource>,
	/// Required if NoProvisionPartyIDs(40174) &gt; 0. The same roles as PartyRole (452).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40177")]
	pub provision_party_role: Option<ProvisionPartyRole>,
	/// ProvisionPartyRoleQualifier
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "2385")]
	pub provision_party_role_qualifier: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionPartyIDSource {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionPartyRole {
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
