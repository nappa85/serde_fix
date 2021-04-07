
use serde::{Serialize, Deserialize};

use crate::entities::RepeatingValues;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Parties {
    /// Repeating group below should contain unique combinations of PartyID, PartyIDSource, and PartyRole
    #[serde(rename = "453")]
    pub inner: RepeatingValues<Party>,
}

impl AsRef<Vec<Party>> for Parties {
    fn as_ref(&self) -> &Vec<Party> {
        self.inner.as_ref()
    }
}

impl AsMut<Vec<Party>> for Parties {
    fn as_mut(&mut self) -> &mut Vec<Party> {
        self.inner.as_mut()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Party {
    /// Required if NoPartyIDs(453) > 0. Identification of the party.
    #[serde(rename = "448")]
	#[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    /// Required if NoPartyIDs(453) > 0. Used to identify classification source.
    #[serde(rename = "447")]
	#[serde(skip_serializing_if = "Option::is_none")]
    pub party_id_source: Option<PartyIDSource>,
    /// Required if NoPartyIDs(453) > 0. Identifies the type of PartyID(448).
    #[serde(rename = "452")]
	#[serde(skip_serializing_if = "Option::is_none")]
    pub party_role: Option<PartyRole>,
    /// PartyRoleQualifier
    #[serde(rename = "2376")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
    pub party_role_qualifier: Option<i32>,
    /// PtysSubGrp
    #[serde(rename = "802")]
    #[serde(alias = "523")]
    #[serde(alias = "803")]
	#[serde(skip_serializing_if = "Option::is_none")]
    pub party_sub_ids: Option<RepeatingValues<PtysSubGrp>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PartyIDSource {
    /// BIC (Bank Identification Code-Swift managed) code (ISO 9362)
    #[serde(rename = "B")]
    BIC,
    /// Generally accepted market participant identifier (e.g. NASD mnemonic)
    #[serde(rename = "C")]
    GenerallyAcceptedMarketParticipantIdentifier,
    /// Proprietary/Custom code
    #[serde(rename = "D")]
    ProprietaryCustomCode,
    /// ISO Country Code
    #[serde(rename = "E")]
    ISOCountryCode,
    /// Settlement Entity Location (note if Local Market Settlement use "E = ISO Country Code")
    #[serde(rename = "F")]
    SettlementEntityLocation,
    /// Market Identifier Code (ISO 10383) MIC
    #[serde(rename = "G")]
    MarketIdentifierCodeMIC,
    /// CSD participant/member code (e.g. Euroclear, DTC, CREST or Kassenverein number)
    #[serde(rename = "H")]
    CSDParticipantMemberCode,
    /// Korean Investor ID
    #[serde(rename = "1")]
    KoreanInvestorID,
    /// Taiwanese Qualified Foreign Investor ID QFII / FID
    #[serde(rename = "2")]
    TaiwaneseQualifiedForeignInvestorID,
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
    UKNationalInsuranceOrPensionNumber,
    /// US Social Security Number
    #[serde(rename = "7")]
    USSocialSecurityNumber,
    /// US Employer Identification Number
    #[serde(rename = "8")]
    USEmployerIdentificationNumber,
    /// Australian Business Number
    #[serde(rename = "9")]
    AustralianBusinessNumber,
    /// Australian Tax File Number
    #[serde(rename = "A")]
    AustralianTaxFileNumber,
    /// Directed broker three character acronym as defined in ISITC 'ETC Best Practice' guidelines document
    #[serde(rename = "I")]
    DirectedBrokerThreeCharacterAcronym,
    /// Tax ID
    #[serde(rename = "J")]
    TaxID,
    /// Australian Company Number
    #[serde(rename = "K")]
    AustralianCompanyNumber,
    /// Australian Registered Body Number
    #[serde(rename = "L")]
    AustralianRegisteredBodyNumber,
    /// CFTC reporting firm identifier
    #[serde(rename = "M")]
    CFTCReportingFirmIdentifier,
    /// Legal Entity Identifier (ISO 17442) LEI
    #[serde(rename = "N")]
    LegalEntityIdentifier,
    /// Interim identifier (An interim entity identifier assigned by a regulatory agency prior to an LEI (ISO 17442) being assigned.
    #[serde(rename = "O")]
    InterimIdentifier,
    /// Short code identifier
    #[serde(rename = "P")]
    ShortCodeIdentifier,
    /// National ID of natural person
    #[serde(rename = "Q")]
    NationalIDOfNaturalPerson,
    /// India Permanent Account Number (Also referred to as PAN ID. An identifier issued by the Income Tax Department of India)
    #[serde(rename = "R")]
    IndiaPermanentAccountNumber,
    /// Firm designated identifier
    #[serde(rename = "S")]
    FirmDesignatedIdentifier,
    /// Special Segregated Account ID
    #[serde(rename = "T")]
    SpecialSegregatedAccountID,
    /// Master Special Segregated Account ID
    #[serde(rename = "U")]
    MasterSpecialSegregatedAccountID,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PartyRole {
    /// Executing Firm (formerly FIX 4.2 ExecBroker)
    #[serde(rename = "1")]
    ExecutingFirm,
    /// Broker of Credit (formerly FIX 4.2 BrokerOfCredit)
    #[serde(rename = "2")]
    BrokerOfCredit,
    /// Client ID (formerly FIX 4.2 ClientID)
    #[serde(rename = "3")]
    ClientID,
    /// Clearing Firm (formerly FIX 4.2 ClearingFirm)
    #[serde(rename = "4")]
    ClearingFirm,
    /// Investor ID
    #[serde(rename = "5")]
    InvestorID,
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
    FundManagerClientID,
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
    GivenupClearingFirm,
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
    ContraInvestorID,
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
    OrderEntryOperatorID,
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
    SessionID,
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
    LocationID,
    /// Desk ID
    #[serde(rename = "76")]
    DeskID,
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
    BrokerClientID,
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
    CLSMemberBank,
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
    GiveupTradingFirm,
    /// Takeup trading firm
    #[serde(rename = "96")]
    TakeupTradingFirm,
    /// Give-up clearing firm
    #[serde(rename = "97")]
    GiveupClearingFirm,
    /// Take-up clearing firm
    #[serde(rename = "98")]
    TakeupClearingFirm,
    /// Originating Market
    #[serde(rename = "99")]
    OriginatingMarket,
    /// Margin account (Elaboration: Also referred to as "performance bond account". The margin account is the calculated margin requirements. Typically represents the aggregation of one or more position accounts)
    #[serde(rename = "100")]
    MarginAccount,
    /// Collateral asset account (Elaboration: The account at which individual collateral assets are maintained. Typically, although not always, one-for-one with the settlement account)
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
    BeneficiarysBankOrDepositoryInstitution,
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

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct PtysSubGrp {
    /// PartySubID
    #[serde(rename = "523")]
    pub party_sub_id: Option<String>,
    /// PartySubIDType
    #[serde(rename = "803")]
    pub party_sub_id_type: Option<PartySubIDType>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PartySubIDType {
    /// Firm
    #[serde(rename = "1")]
    Firm,
    /// Person
    #[serde(rename = "2")]
    Person,
    /// System
    #[serde(rename = "3")]
    System,
    /// Application
    #[serde(rename = "4")]
    Application,
    /// Full legal name of firm
    #[serde(rename = "5")]
    FullLegalNameOfFirm,
    /// Postal address
    #[serde(rename = "6")]
    PostalAddress,
    /// Phone number
    #[serde(rename = "7")]
    PhoneNumber,
    /// Email address
    #[serde(rename = "8")]
    EmailAddress,
    /// Contact name
    #[serde(rename = "9")]
    ContactName,
    /// Securities account number (for settlement instructions)
    #[serde(rename = "10")]
    SecuritiesAccountNumber,
    /// Registration number (for settlement instructions and confirmations)
    #[serde(rename = "11")]
    RegistrationNumber,
    /// Registered address (for confirmation purposes)
    #[serde(rename = "12")]
    RegisteredAddressConfirmation,
    /// Regulatory status (for confirmation purposes)
    #[serde(rename = "13")]
    RegulatoryStatus,
    /// Registration name (for settlement instructions)
    #[serde(rename = "14")]
    RegistrationName,
    /// Cash account number (for settlement instructions)
    #[serde(rename = "15")]
    CashAccountNumber,
    /// BIC
    #[serde(rename = "16")]
    BIC,
    /// CSD participant member code
    #[serde(rename = "17")]
    CSDParticipantMemberCode,
    /// Registered address
    #[serde(rename = "18")]
    RegisteredAddress,
    /// Fund account name
    #[serde(rename = "19")]
    FundAccountName,
    /// Telex number
    #[serde(rename = "20")]
    TelexNumber,
    /// Fax number
    #[serde(rename = "21")]
    FaxNumber,
    /// Securities account name
    #[serde(rename = "22")]
    SecuritiesAccountName,
    /// Cash account name
    #[serde(rename = "23")]
    CashAccountName,
    /// Department
    #[serde(rename = "24")]
    Department,
    /// Location desk
    #[serde(rename = "25")]
    LocationDesk,
    /// Position account type
    #[serde(rename = "26")]
    PositionAccountType,
    /// Security locate ID
    #[serde(rename = "27")]
    SecurityLocateID,
    /// Market maker
    #[serde(rename = "28")]
    MarketMaker,
    /// Eligible counterparty
    #[serde(rename = "29")]
    EligibleCounterparty,
    /// Professional client
    #[serde(rename = "30")]
    ProfessionalClient,
    /// Location
    #[serde(rename = "31")]
    Location,
    /// Execution venue
    #[serde(rename = "32")]
    ExecutionVenue,
    /// Currency delivery identifier
    #[serde(rename = "33")]
    CurrencyDeliveryIdentifier,
    /// Address City
    #[serde(rename = "34")]
    AddressCity,
    /// Address State/Province
    #[serde(rename = "35")]
    AddressStateProvince,
    /// Address Postal Code
    #[serde(rename = "36")]
    AddressPostalCode,
    /// Address Street
    #[serde(rename = "37")]
    AddressStreet,
    /// Address Country (ISO country code)
    #[serde(rename = "38")]
    AddressCountry,
    /// ISO country code
    #[serde(rename = "39")]
    ISOCountryCode,
    /// MarketSegment
    #[serde(rename = "40")]
    MarketSegment,
    /// Customer account type
    #[serde(rename = "41")]
    CustomerAccountType,
    /// Omnibus account
    #[serde(rename = "42")]
    OmnibusAccount,
    /// Funds segregation type
    #[serde(rename = "43")]
    FundsSegregationType,
    /// Guarantee fund (Elaboration: Identifies a guarantee fund related to an account. Used when one account has multiple funds of collateral, each guaranteeing different positions. Can be used for PartyRole(452) = Customer Account(24))
    #[serde(rename = "44")]
    GuaranteeFund,
    /// Swap dealer
    #[serde(rename = "45")]
    SwapDealer,
    /// Major participant
    #[serde(rename = "46")]
    MajorParticipant,
    /// Financial entity
    #[serde(rename = "47")]
    FinancialEntity,
    /// U.S. person
    #[serde(rename = "48")]
    USPerson,
    /// Reporting entity indicator
    #[serde(rename = "49")]
    ReportingEntityIndicator,
    /// Elected clearing requirement exception
    #[serde(rename = "50")]
    ElectedClearingRequirementException,
    /// Business center
    #[serde(rename = "51")]
    BusinessCenter,
    /// Reference text
    #[serde(rename = "52")]
    ReferenceText,
    /// Short-marking exempt account
    #[serde(rename = "53")]
    ShortMarkingExemptAccount,
    /// Parent firm identifier (Implementation-specific identifier of this party's parent entity)
    #[serde(rename = "54")]
    ParentFirmIdentifier,
    /// Parent firm name
    #[serde(rename = "55")]
    ParentFirmName,
    /// Deal identifier (The internal identifier assigned to the trade by this party, particularly by a Clearing Organization)
    #[serde(rename = "56")]
    DealIdentifier,
    /// System trade identifier
    #[serde(rename = "57")]
    SystemTradeIdentifier,
    /// System trade sub-identifier
    #[serde(rename = "58")]
    SystemTradeSubIdentifier,
    /// Futures Commission Merchant (FCM) code (The FCM's code or identifier in relation to the PartyRole(452). For example, if PartyRole(452) is the exchange or clearinghouse, the FCM code/ID specified in PartySubID(523) is the FCM's identifier at the exchange or clearinghouse)
    #[serde(rename = "59")]
    FuturesCommissionMerchantCode,
    /// Delivery terminal customer account/code (Usually used for gas delivery to identify whose account the gas is allocated to at the delivery terminal. Often referred to as "HUB" code)
    #[serde(rename = "60")]
    DeliveryTerminalCustomerAccountCode,
    /// Voluntary reporting entity (The entity voluntarily reporting the trade to the regulator. Set PartySubID(523)=Y if true)
    #[serde(rename = "61")]
    VoluntaryReportingEntity,
    /// Reporting obligation jurisdiction (For a trade that falls under multiple jurisdictions this may be used to identify, through PartySubID(523), the reporting jurisdiction to which the party is obligated to report)
    #[serde(rename = "62")]
    ReportingPbligationJurisdiction,
    /// Voluntary reporting jurisdiction (For a trade that falls under multiple jurisdictions this may be used to identify, through PartySubID(523), the regulatory jurisdiction to which the party is submitting a voluntary report)
    #[serde(rename = "63")]
    VoluntaryReportingJurisdiction,
    /// Company Activities
    #[serde(rename = "64")]
    CompanyActivities,
    /// European Economic Area domiciled
    #[serde(rename = "65")]
    EuropeanEconomicAreaDomiciled,
    /// Contract linked to commercial or treasury financing for this counterparty
    #[serde(rename = "66")]
    ContractLinkedToCommercialOrTreasuryFinancingForThisCounterparty,
    /// Contract above clearing threshold for this counterparty
    #[serde(rename = "67")]
    ContractAboveClearingThresholdForThisCounterparty,
    /// Voluntary reporting party (When PartySubID(523)=Y, identifies that the party is reporting voluntarily when VoluntaryRegulatoryReport(1935) = Y)
    #[serde(rename = "68")]
    VoluntaryReportingParty,
    /// End user (When PartySubID(523)=Y the counterparty is neither the swap dealer, major swap participant nor financial entity as defined in the regulations)
    #[serde(rename = "69")]
    EndUser,
    /// Location or jurisdiction
    #[serde(rename = "70")]
    LocationOrJurisdiction,
    /// Derivatives dealer (Elaboration: Indicates whether the party is a derivatives dealer or not (Y/N). The Canadian regulator's defined term for identifying the trade counterparty as "a person or company engaging in or holding himself, herself or itself out as engaging in the business of trading in derivatives in Ontario as principal or agent")
    #[serde(rename = "71")]
    DerivativesDealer,
    /// Domicile (Elaboration: Country and optionally province, state or region of domicile. The party sub-ID value is either a 2-character ISO 3166 country code or a hyphenated combination of the country code and the standard post-office abbreviation of province, state or region if necessary. E.g. "US" for United States or "CA-QC" for Quebec Canada)
    #[serde(rename = "72")]
    Domicile,
    /// Exempt from recognition (Elaboration: Used with party role 21 "Clearing Organization" to indicate exemption (Y/N). Identifies a clearing agency as exempt from oversight in Ontario, i.e. one that 1) only provides limited services and does not present significant risks or 2) is foreign-based, indends to operate in Ontario but is subject to regulatory oversight in another jurisdiction)
    #[serde(rename = "73")]
    ExemptFromRecognition,
    /// Payer
    #[serde(rename = "74")]
    Payer,
    /// Receiver
    #[serde(rename = "75")]
    Receiver,
    /// Systematic Internaliser (SI)
    #[serde(rename = "76")]
    SystematicInternaliser,
    /// Publishing entity indicator
    #[serde(rename = "77")]
    PublishingEntityIndicator,
    /// First name
    #[serde(rename = "78")]
    FirstName,
    /// Surname
    #[serde(rename = "79")]
    Surname,
    /// Date of birth
    #[serde(rename = "80")]
    DateOfBirth,
    /// Order transmitting firm
    #[serde(rename = "81")]
    OrderTransmittingFirm,
    /// Order transmitting firm for buyer
    #[serde(rename = "82")]
    OrderTransmittingFirmForBuyer,
    /// Order transmitter for seller
    #[serde(rename = "83")]
    OrderTransmitterForSeller,
    /// Legal Entity Identifier (ISO 17442) LEI
    #[serde(rename = "84")]
    LegalEntityIdentifierLEI,
    /// Sub-sector classification
    #[serde(rename = "85")]
    SubSectorClassification,
    /// Party side
    #[serde(rename = "86")]
    PartySide,
    /// Legal registration country
    #[serde(rename = "87")]
    LegalRegistrationCountry,
}
