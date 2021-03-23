
use serde::{Serialize, Deserialize};

use crate::entities::{Boolean, data_field, LocalMktDate, MonthYear, TZTimeOnly};

use super::{evnt_grp::EvntGrps, sec_alt_id_grp::SecAltIDGrp, security_xml::SecurityXML, time_unit::TimeUnit};

//regex: ^(=>\s+)?(\d+)\s+(\w+)(\s+@\w+)?\s+([YNC])(\s+.+)?$
//replace: /// $6\n#[serde(rename = "$2")]\npub $3: $5,
#[derive(Serialize, Deserialize, Debug)]
pub struct Instrument {
    /// Common, "human understood" representation of the security. SecurityID (48) value can be specified if no symbol exists (e.g. non-exchange traded Collective Investment Vehicles). Use "[N/A]" for products which do not have a symbol.
    #[serde(rename = "55")]
    pub symbol: Option<String>,
    /// Used in Fixed Income with a value of "WI" to indicate "When Issued" for a security to be reissued under an old CUSIP or ISIN or with a value of "CD" to indicate a EUCP with lump-sum interest rather than discount price.
    #[serde(rename = "65")]
    pub symbol_sfx: Option<String>,
    /// Takes precedence in identifying security to counterparty over SecurityAltID (455) block. Requires SecurityIDSource (22) if specified.
    #[serde(rename = "48")]
    pub security_id: Option<String>,
    /// Required if SecurityID (48) is specified.
    #[serde(rename = "22")]
    pub security_id_source: Option<SecurityIDSource>,
    /// Number of alternate Security Identifiers
    #[serde(flatten)]
    pub sec_alt_id_grp: SecAltIDGrp,
    /// Indicates the type of product the security is associated with (high-level category)
    #[serde(rename = "460")]
    pub product: Option<Product>,
    /// Identifies an entire suite of products for a given market. In Futures this may be "interest rates", "agricultural", "equity indexes", etc
    #[serde(rename = "1227")]
    pub product_complex: Option<String>,
    /// An exchange specific name assigned to a group of related securities which may be concurrently affected by market events and actions.
    #[serde(rename = "1151")]
    pub security_group: Option<String>,
    /// Indicates the type of security using ISO 10962 standard, Classification of Financial Instruments (CFI code) values. It is recommended that CFICode (461) be used instead of SecurityType (167) for non-Fixed Income instruments.
    #[serde(rename = "461")]
    pub cfi_code: Option<String>,
    /// It is recommended that CFICode (461) be used instead of SecurityType (167) for non-Fixed Income instruments. Required for Fixed Income. Refer to Volume 7 - Fixed Income Futures and Options should be specified using the CFICode (461) field instead of SecurityType (167) (Refer to Volume 7 - Recommendations and Guidelines for Futures and Options Markets.)
    #[serde(rename = "167")]
    pub security_type: Option<SecurityType>,
    /// Sub-type qualification/identification of the SecurityType (167) (e.g. for SecurityType="MLEG"). If specified, SecurityType (167) is required.
    #[serde(rename = "762")]
    pub security_sub_type: Option<String>,
    /// Specifies the month and year of maturity. Applicable for standardized derivatives which are typically only referenced b y month and year (e.g. S&P futures). Note MaturityDate (541) (a full date) can also be specified.
    #[serde(rename = "200")]
    pub maturity_month_year: Option<MonthYear>,// TODO: manage time format?
    /// Specifies date of maturity (a full date). Note that standardized derivatives which are typically only referenced by month and year (e.g. S&P futures).may use MaturityMonthYear (200) and/or this field. When using MaturityMonthYear, it is recommended that markets and sell sides report the MaturityDate (541) on all outbound messages as a means of data enrichment.
    #[serde(rename = "541")]
    pub maturity_date: Option<LocalMktDate>,
    /// For NDFs this represents the fixing time of the contract. It is optional to specify the fixing time.
    #[serde(rename = "1079")]
    pub maturity_time: Option<TZTimeOnly>,// TODO: manage time format?
    /// Indicator to determine if Instrument is Settle on Open.
    #[serde(rename = "966")]
    pub settle_on_open_flag: Option<String>,
    /// InstrmtAssignmentMethod
    #[serde(rename = "1049")]
    pub instrmt_assignment_method: Option<InstrmtAssignmentMethod>,
    /// Gives the current state of the instrument
    #[serde(rename = "965")]
    pub security_status: Option<SecurityStatus>,
    /// Date interest is to be paid. Used in identifying Corporate Bond issues.
    #[serde(rename = "224")]
    pub coupon_payment_date: Option<LocalMktDate>,
    /// RestructuringType
    #[serde(rename = "1449")]
    pub restructuring_type: Option<RestructuringType>,
    /// Seniority
    #[serde(rename = "1450")]
    pub seniority: Option<Seniority>,
    /// NotionalPercentageOutstanding
    #[serde(rename = "1451")]
    pub notional_percentage_outstanding: Option<f32>,
    /// OriginalNotionalPercentageOutstanding
    #[serde(rename = "1452")]
    pub original_notional_percentage_outstanding: Option<f32>,
    /// AttachmentPoint
    #[serde(rename = "1457")]
    pub attachment_point: Option<f32>,
    /// DetachmentPoint
    #[serde(rename = "1458")]
    pub detachment_point: Option<f32>,
    /// Date instrument was issued. For Fixed Income IOIs for new issues, specifies the issue date.
    #[serde(rename = "225")]
    pub issue_date: Option<LocalMktDate>,
    /// (Deprecated in FIX.4.4)
    #[serde(rename = "239")]
    pub repo_collateral_security_type: Option<RepoCollateralSecurityType>,
    /// (Deprecated in FIX.4.4)
    #[serde(rename = "226")]
    pub repurchase_term: Option<i32>,
    /// (Deprecated in FIX.4.4)
    #[serde(rename = "227")]
    pub repurchase_rate: Option<f32>,
    /// For Fixed Income: Amortization Factor for deriving Current face from Original face for ABS or MBS securities, note the fraction may be greater than, equal to or less than 1. In TIPS securities this is the Inflation index. Qty * Factor * Price = Gross Trade Amount. For Derivatives: Contract Value Factor by which price must be adjusted to determine the true nominal value of one futures/options contract. (Qty * Price) * Factor = Nominal Value.
    #[serde(rename = "228")]
    pub factor: Option<f64>,
    /// CreditRating
    #[serde(rename = "255")]
    pub credit_rating: Option<String>,
    /// The location at which records of ownership are maintained for this instrument, and at which ownership changes must be recorded. Can be used in conjunction with ISIN to address ISIN uniqueness issues.
    #[serde(rename = "543")]
    pub instr_registry: Option<String>,
    /// ISO Country code of instrument issue (e.g. the country portion typically used in ISIN). Can be used in conjunction with non-ISIN SecurityID (48) (e.g. CUSIP for Municipal Bonds without ISIN) to provide uniqueness.
    #[serde(rename = "470")]
    pub country_of_issue: Option<Country>,
    /// A two-character state or province abbreviation.
    #[serde(rename = "471")]
    pub state_or_province_of_issue: Option<String>,
    /// The three-character IATA code for a locale (e.g. airport code for Municipal Bonds).
    #[serde(rename = "472")]
    pub locale_of_issue: Option<String>,
    /// (Deprecated in FIX.4.4)
    #[serde(rename = "240")]
    pub redemption_date: Option<LocalMktDate>,
    /// Used for derivatives, such as options and covered warrants
    #[serde(rename = "202")]
    pub strike_price: Option<f64>,
    /// Used for derivatives
    #[serde(rename = "947")]
    pub strike_currency: Option<Currency>,
    /// Used for derivatives. Multiplier applied to the strike price for the purpose of calculating the settlement value.
    #[serde(rename = "967")]
    pub strike_multiplier: Option<f64>,
    /// Used for derivatives. The number of shares/units for the financial instrument involved in the option trade.
    #[serde(rename = "968")]
    pub strike_value: Option<f64>,
    /// StrikePriceDeterminationMethod
    #[serde(rename = "1478")]
    pub strike_price_determination_method: Option<StrikePriceDeterminationMethod>,
    /// When specified, PutOrCall(201), StrikePrice(202), and StrikePriceBoundaryPrecision(1480) must also be specified.
    #[serde(rename = "1479")]
    pub strike_price_boundary_method: Option<StrikePriceBoundaryMethod>,
    /// StrikePriceBoundaryPrecision
    #[serde(rename = "1480")]
    pub strike_price_boundary_precision: Option<f32>,
    /// UnderlyingPriceDeterminationMethod
    #[serde(rename = "1481")]
    pub underlying_price_determination_method: Option<UnderlyingPriceDeterminationMethod>,
    /// Used for derivatives, such as options and covered warrants to indicate a versioning of the contract when required due to corporate actions to the underlying. Should not be used to indicate type of option - use the CFICode[461] for this purpose.
    #[serde(rename = "206")]
    pub opt_attribute: Option<String>,
    /// For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g. contracts vs. shares) amount.
    #[serde(rename = "231")]
    pub contract_multiplier: Option<f64>,
    /// ContractMultiplierUnit
    #[serde(rename = "1435")]
    pub contract_multiplier_unit: Option<ContractMultiplierUnit>,
    /// FlowScheduleType
    #[serde(rename = "1439")]
    pub flow_schedule_type: Option<FlowScheduleType>,
    /// Minimum price increment for the instrument. Could also be used to represent tick value.
    #[serde(rename = "969")]
    pub min_price_increment: Option<f64>,
    /// Minimum price increment amount associated with the MinPriceIncrement (969) . For listed derivatives, the value can be calculated by multiplying MinPriceIncrement by ContractValueFactor (231)
    #[serde(rename = "1146")]
    pub min_price_increment_amount: Option<f64>,
    /// Used to indicate the size of the underlying commodity on which the contract is based (e.g., 2500 lbs of lean cattle, 1000 barrels of crude oil, 1000 bushels of corn, etc.)
    #[serde(rename = "996")]
    pub unit_of_measure: Option<UnitOfMeasure>,
    /// UnitOfMeasureQty
    #[serde(rename = "1147")]
    pub unit_of_measure_qty: Option<f64>,
    /// PriceUnitOfMeasure
    #[serde(rename = "1191")]
    pub price_unit_of_measure: Option<UnitOfMeasure>,
    /// PriceUnitOfMeasureQty
    #[serde(rename = "1192")]
    pub price_unit_of_measure_qty: Option<f64>,
    /// Conditionally required if SettlSubMethod(2579) is specified.
    #[serde(rename = "1193")]
    pub settl_method: Option<SettlMethod>,
    /// Type of exercise of a derivatives security
    #[serde(rename = "1194")]
    pub exercise_style: Option<ExerciseStyle>,
    /// OptPayoutType
    #[serde(rename = "1482")]
    pub opt_payout_type: Option<OptPayoutType>,
    /// Conditionally required if OptPayoutType(1482) = 3 (Binary).
    #[serde(rename = "1195")]
    pub opt_payout_amount: Option<f64>,
    /// Method for price quotation
    #[serde(rename = "1196")]
    pub price_quote_method: Option<PriceQuoteMethod>,
    /// For futures, indicates type of valuation method applied
    #[serde(rename = "1197")]
    pub valuation_method: Option<ValuationMethod>,
    /// Indicates whether the instruments are pre-listed only or can also be defined via user request
    #[serde(rename = "1198")]
    pub list_method: Option<ListMethod>,
    /// Used to express the ceiling price of a capped call
    #[serde(rename = "1199")]
    pub cap_price: Option<f64>,
    /// Used to express the floor price of a capped put
    #[serde(rename = "1200")]
    pub floor_price: Option<f64>,
    /// Used to express option right
    #[serde(rename = "201")]
    pub put_or_call: Option<PutOrCall>,
    /// Used to indicate if a security has been defined as flexible according to "non-standard" means. Analog to CFICode Standard/Non-standard indicator
    #[serde(rename = "1244")]
    pub flexible_indicator: Option<Boolean>,
    /// Used to indicate if a product or group of product supports the creation of flexible securities
    #[serde(rename = "1242")]
    pub flex_product_eligibility_indicator: Option<Boolean>,
    /// Used to indicate a time unit for the contract (e.g., days, weeks, months, etc.)
    #[serde(rename = "997")]
    pub time_unit: Option<TimeUnit>,
    /// For Fixed Income.
    #[serde(rename = "223")]
    pub coupon_rate: Option<f32>,
    /// Can be used to identify the security.
    #[serde(rename = "207")]
    pub security_exchange: Option<String>,
    /// Position Limit for the instrument.
    #[serde(rename = "970")]
    pub position_limit: Option<i32>,
    /// Near-term Position Limit for the instrument.
    #[serde(rename = "971")]
    pub nt_position_limit: Option<i32>,
    /// Issuer
    #[serde(rename = "106")]
    pub issuer: Option<String>,
    /// Must be set if EncodedIssuer(349) field is specified and must immediately precede it.
    /// Encoded (non-ASCII characters) representation of the Issuer(106) field in the encoded format specified via the MessageEncoding(347) field.
    #[serde(rename = "348")]
    #[serde(alias = "349")]
    pub encoded_issuer: Option<EncodedIssuer>,
    /// SecurityDesc
    #[serde(rename = "107")]
    pub security_desc: Option<String>,
    /// Must be set if EncodedSecurityDesc(351) field is specified and must immediately precede it.
    /// Encoded (non-ASCII characters) representation of the SecurityDesc(107) field in the encoded format specified via the MessageEncoding(347) field.
    #[serde(rename = "350")]
    #[serde(alias = "351")]
    pub encoded_security_desc: Option<EncodedSecurityDesc>,
    /// Embedded XML document describing the instrument.
    #[serde(flatten)]
    pub security_xml: SecurityXML,
    /// Identifies MBS / ABS pool
    #[serde(rename = "691")]
    pub pool: Option<String>,
    /// Must be present for MBS/TBA
    #[serde(rename = "667")]
    pub contract_settl_month: Option<MonthYear>,
    /// The program under which a commercial paper is issued
    #[serde(rename = "875")]
    pub cp_program: Option<CPProgram>,
    /// The registration type of a commercial paper issuance
    #[serde(rename = "876")]
    pub cp_reg_type: Option<String>,
    /// Number of repeating EventType group entries.
    #[serde(flatten)]
    pub evnt_grps: EvntGrps,
    /// If different from IssueDate
    #[serde(rename = "873")]
    pub dated_date: Option<LocalMktDate>,
    /// If different from IssueDate (225) and DatedDate
    #[serde(rename = "874")]
    pub interest_accrual_date: Option<LocalMktDate>,
    /*<InstrumentParties> N Used to identify the parties related to a specific instrument.
    <ComplexEvents> N 
    /// 
    1739 ObligationType @ObligTyp N 
    #[serde(rename = "1524")]
    pub PriceQuoteCurrency: N,
    /// 
    1716 UnitOfMeasureCurrency @UOMCcy N 
    #[serde(rename = "1687")]
    pub ShortSaleRestriction: N,
    /// 
    1787 RefTickTableID @RefTickTblID N 
    #[serde(rename = "1717")]
    pub PriceUnitOfMeasureCurrency: N,

    Spread table code referred by the security or symbol
    /// Required if AssetSubClass(1939) is specified.
    #[serde(rename = "1938")]
    pub AssetClass: C,
    /// Required if AssetType(1940) is specified.
    #[serde(rename = "1939")]
    pub AssetSubClass: C,
    /// Required if AssetSubType(2735) is specified.
    #[serde(rename = "1940")]
    pub AssetType: C,
    <SecondaryAssetGrp> N 
    /// 
    1942 NthToDefault @NthDflt C Conditionally required when MthToDefault(1943) is specified.
    #[serde(rename = "1941")]
    pub SwapClass: N,
    /// 
    1944 SettledEntityMatrixSource @SettldMtrxSrc N 
    #[serde(rename = "1943")]
    pub MthToDefault: N,
    /// 
    1946 CouponType @CpnTyp N 
    #[serde(rename = "1945")]
    pub SettledEntityMatrixPublicationDate: N,
    /// 
    1948 CouponFrequencyPeriod @CpnPeriod C Conditionally required when CouponFrequencyUnit(1949) is specified.
    #[serde(rename = "1947")]
    pub TotalIssuedAmount: N,
    /// Conditionally required when CouponFrequencyPeriod(1948) is specified.
    #[serde(rename = "1949")]
    pub CouponFrequencyUnit: C,
    /// 
    1951 ConvertibleBondEquityID @CnvrtBondEqtyID N 
    #[serde(rename = "1950")]
    pub CouponDayCount: N,
    /// Conditionally required when ConvertibleBondEquityID(1951) is specified.
    #[serde(rename = "1952")]
    pub ConvertibleBondEquityIDSource: C,
    /// 
    1954 LienSeniority @LienSnrty N 
    #[serde(rename = "1953")]
    pub ContractPriceRefMonth: N,
    /// 
    1956 ReferenceEntityType @RefEntityTyp N 
    #[serde(rename = "1955")]
    pub LoanFacility: N,
    /// 
    1958 IndexAnnexVersion @NdxAnxVer N 
    #[serde(rename = "1957")]
    pub IndexSeries: N,
    /// 
    1960 IndexAnnexSource @NdxAnxSrc N 
    #[serde(rename = "1959")]
    pub IndexAnnexDate: N,
    <DateAdjustment> N 
    <StreamGrp> N 
    <ProvisionGrp> N 
    <AdditionalTermGrp> N 
    <ProtectionTermGrp> N 
    <CashSettlTermGrp> N 
    <PhysicalSettlTermGrp> N 
    <AssetAttributeGrp> N 
    /// 
    1577 SettlRateIndex @SettlNdx N 
    #[serde(rename = "1575")]
    pub SwapSubClass: N,
    /// 
    1581 OptionExpirationDesc @ExpDesc N 
    #[serde(rename = "1580")]
    pub SettlRateIndexLocation: N,
    /// Must be set if EncodedOptionExpirationDesc(1697) field is specified and must immediately precede it.
    #[serde(rename = "1678")]
    pub EncodedOptionExpirationDescLen: C,
    /// Encoded (non-ASCII characters) representation of the OptionExpirationDesc(1581) field in the encoded format specified via the MessageEncoding(347) field.
    #[serde(rename = "1697")]
    pub EncodedOptionExpirationDesc: N,
    /// 
    1866 StrikeIndex @StrkNdx N 
    #[serde(rename = "1698")]
    pub StrikeUnitOfMeasure: N,
    /// 
    2002 ValuationSource @ValSrc N 
    #[serde(rename = "2001")]
    pub StrikeIndexSpread: N,
    /// 
    2141 StrategyType @StrtTyp N 
    #[serde(rename = "2140")]
    pub ValuationReferenceModel: N,
    /// 
    2143 SettlDisruptionProvision @SettlDsrptnProv N 
    #[serde(rename = "2142")]
    pub CommonPricingIndicator: N,
    /// 
    2145 InstrumentRoundingPrecision @RndPrcsn N 
    #[serde(rename = "2144")]
    pub InstrumentRoundingDirection: N,
    <PricingDateTime> N 
    <MarketDisruption> N 
    <OptionExercise> N 
    /// 
    2210 AssetGroup @AssetGrp N 
    #[serde(rename = "2353")]
    pub TradingUnitPeriodMultiplier: N,
    /// 
    2577 StrikePricePrecision @StrkPxPrcsn N 
    #[serde(rename = "2578")]
    pub OrigStrikePrice: N,
    /// 
    2575 BlockTradeEligibilityIndicator @BlckTrdEligInd N 
    #[serde(rename = "2579")]
    pub SettlSubMethod: N,
    /// 
    2576 InstrumentPricePrecision @PxPrcsn N 
    #[serde(rename = "2574")]
    pub LowExercisePriceOptionIndicator: N,
    /// 
    2601 StrikeIndexQuote @StrkNdxQte N 
    #[serde(rename = "2600")]
    pub StrikeIndexCurvePoint: N,
    <ExtraordinaryEventGrp> N 
    /// 
    2603 ExchangeLookAlike @ExchLookAlike N 
    #[serde(rename = "2602")]
    pub ExtraordinaryEventAdjustmentMethod: N,
    /// Used to express in-the-moneyness behavior in general terms for the option without the use of StrikePrice(202) and PutOrCall(201).
    #[serde(rename = "2681")]
    pub InTheMoneyCondition: N,
    /// 
    2714 FinancialInstrumentFullName @FullName N 
    #[serde(rename = "2685")]
    pub ContraryInstructionEligibilityIndicator: N,
    /// Must be set if EncodedFinancialInstrumentFullName(2716) field is specified and must immediately precede it.
    #[serde(rename = "2715")]
    pub EncodedFinancialInstrumentFullNameLen: N,
    /// Encoded (non-ASCII characters) representation of the FinancialInstrumentFullName(2714) field in the encoded format specified via the MessageEncoding(347) field.
    #[serde(rename = "2716")]
    pub EncodedFinancialInstrumentFullName: N,
    /// 
    2737 FinancialInstrumentShortName @ShrtName N 
    #[serde(rename = "2735")]
    pub AssetSubType: N,
    /// 
    2752 DeliveryRouteOrCharter @RteChrtr N 
    #[serde(rename = "2753")]
    pub ReturnTrigger: N,
    /// 
    #[serde(rename = "2879")]
    pub CouponOtherDayCount: N,*/
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SecurityIDSource {
    /// CUSIP
    #[serde(rename = "1")]
    CUSIP,
    /// SEDOL
    #[serde(rename = "2")]
    SEDOL,
    /// QUIK
    #[serde(rename = "3")]
    QUIK,
    /// ISIN
    #[serde(rename = "4")]
    ISIN,
    /// RIC
    #[serde(rename = "5")]
    RIC,
    /// ISO Currency Code
    #[serde(rename = "6")]
    ISOCurrencyCode,
    /// ISO Country Code
    #[serde(rename = "7")]
    ISOCountryCode,
    /// Exchange Symbol
    #[serde(rename = "8")]
    ExchangeSymbol,
    /// Consolidated Tape Association (CTA) Symbol (SIAC CTS/CQS line format)
    #[serde(rename = "9")]
    CTASymbol,
    /// Bloomberg Symbol
    #[serde(rename = "A")]
    BloombergSymbol,
    /// Wertpapier
    #[serde(rename = "B")]
    Wertpapier,
    /// Dutch
    #[serde(rename = "C")]
    Dutch,
    /// Valoren
    #[serde(rename = "D")]
    Valoren,
    /// Sicovam
    #[serde(rename = "E")]
    Sicovam,
    /// Belgian
    #[serde(rename = "F")]
    Belgian,
    /// "Common" (Clearstream and Euroclear)
    #[serde(rename = "G")]
    Common,
    /// Clearing House / Clearing Organization
    #[serde(rename = "H")]
    ClearingHouse,
    /// ISDA/FpML Product Specification
    #[serde(rename = "I")]
    ISDAFpMLProductSpecification,
    /// Option Price Reporting Authority
    #[serde(rename = "J")]
    OptionPriceReportingAuthority,
    /// ISDA/FpML Product URL (URL in SecurityID)
    #[serde(rename = "K")]
    ISDAFpMLProductURL,
    /// Letter of Credit
    #[serde(rename = "L")]
    LetterOfCredit,
    /// Marketplace-assigned Identifier
    #[serde(rename = "M")]
    MarketplaceAssignedIdentifier,
    /// Markit RED entity CLIP
    #[serde(rename = "N")]
    MarkitREDEntityCLIP,
    /// Markit RED pair CLIP
    #[serde(rename = "P")]
    MarkitREDPairCLIP,
    /// CFTC commodity code
    #[serde(rename = "Q")]
    CFTCCommodityCode,
    /// ISDA Commodity Reference Price
    #[serde(rename = "R")]
    ISDACommodityReferencePrice,
    /// Financial Instrument Global Identifier
    #[serde(rename = "S")]
    FinancialInstrumentGlobalIdentifier,
    /// Legal Entity Identifier
    #[serde(rename = "T")]
    LegalEntityIdentifier,
    /// Synthetic
    #[serde(rename = "U")]
    Synthetic,
    /// Fidessa Instrument Mnemonic (FIM)
    #[serde(rename = "V")]
    FIM,
    /// Index name
    #[serde(rename = "W")]
    IndexName,
    /// Uniform Symbol (UMTF Symbol)
    #[serde(rename = "X")]
    UMTFSymbol,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Product {
    /// AGENCY
    #[serde(rename = "1")]
    AGENCY,
    /// COMMODITY
    #[serde(rename = "2")]
    COMMODITY,
    /// CORPORATE
    #[serde(rename = "3")]
    CORPORATE,
    /// CURRENCY
    #[serde(rename = "4")]
    CURRENCY,
    /// EQUITY
    #[serde(rename = "5")]
    EQUITY,
    /// GOVERNMENT
    #[serde(rename = "6")]
    GOVERNMENT,
    /// INDEX
    #[serde(rename = "7")]
    INDEX,
    /// LOAN
    #[serde(rename = "8")]
    LOAN,
    /// MONEYMARKET
    #[serde(rename = "9")]
    MONEYMARKET,
    /// MORTGAGE
    #[serde(rename = "10")]
    MORTGAGE,
    /// MUNICIPAL
    #[serde(rename = "11")]
    MUNICIPAL,
    /// OTHER
    #[serde(rename = "12")]
    OTHER,
    /// FINANCING
    #[serde(rename = "13")]
    FINANCING,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SecurityType {
    /// Future
    #[serde(rename = "FUT")]
    Future,
    /// Option
    #[serde(rename = "OPT")]
    Option,
    /// US Treasury Note (Deprecated Value Use TNOTE)
    #[serde(rename = "UST")]
    UST,
    /// US Treasury Bill (Deprecated Value Use TBILL)
    #[serde(rename = "USTB")]
    USTB,
    /// Euro Supranational Coupons *
    #[serde(rename = "EUSUPRA")]
    EuroSupranationalCoupons,
    /// Federal Agency Coupon
    #[serde(rename = "FAC")]
    FederalAgencyCoupon,
    /// Federal Agency Discount Note
    #[serde(rename = "FADN")]
    FederalAgencyDiscountNote,
    /// Private Export Funding *
    #[serde(rename = "PEF")]
    PrivateExportFunding,
    /// USD Supranational Coupons *
    #[serde(rename = "SUPRA")]
    USDSupranationalCoupons,
    /// Corporate Bond
    #[serde(rename = "CORP")]
    CorporateBond,
    /// Corporate Private Placement
    #[serde(rename = "CPP")]
    CorporatePrivatePlacement,
    /// Convertible Bond
    #[serde(rename = "CB")]
    ConvertibleBond,
    /// Dual Currency
    #[serde(rename = "DUAL")]
    DualCurrency,
    /// Euro Corporate Bond
    #[serde(rename = "EUCORP")]
    EuroCorporateBond,
    /// Indexed Linked
    #[serde(rename = "XLINKD")]
    IndexedLinked,
    /// Structured Notes
    #[serde(rename = "STRUCT")]
    StructuredNotes,
    /// Yankee Corporate Bond
    #[serde(rename = "YANK")]
    YankeeCorporateBond,
    /// Foreign Exchange Contract
    #[serde(rename = "FOR")]
    ForeignExchangeContract,
    /// Common Stock
    #[serde(rename = "CS")]
    CommonStock,
    /// Preferred Stock
    #[serde(rename = "PS")]
    PreferredStock,
    /// Repurchase
    #[serde(rename = "REPO")]
    Repurchase,
    /// Forward
    #[serde(rename = "FORWARD")]
    Forward,
    /// Buy Sellback
    #[serde(rename = "BUYSELL")]
    BuySellback,
    /// Securities Loan
    #[serde(rename = "SECLOAN")]
    SecuritiesLoan,
    /// Securities Pledge
    #[serde(rename = "SECPLEDGE")]
    SecuritiesPledge,
    /// Brady Bond
    #[serde(rename = "BRADY")]
    BradyBond,
    /// Euro Sovereigns *
    #[serde(rename = "EUSOV")]
    EuroSovereigns,
    /// US Treasury Bond
    #[serde(rename = "TBOND")]
    USTreasuryBond,
    /// Interest Strip From Any Bond Or Note
    #[serde(rename = "TINT")]
    InterestStripFromAnyBondOrNote,
    /// Treasury Inflation Protected Securities
    #[serde(rename = "TIPS")]
    TreasuryInflationProtectedSecurities,
    /// Principal Strip Of A Callable Bond Or Note
    #[serde(rename = "TCAL")]
    PrincipalStripOfACallableBondOrNote,
    /// Principal Strip From A Non-Callable Bond Or Note
    #[serde(rename = "TPRN")]
    PrincipalStripFromANonCallableBondOrNote,
    /// US Treasury Note
    #[serde(rename = "TNOTE")]
    USTreasuryNote,
    /// US Treasury Bill
    #[serde(rename = "TBILL")]
    USTreasuryBill,
    /// Term Loan
    #[serde(rename = "TERM")]
    TermLoan,
    /// Revolver Loan
    #[serde(rename = "RVLV")]
    RevolverLoan,
    /// Revolver/Term Loan
    #[serde(rename = "RVLVTRM")]
    RevolverTermLoan,
    /// Bridge Loan
    #[serde(rename = "BRIDGE")]
    BridgeLoan,
    /// Letter Of Credit
    #[serde(rename = "LOFC")]
    LetterOfCredit,
    /// Swing Line Facility
    #[serde(rename = "SWING")]
    SwingLineFacility,
    /// Debtor In Possession
    #[serde(rename = "DINP")]
    DebtorInPossession,
    /// Defaulted
    #[serde(rename = "DEFLTED")]
    Defaulted,
    /// Withdrawn
    #[serde(rename = "WITHDRN")]
    Withdrawn,
    /// Replaced
    #[serde(rename = "REPLACD")]
    Replaced,
    /// Matured
    #[serde(rename = "MATURED")]
    Matured,
    /// Amended & Restated
    #[serde(rename = "AMENDED")]
    AmendedRestated,
    /// Retired
    #[serde(rename = "RETIRED")]
    Retired,
    /// Bankers Acceptance
    #[serde(rename = "BA")]
    BankersAcceptance,
    /// Bank Notes
    #[serde(rename = "BN")]
    BankNotes,
    /// Bill Of Exchanges
    #[serde(rename = "BOX")]
    BillOfExchanges,
    /// Certificate Of Deposit
    #[serde(rename = "CD")]
    CertificateOfDeposit,
    /// Call Loans
    #[serde(rename = "CL")]
    CallLoans,
    /// Commercial Paper
    #[serde(rename = "CP")]
    CommercialPaper,
    /// Deposit Notes
    #[serde(rename = "DN")]
    DepositNotes,
    /// Euro Certificate Of Deposit
    #[serde(rename = "EUCD")]
    EuroCertificateOfDeposit,
    /// Euro Commercial Paper
    #[serde(rename = "EUCP")]
    EuroCommercialPaper,
    /// Liquidity Note
    #[serde(rename = "LQN")]
    LiquidityNote,
    /// Medium Term Notes
    #[serde(rename = "MTN")]
    MediumTermNotes,
    /// Overnight
    #[serde(rename = "ONITE")]
    Overnight,
    /// Promissory Note
    #[serde(rename = "PN")]
    PromissoryNote,
    /// Plazos Fijos
    #[serde(rename = "PZFJ")]
    PlazosFijos,
    /// Short Term Loan Note
    #[serde(rename = "STN")]
    ShortTermLoanNote,
    /// Time Deposit
    #[serde(rename = "TD")]
    TimeDeposit,
    /// Extended Comm Note
    #[serde(rename = "XCN")]
    ExtendedCommNote,
    /// Yankee Certificate Of Deposit
    #[serde(rename = "YCD")]
    YankeeCertificateOfDeposit,
    /// Asset-backed Securities
    #[serde(rename = "ABS")]
    AssetbackedSecurities,
    /// Corp. Mortgage-backed Securities
    #[serde(rename = "CMBS")]
    CorpMortgageBackedSecurities,
    /// Collateralized Mortgage Obligation
    #[serde(rename = "CMO")]
    CollateralizedMortgageObligation,
    /// IOETTE Mortgage
    #[serde(rename = "IET")]
    IOETTEMortgage,
    /// Mortgage-backed Securities
    #[serde(rename = "MBS")]
    MortgageBackedSecurities,
    /// Mortgage Interest Only
    #[serde(rename = "MIO")]
    MortgageInterestOnly,
    /// Mortgage Principal Only
    #[serde(rename = "MPO")]
    MortgagePrincipalOnly,
    /// Mortgage Private Placement
    #[serde(rename = "MPP")]
    MortgagePrivatePlacement,
    /// Miscellaneous Pass-through
    #[serde(rename = "MPT")]
    MiscellaneousPassThrough,
    /// Pfandbriefe *
    #[serde(rename = "PFAND")]
    Pfandbriefe,
    /// To Be Announced
    #[serde(rename = "TBA")]
    ToBeAnnounced,
    /// Other Anticipation Notes (BAN, GAN, etc.)
    #[serde(rename = "AN")]
    OtherAnticipationNotes,
    /// Certificate Of Obligation
    #[serde(rename = "COFO")]
    CertificateOfObligation,
    /// Certificate Of Participation
    #[serde(rename = "COFP")]
    CertificateOfParticipation,
    /// General Obligation Bonds
    #[serde(rename = "GO")]
    GeneralObligationBonds,
    /// Mandatory Tender
    #[serde(rename = "MT")]
    MandatoryTender,
    /// Revenue Anticipation Note
    #[serde(rename = "RAN")]
    RevenueAnticipationNote,
    /// Revenue Bonds
    #[serde(rename = "REV")]
    RevenueBonds,
    /// Special Assessment
    #[serde(rename = "SPCLA")]
    SpecialAssessment,
    /// Special Obligation
    #[serde(rename = "SPCLO")]
    SpecialObligation,
    /// Special Tax
    #[serde(rename = "SPCLT")]
    SpecialTax,
    /// Tax Anticipation Note
    #[serde(rename = "TAN")]
    TaxAnticipationNote,
    /// Tax Allocation
    #[serde(rename = "TAXA")]
    TaxAllocation,
    /// Tax Exempt Commercial Paper
    #[serde(rename = "TECP")]
    TaxExemptCommercialPaper,
    /// Tax Revenue Anticipation Note
    #[serde(rename = "TRAN")]
    TaxRevenueAnticipationNote,
    /// Variable Rate Demand Note
    #[serde(rename = "VRDN")]
    VariableRateDemandNote,
    /// Warrant
    #[serde(rename = "WAR")]
    Warrant,
    /// Mutual Fund
    #[serde(rename = "MF")]
    MutualFund,
    /// Multileg Instrument
    #[serde(rename = "MLEG")]
    MultilegInstrument,
    /// No Security Type
    #[serde(rename = "NONE")]
    NoSecurityType,
    /// Options on Futures
    #[serde(rename = "OOF")]
    OptionsOnFutures,
    /// Options on Physical
    #[serde(rename = "OOP")]
    OptionsOnPhysical,
    /// Cash
    #[serde(rename = "CASH")]
    Cash,
    /// Interest Rate Swap
    #[serde(rename = "IRS")]
    InterestRateSwap,
    /// Bank Depository Note
    #[serde(rename = "BDN")]
    BankDepositoryNote,
    /// Canadian Money Markets
    #[serde(rename = "CAMM")]
    CanadianMoneyMarkets,
    /// Canadian Treasury Notes
    #[serde(rename = "CAN")]
    CanadianTreasuryNotes,
    /// Canadian Treasury Bills
    #[serde(rename = "CTB")]
    CanadianTreasuryBills,
    /// Credit Default Swap
    #[serde(rename = "CDS")]
    CreditDefaultSwap,
    /// Canadian Mortgage Bonds
    #[serde(rename = "CMB")]
    CanadianMortgageBonds,
    /// Euro Corporate Floating Rate Notes
    #[serde(rename = "EUFRN")]
    EuroCorporateFloatingRateNotes,
    /// US Corporate Floating Rate Notes
    #[serde(rename = "FRN")]
    USCorporateFloatingRateNotes,
    /// Canadian Provincial Bonds
    #[serde(rename = "PROV")]
    CanadianProvincialBonds,
    /// Secured Liquidity Note
    #[serde(rename = "SLQN")]
    SecuredLiquidityNote,
    /// Treasury Bill - non US
    #[serde(rename = "TB")]
    TreasuryBillNonUS,
    /// Term Liquidity Note
    #[serde(rename = "TLQN")]
    TermLiquidityNote,
    /// Taxable Municipal CP
    #[serde(rename = "TMCP")]
    TaxableMunicipalCP,
    /// Wildcard entry for use on Security Definition Request
    #[serde(rename = "?")]
    Wildcard,
    /// Options on Combo
    #[serde(rename = "OOC")]
    OptionsOnCombo,
    /// Non-deliverable forward
    #[serde(rename = "FXNDF")]
    NonDeliverableForward,
    /// FX Spot
    #[serde(rename = "FXSPOT")]
    FXSpot,
    /// FX Forward
    #[serde(rename = "FXFWD")]
    FXForward,
    /// FX Swap
    #[serde(rename = "FXSWAP")]
    FXSwap,
    /// Deliver versus pledge
    #[serde(rename = "DVPLDG")]
    DeliverVersusPledge,
    /// Commodity swap
    #[serde(rename = "CMDTYSWAP")]
    CommoditySwap,
    /// Futures option swap
    #[serde(rename = "SWAPTION")]
    FuturesOptionSwap,
    /// Derivative Forward
    #[serde(rename = "FWD")]
    DerivativeForward,
    /// Total return swap
    #[serde(rename = "TRS")]
    TotalReturnSwap,
    /// Cap (In an interest rate cap, the buyer receives payments at the end of each period in which the rate indec exceeds the agreed strike rate)
    #[serde(rename = "CAP")]
    Cap,
    /// Collar (In an interest rate collar, this is a combination of a cap and a floor)
    #[serde(rename = "CLLR")]
    Collar,
    /// Exotic
    #[serde(rename = "EXOTIC")]
    Exotic,
    /// Floor (In an interest rate floor, the buyer receives payments at the end of each period in which the rate index is below the agreed strike rate)
    #[serde(rename = "FLR")]
    Floor,
    /// Forward Rate Agreement
    #[serde(rename = "FRA")]
    ForwardRateAgreement,
    /// Loan/lease
    #[serde(rename = "LOANLEASE")]
    Loanlease,
    /// Spot forward
    #[serde(rename = "SPOTFWD")]
    SpotForward,
    /// Transmission
    #[serde(rename = "XMISSION")]
    Transmission,
    /// General type for a contract based on an established index
    #[serde(rename = "INDEX")]
    GeneralIndex,
    /// Collateral basket
    #[serde(rename = "COLLBSKT")]
    CollateralBasket,
    /// Bond basket
    #[serde(rename = "BDBSKT")]
    BondBasket,
    /// Contract for difference
    #[serde(rename = "CFD")]
    ContractForDifference,
    /// Correlation swap
    #[serde(rename = "CRLTNSWAP")]
    CorrelationSwap,
    /// Dividend swap
    #[serde(rename = "DVDNDSWAP")]
    DividendSwap,
    /// Equity basket
    #[serde(rename = "EQBSKT")]
    EquityBasket,
    /// Equity forward
    #[serde(rename = "EQFWD")]
    EquityForward,
    /// Return swap
    #[serde(rename = "RTRNSWAP")]
    ReturnSwap,
    /// Variance swap
    #[serde(rename = "VARSWAP")]
    VarianceSwap,
    /// Non-deliverable Swap
    #[serde(rename = "FXNDS")]
    NonDeliverableSwap,
    /// Portfolio Swaps
    #[serde(rename = "PRTFLIOSWAP")]
    PortfolioSwaps,
    /// Futures on a Swap
    #[serde(rename = "FUTSWAP")]
    FuturesOnASwap,
    /// Forwards on a Swap
    #[serde(rename = "FWDSWAP")]
    ForwardsOnASwap,
    /// Forward Freight Agreement
    #[serde(rename = "FWDFRTAGMT")]
    ForwardFreightAgreement,
    /// Spread Betting
    #[serde(rename = "SPREADBET")]
    SpreadBetting,
    /// Other
    #[serde(rename = "Other")]
    Other,
    /// Depository Receipts
    #[serde(rename = "DR")]
    DepositoryReceipts,
    /// Exchange traded commodity
    #[serde(rename = "ETC")]
    ExchangeTradedCommodity,
    /// Exchange traded note
    #[serde(rename = "ETN")]
    ExchangeTradedNote,
    /// Securitized derivative
    #[serde(rename = "SECDERIV")]
    SecuritizedDerivative,
    /// Structured finance product
    #[serde(rename = "SFP")]
    StructuredFinanceProduct,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum InstrmtAssignmentMethod {
    /// Random
    #[serde(rename = "R")]
    Random,
    /// ProRata
    #[serde(rename = "P")]
    ProRata,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SecurityStatus {
    /// Active
    #[serde(rename = "1")]
    Active,
    /// Inactive
    #[serde(rename = "2")]
    Inactive,
    /// Active, closing orders only
    #[serde(rename = "3")]
    ActiveClosingOrdersOnly,
    /// Expired
    #[serde(rename = "4")]
    Expired,
    /// Delisted
    #[serde(rename = "5")]
    Delisted,
    /// Knocked-out
    #[serde(rename = "6")]
    KnockedOut,
    /// Knock-out revoked
    #[serde(rename = "7")]
    KnockOutRevoked,
    /// Pending Expiry
    #[serde(rename = "8")]
    PendingExpiry,
    /// Suspended
    #[serde(rename = "9")]
    Suspended,
    /// Published
    #[serde(rename = "10")]
    Published,
    /// Pending Deletion
    #[serde(rename = "11")]
    PendingDeletion,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum RestructuringType {
    /// Full Restructuring
    #[serde(rename = "FR")]
    FullRestructuring,
    /// Modified Restructuring
    #[serde(rename = "MR")]
    ModifiedRestructuring,
    /// Modified Mod Restructuring
    #[serde(rename = "MM")]
    ModifiedModRestructuring,
    /// No Restructuring specified
    #[serde(rename = "XR")]
    NoRestructuringSpecified,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Seniority {
    /// Senior Secured
    #[serde(rename = "SD")]
    SeniorSecured,
    /// Senior
    #[serde(rename = "SR")]
    Senior,
    /// Subordinated
    #[serde(rename = "SB")]
    Subordinated,
    /// Junior
    #[serde(rename = "JR")]
    Junior,
    /// Mezzanine
    #[serde(rename = "MZ")]
    Mezzanine,
    /// Senior Non-Preferred}
    #[serde(rename = "SN")]
    SeniorNonPreferred,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum RepoCollateralSecurityType {
    /// Future
    FUT,
    /// Option
    OPT,
    /// US Treasury Note (Deprecated Value Use TNOTE)
    UST,
    /// US Treasury Bill (Deprecated Value Use TBILL)
    USTB,
    /// Euro Supranational Coupons *
    EUSUPRA,
    /// Federal Agency Coupon
    FAC,
    /// Federal Agency Discount Note
    FADN,
    /// Private Export Funding *
    PEF,
    /// USD Supranational Coupons *
    SUPRA,
    /// Corporate Bond
    CORP,
    /// Corporate Private Placement
    CPP,
    /// Convertible Bond
    CB,
    /// Dual Currency
    DUAL,
    /// Euro Corporate Bond
    EUCORP,
    /// Indexed Linked
    XLINKD,
    /// Structured Notes
    STRUCT,
    /// Yankee Corporate Bond
    YANK,
    /// Foreign Exchange Contract
    FOR,
    /// Common Stock
    CS,
    /// Preferred Stock
    PS,
    /// Repurchase
    REPO,
    /// Forward
    FORWARD,
    /// Buy Sellback
    BUYSELL,
    /// Securities Loan
    SECLOAN,
    /// Securities Pledge
    SECPLEDGE,
    /// Brady Bond
    BRADY,
    /// Euro Sovereigns *
    EUSOV,
    /// US Treasury Bond
    TBOND,
    /// Interest Strip From Any Bond Or Note
    TINT,
    /// Treasury Inflation Protected Securities
    TIPS,
    /// Principal Strip Of A Callable Bond Or Note
    TCAL,
    /// Principal Strip From A Non-Callable Bond Or Note
    TPRN,
    /// US Treasury Note
    TNOTE,
    /// US Treasury Bill
    TBILL,
    /// Term Loan
    TERM,
    /// Revolver Loan
    RVLV,
    /// Revolver/Term Loan
    RVLVTRM,
    /// Bridge Loan
    BRIDGE,
    /// Letter Of Credit
    LOFC,
    /// Swing Line Facility
    SWING,
    /// Debtor In Possession
    DINP,
    /// Defaulted
    DEFLTED,
    /// Withdrawn
    WITHDRN,
    /// Replaced
    REPLACD,
    /// Matured
    MATURED,
    /// Amended & Restated
    AMENDED,
    /// Retired
    RETIRED,
    /// Bankers Acceptance
    BA,
    /// Bank Notes
    BN,
    /// Bill Of Exchanges
    BOX,
    /// Certificate Of Deposit
    CD,
    /// Call Loans
    CL,
    /// Commercial Paper
    CP,
    /// Deposit Notes
    DN,
    /// Euro Certificate Of Deposit
    EUCD,
    /// Euro Commercial Paper
    EUCP,
    /// Liquidity Note
    LQN,
    /// Medium Term Notes
    MTN,
    /// Overnight
    ONITE,
    /// Promissory Note
    PN,
    /// Plazos Fijos
    PZFJ,
    /// Short Term Loan Note
    STN,
    /// Time Deposit
    TD,
    /// Extended Comm Note
    XCN,
    /// Yankee Certificate Of Deposit
    YCD,
    /// Asset-backed Securities
    ABS,
    /// Corp. Mortgage-backed Securities
    CMBS,
    /// Collateralized Mortgage Obligation
    CMO,
    /// IOETTE Mortgage
    IET,
    /// Mortgage-backed Securities
    MBS,
    /// Mortgage Interest Only
    MIO,
    /// Mortgage Principal Only
    MPO,
    /// Mortgage Private Placement
    MPP,
    /// Miscellaneous Pass-through
    MPT,
    /// Pfandbriefe *
    PFAND,
    /// To Be Announced
    TBA,
    /// Other Anticipation Notes (BAN, GAN, etc.)
    AN,
    /// Certificate Of Obligation
    COFO,
    /// Certificate Of Participation
    COFP,
    /// General Obligation Bonds
    GO,
    /// Mandatory Tender
    MT,
    /// Revenue Anticipation Note
    RAN,
    /// Revenue Bonds
    REV,
    /// Special Assessment
    SPCLA,
    /// Special Obligation
    SPCLO,
    /// Special Tax
    SPCLT,
    /// Tax Anticipation Note
    TAN,
    /// Tax Allocation
    TAXA,
    /// Tax Exempt Commercial Paper
    TECP,
    /// Tax Revenue Anticipation Note
    TRAN,
    /// Variable Rate Demand Note
    VRDN,
    /// Warrant
    WAR,
    /// Mutual Fund
    MF,
    /// Multileg Instrument
    MLEG,
    /// No Security Type
    NONE,
    /// Options on Futures
    OOF,
    /// Options on Physical
    OOP,
    /// Cash
    CASH,
    /// Interest Rate Swap
    IRS,
    /// Bank Depository Note
    BDN,
    /// Canadian Money Markets
    CAMM,
    /// Canadian Treasury Notes
    CAN,
    /// Canadian Treasury Bills
    CTB,
    /// Credit Default Swap
    CDS,
    /// Canadian Mortgage Bonds
    CMB,
    /// Euro Corporate Floating Rate Notes
    EUFRN,
    /// US Corporate Floating Rate Notes
    FRN,
    /// Canadian Provincial Bonds
    PROV,
    /// Secured Liquidity Note
    SLQN,
    /// Treasury Bill - non US
    TB,
    /// Term Liquidity Note
    TLQN,
    /// Taxable Municipal CP
    TMCP,
    /// Wildcard entry for use on Security Definition Request
    #[serde(rename = "?")]
    Wildcard,
    /// Options on Combo
    OOC,
    /// Non-deliverable forward
    FXNDF,
    /// FX Spot
    FXSPOT,
    /// FX Forward
    FXFWD,
    /// FX Swap
    FXSWAP,
    /// Deliver versus pledge
    DVPLDG,
    /// Commodity swap
    CMDTYSWAP,
    /// Futures option swap
    SWAPTION,
    /// Derivative Forward
    FWD,
    /// Total return swap
    TRS,
    /// Cap (In an interest rate cap, the buyer receives payments at the end of each period in which the rate indec exceeds the agreed strike rate)
    CAP,
    /// Collar (In an interest rate collar, this is a combination of a cap and a floor)
    CLLR,
    /// Exotic
    EXOTIC,
    /// Floor (In an interest rate floor, the buyer receives payments at the end of each period in which the rate index is below the agreed strike rate)
    FLR,
    /// Forward Rate Agreement
    FRA,
    /// Loan/lease
    LOANLEASE,
    /// Spot forward
    SPOTFWD,
    /// Transmission
    XMISSION,
    /// General type for a contract based on an established index
    INDEX,
    /// Collateral basket
    COLLBSKT,
    /// Bond basket
    BDBSKT,
    /// Contract for difference
    CFD,
    /// Correlation swap
    CRLTNSWAP,
    /// Dividend swap
    DVDNDSWAP,
    /// Equity basket
    EQBSKT,
    /// Equity forward
    EQFWD,
    /// Return swap
    RTRNSWAP,
    /// Variance swap
    VARSWAP,
    /// Non-deliverable Swap
    FXNDS,
    /// Portfolio Swaps
    PRTFLIOSWAP,
    /// Futures on a Swap
    FUTSWAP,
    /// Forwards on a Swap
    FWDSWAP,
    /// Forward Freight Agreement
    FWDFRTAGMT,
    /// Spread Betting
    SPREADBET,
    /// Other
    Other,
    /// Depository Receipts
    DR,
    /// Exchange traded commodity
    ETC,
    /// Exchange traded note
    ETN,
    /// Securitized derivative
    SECDERIV,
    /// Structured finance product
    SFP,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Country {
    /// AFGHANISTAN
    AF,
    /// ALBANIA
    AL,
    /// ALGERIA
    DZ,
    /// AMERICAN SAMOA
    AS,
    /// ANDORRA
    AD,
    /// ANGOLA
    AO,
    /// ANGUILLA
    AI,
    /// ANTARCTICA
    AQ,
    /// ANTIGUA AND BARBUDA
    AG,
    /// ARGENTINA
    AR,
    /// ARMENIA
    AM,
    /// ARUBA
    AW,
    /// AUSTRALIA
    AU,
    /// AUSTRIA
    AT,
    /// AZERBAIJAN
    AZ,
    /// BAHAMAS
    BS,
    /// BAHRAIN
    BH,
    /// BANGLADESH
    BD,
    /// BARBADOS
    BB,
    /// BELARUS
    BY,
    /// BELGIUM
    BE,
    /// BELIZE
    BZ,
    /// BENIN
    BJ,
    /// BERMUDA
    BM,
    /// BHUTAN
    BT,
    /// BOLIVIA
    BO,
    /// BOSNIA AND HERZEGOVINA
    BA,
    /// BOTSWANA
    BW,
    /// BOUVET ISLAND
    BV,
    /// BRAZIL
    BR,
    /// BRITISH INDIAN OCEAN TERRITORY
    IO,
    /// BRUNEI DARUSSALAM
    BN,
    /// BULGARIA
    BG,
    /// BURKINA FASO
    BF,
    /// BURUNDI
    BI,
    /// CAMBODIA
    KH,
    /// CAMEROON
    CM,
    /// CANADA
    CA,
    /// CAPE VERDE
    CV,
    /// CAYMAN ISLANDS
    KY,
    /// CENTRAL AFRICAN REPUBLIC
    CF,
    /// CHAD
    TD,
    /// CHILE
    CL,
    /// CHINA
    CN,
    /// CHRISTMAS ISLAND
    CX,
    /// COCOS (KEELING) ISLANDS
    CC,
    /// COLOMBIA
    CO,
    /// COMOROS
    KM,
    /// CONGO
    CG,
    /// CONGO, THE DEMOCRATIC REPUBLIC OF THE
    CD,
    /// COOK ISLANDS
    CK,
    /// COSTA RICA
    CR,
    /// COTE D'IVOIRE
    CI,
    /// CROATIA
    HR,
    /// CUBA
    CU,
    /// CYPRUS
    CY,
    /// CZECH REPUBLIC
    CZ,
    /// DENMARK
    DK,
    /// DJIBOUTI
    DJ,
    /// DOMINICA
    DM,
    /// DOMINICAN REPUBLIC
    DO,
    /// ECUADOR
    EC,
    /// EGYPT
    EG,
    /// EL SALVADOR
    SV,
    /// EQUATORIAL GUINEA
    GQ,
    /// ERITREA
    ER,
    /// ESTONIA
    EE,
    /// ETHIOPIA
    ET,
    /// FALKLAND ISLANDS (MALVINAS)
    FK,
    /// FAROE ISLANDS
    FO,
    /// FIJI
    FJ,
    /// FINLAND
    FI,
    /// FRANCE
    FR,
    /// FRENCH GUIANA
    GF,
    /// FRENCH POLYNESIA
    PF,
    /// FRENCH SOUTHERN TERRITORIES
    TF,
    /// GABON
    GA,
    /// GAMBIA
    GM,
    /// GEORGIA
    GE,
    /// GERMANY
    DE,
    /// GHANA
    GH,
    /// GIBRALTAR
    GI,
    /// GREECE
    GR,
    /// GREENLAND
    GL,
    /// GRENADA
    GD,
    /// GUADELOUPE
    GP,
    /// GUAM
    GU,
    /// GUATEMALA
    GT,
    /// GUINEA
    GN,
    /// GUINEA-BISSAU
    GW,
    /// GUYANA
    GY,
    /// HAITI
    HT,
    /// HEARD ISLAND AND MCDONALD ISLANDS
    HM,
    /// HOLY SEE (VATICAN CITY STATE)
    VA,
    /// HONDURAS
    HN,
    /// HONG KONG
    HK,
    /// HUNGARY
    HU,
    /// ICELAND
    IS,
    /// INDIA
    IN,
    /// INDONESIA
    ID,
    /// IRAN, ISLAMIC REPUBLIC OF
    IR,
    /// IRAQ
    IQ,
    /// IRELAND
    IE,
    /// ISRAEL
    IL,
    /// ITALY
    IT,
    /// JAMAICA
    JM,
    /// JAPAN
    JP,
    /// JORDAN
    JO,
    /// KAZAKHSTAN
    KZ,
    /// KENYA
    KE,
    /// KIRIBATI
    KI,
    /// KOREA, DEMOCRATIC PEOPLE'S REPUBLIC OF
    KP,
    /// KOREA, REPUBLIC OF
    KR,
    /// KUWAIT
    KW,
    /// KYRGYZSTAN
    KG,
    /// LAO PEOPLE'S DEMOCRATIC REPUBLIC
    LA,
    /// LATVIA
    LV,
    /// LEBANON
    LB,
    /// LESOTHO
    LS,
    /// LIBERIA
    LR,
    /// LIBYAN ARAB JAMAHIRIYA
    LY,
    /// LIECHTENSTEIN
    LI,
    /// LITHUANIA
    LT,
    /// LUXEMBOURG
    LU,
    /// MACAO
    MO,
    /// MACEDONIA, THE FORMER YUGOSLAV REPUBLIC OF
    MK,
    /// MADAGASCAR
    MG,
    /// MALAWI
    MW,
    /// MALAYSIA
    MY,
    /// MALDIVES
    MV,
    /// MALI
    ML,
    /// MALTA
    MT,
    /// MARSHALL ISLANDS
    MH,
    /// MARTINIQUE
    MQ,
    /// MAURITANIA
    MR,
    /// MAURITIUS
    MU,
    /// MAYOTTE
    YT,
    /// MEXICO
    MX,
    /// MICRONESIA, FEDERATED STATES OF
    FM,
    /// MOLDOVA, REPUBLIC OF
    MD,
    /// MONACO
    MC,
    /// MONGOLIA
    MN,
    /// MONTSERRAT
    MS,
    /// MOROCCO
    MA,
    /// MOZAMBIQUE
    MZ,
    /// MYANMAR
    MM,
    /// NAMIBIA
    NA,
    /// NAURU
    NR,
    /// NEPAL
    NP,
    /// NETHERLANDS
    NL,
    /// NETHERLANDS ANTILLES
    AN,
    /// NEW CALEDONIA
    NC,
    /// NEW ZEALAND
    NZ,
    /// NICARAGUA
    NI,
    /// NIGER
    NE,
    /// NIGERIA
    NG,
    /// NIUE
    NU,
    /// NORFOLK ISLAND
    NF,
    /// NORTHERN MARIANA ISLANDS
    MP,
    /// NORWAY
    NO,
    /// OMAN
    OM,
    /// PAKISTAN
    PK,
    /// PALAU
    PW,
    /// PALESTINIAN TERRITORY, OCCUPIED
    PS,
    /// PANAMA
    PA,
    /// PAPUA NEW GUINEA
    PG,
    /// PARAGUAY
    PY,
    /// PERU
    PE,
    /// PHILIPPINES
    PH,
    /// PITCAIRN
    PN,
    /// POLAND
    PL,
    /// PORTUGAL
    PT,
    /// PUERTO RICO
    PR,
    /// QATAR
    QA,
    /// REUNION
    RE,
    /// ROMANIA
    RO,
    /// RUSSIAN FEDERATION
    RU,
    /// RWANDA
    RW,
    /// SAINT HELENA
    SH,
    /// SAINT KITTS AND NEVIS
    KN,
    /// SAINT LUCIA
    LC,
    /// SAINT PIERRE AND MIQUELON
    PM,
    /// SAINT VINCENT AND THE GRENADINES
    VC,
    /// SAMOA
    WS,
    /// SAN MARINO
    SM,
    /// SAO TOME AND PRINCIPE
    ST,
    /// SAUDI ARABIA
    SA,
    /// SENEGAL
    SN,
    /// SEYCHELLES
    SC,
    /// SIERRA LEONE
    SL,
    /// SINGAPORE
    SG,
    /// SLOVAKIA
    SK,
    /// SLOVENIA
    SI,
    /// SOLOMON ISLANDS
    SB,
    /// SOMALIA
    SO,
    /// SOUTH AFRICA
    ZA,
    /// SOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDS
    GS,
    /// SPAIN
    ES,
    /// SRI LANKA
    LK,
    /// SUDAN
    SD,
    /// SURINAME
    SR,
    /// SVALBARD AND JAN MAYEN
    SJ,
    /// SWAZILAND
    SZ,
    /// SWEDEN
    SE,
    /// SWITZERLAND
    CH,
    /// SYRIAN ARAB REPUBLIC
    SY,
    /// TAIWAN, PROVINCE OF CHINA
    TW,
    /// TAJIKISTAN
    TJ,
    /// TANZANIA, UNITED REPUBLIC OF
    TZ,
    /// THAILAND
    TH,
    /// TIMOR-LESTE
    TL,
    /// TOGO
    TG,
    /// TOKELAU
    TK,
    /// TONGA
    TO,
    /// TRINIDAD AND TOBAGO
    TT,
    /// TUNISIA
    TN,
    /// TURKEY
    TR,
    /// TURKMENISTAN
    TM,
    /// TURKS AND CAICOS ISLANDS
    TC,
    /// TUVALU
    TV,
    /// UGANDA
    UG,
    /// UKRAINE
    UA,
    /// UNITED ARAB EMIRATES
    AE,
    /// UNITED KINGDOM
    GB,
    /// UNITED STATES
    US,
    /// UNITED STATES MINOR OUTLYING ISLANDS
    UM,
    /// URUGUAY
    UY,
    /// UZBEKISTAN
    UZ,
    /// VANUATU
    VU,
    /// VENEZUELA
    VE,
    /// VIET NAM
    VN,
    /// VIRGIN ISLANDS, BRITISH
    VG,
    /// VIRGIN ISLANDS, U.S.
    VI,
    /// WALLIS AND FUTUNA
    WF,
    /// WESTERN SAHARA
    EH,
    /// YEMEN
    YE,
    /// YUGOSLAVIA
    YU,
    /// ZAMBIA
    ZM,
    /// ZIMBABWE
    ZW,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Currency {
    /// Afghani
    AFA,
    /// Algerian Dinar
    DZD,
    /// Andorran Peseta
    ADP,
    /// Argentine Peso
    ARS,
    /// Armenian Dram
    AMD,
    /// Aruban Guilder
    AWG,
    /// Australian Dollar
    AUD,
    /// Azerbaijanian Manat
    AZM,
    /// Bahamian Dollar
    BSD,
    /// Bahraini Dinar
    BHD,
    /// Baht
    THB,
    /// Balboa
    PAB,
    /// Barbados Dollar
    BBD,
    /// Belarussian Ruble
    BYB,
    /// Belgian Franc
    BEF,
    /// Belize Dollar
    BZD,
    /// Bermudian Dollar
    BMD,
    /// Bolivar
    VEB,
    /// Boliviano
    BOB,
    /// Brazilian Real
    BRL,
    /// Brunei Dollar
    BND,
    /// Burundi Franc
    BIF,
    /// CFA Franc BCEAO+
    XOF,
    /// CFA Franc BEAC#
    XAF,
    /// CFP Franc
    XPF,
    /// Canadian Dollar
    CAD,
    /// Cape Verde Escudo
    CVE,
    /// Cayman Islands Dollar
    KYD,
    /// Cedi
    GHC,
    /// Chilean Peso
    CLP,
    /// Colombian Peso
    COP,
    /// Comoro Franc
    KMF,
    /// Convertible Marks
    BAM,
    /// Cordoba Oro
    NIO,
    /// Costa Rican Colon
    CRC,
    /// Cuban Peso
    CUP,
    /// Cyprus Pound
    CYP,
    /// Czech Koruna
    CZK,
    /// Dalasi
    GMD,
    /// Danish Krone
    DKK,
    /// Denar
    MKD,
    /// Deutsche Mark
    DEM,
    /// Djibouti Franc
    DJF,
    /// Dobra
    STD,
    /// Dominican Peso
    DOP,
    /// Dong
    VND,
    /// Drachma
    GRD,
    /// East Caribbean Dollar
    XCD,
    /// Egyptian Pound
    EGP,
    /// El Salvador Colon
    SVC,
    /// Ethiopian Birr
    ETB,
    /// Euro
    EUR,
    /// Falkland Islands Pound
    FKP,
    /// Fiji Dollar
    FJD,
    /// Forint
    HUF,
    /// Franc Congolais
    CDF,
    /// French Franc
    FRF,
    /// Gibraltar Pound
    GIP,
    /// Gourde
    HTG,
    /// Guarani
    PYG,
    /// Guinea Franc
    GNF,
    /// Guinea-Bissau Peso
    GWP,
    /// Guyana Dollar
    GYD,
    /// Hong Kong Dollar
    HKD,
    /// Hryvnia
    UAH,
    /// Iceland Krona
    ISK,
    /// Indian Rupee
    INR,
    /// Iranian Rial
    IRR,
    /// Iraqi Dinar
    IQD,
    /// Irish Pound
    IEP,
    /// Italian Lira
    ITL,
    /// Jamaican Dollar
    JMD,
    /// Jordanian Dinar
    JOD,
    /// Kenyan Shilling
    KES,
    /// Kina
    PGK,
    /// Kip
    LAK,
    /// Kroon
    EEK,
    /// Kuna
    HRK,
    /// Kuwaiti Dinar
    KWD,
    /// Kwacha
    MWK,
    /// Kwacha
    ZMK,
    /// Kwanza Reajustado
    AOR,
    /// Kyat
    MMK,
    /// Lari
    GEL,
    /// Latvian Lats
    LVL,
    /// Lebanese Pound
    LBP,
    /// Lek
    ALL,
    /// Lempira
    HNL,
    /// Leone
    SLL,
    /// Leu
    ROL,
    /// Lev
    BGL,
    /// Liberian Dollar
    LRD,
    /// Libyan Dinar
    LYD,
    /// Lilangeni
    SZL,
    /// Lithuanian Litas
    LTL,
    /// Loti
    LSL,
    /// Luxembourg Franc
    LUF,
    /// Malagasy Franc
    MGF,
    /// Malaysian Ringgit
    MYR,
    /// Maltese Lira
    MTL,
    /// Manat
    TMM,
    /// Markka
    FIM,
    /// Mauritius Rupee
    MUR,
    /// Metical
    MZM,
    /// Mexican Peso
    MXN,
    /// Mexican Unidad de Inversion (UDI)
    MXV,
    /// Moldovan Leu
    MDL,
    /// Moroccan Dirham
    MAD,
    /// Mvdol
    BOV,
    /// Naira
    NGN,
    /// Nakfa
    ERN,
    /// Namibia Dollar
    NAD,
    /// Nepalese Rupee
    NPR,
    /// Netherlands Antillian Guilder
    ANG,
    /// Netherlands Guilder
    NLG,
    /// New Dinar
    YUM,
    /// New Israeli Sheqel
    ILS,
    /// New Kwanza
    AON,
    /// New Taiwan Dollar
    TWD,
    /// New Zaire
    ZRN,
    /// New Zealand Dollar
    NZD,
    /// Next day
    USN,
    /// Ngultrum
    BTN,
    /// North Korean Won
    KPW,
    /// Norwegian Krone
    NOK,
    /// Nuevo Sol
    PEN,
    /// Ouguiya
    MRO,
    /// Pa'anga
    TOP,
    /// Pakistan Rupee
    PKR,
    /// Pataca
    MOP,
    /// Peso Uruguayo
    UYU,
    /// Philippine Peso
    PHP,
    /// Portuguese Escudo
    PTE,
    /// Pound Sterling
    GBP,
    /// Pula
    BWP,
    /// Qatari Rial
    QAR,
    /// Quetzal
    GTQ,
    /// Rand
    ZAR,
    /// Rial Omani
    OMR,
    /// Riel
    KHR,
    /// Rufiyaa
    MVR,
    /// Rupiah
    IDR,
    /// Russian Ruble
    RUB,
    /// Russian Ruble
    RUR,
    /// Rwanda Franc
    RWF,
    /// SDR
    XDR,
    /// Same day
    USS,
    /// Saudi Riyal
    SAR,
    /// Schilling
    ATS,
    /// Seychelles Rupee
    SCR,
    /// Singapore Dollar
    SGD,
    /// Slovak Koruna
    SKK,
    /// Solomon Islands Dollar
    SBD,
    /// Som
    KGS,
    /// Somali Shilling
    SOS,
    /// Spanish Peseta
    ESP,
    /// Sri Lanka Rupee
    LKR,
    /// St Helena Pound
    SHP,
    /// Sucre
    ECS,
    /// Sudanese Dinar
    SDD,
    /// Surinam Guilder
    SRG,
    /// Swedish Krona
    SEK,
    /// Swiss Franc
    CHF,
    /// Syrian Pound
    SYP,
    /// Tajik Ruble
    TJR,
    /// Taka
    BDT,
    /// Tala
    WST,
    /// Tanzanian Shilling
    TZS,
    /// Tenge
    KZT,
    /// Timor Escudo
    TPE,
    /// Tolar
    SIT,
    /// Trinidad and Tobago Dollar
    TTD,
    /// Tugrik
    MNT,
    /// Tunisian Dinar
    TND,
    /// Turkish Lira
    TRL,
    /// UAE Dirham
    AED,
    /// US Dollar
    USD,
    /// Uganda Shilling
    UGX,
    /// Unidad de Valor Constante (UVC)
    ECV,
    /// Unidades de fomento
    CLF,
    /// Uzbekistan Sum
    UZS,
    /// Vatu
    VUV,
    /// Won
    KRW,
    /// Yemeni Rial
    YER,
    /// Yen
    JPY,
    /// Yuan Renminbi
    CNY,
    /// Zimbabwe Dollar
    ZWD,
    /// Zloty
    PLN,
    /// financial Rand
    ZAL,
    /// Afghani
    #[serde(rename = "004")]
    N004,
    /// Algerian Dinar
    #[serde(rename = "01")]
    N01,
    /// Andorran Peseta
    #[serde(rename = "020")]
    N020,
    /// Argentine Peso
    #[serde(rename = "032")]
    N032,
    /// Armenian Dram
    #[serde(rename = "051")]
    N051,
    /// Aruban Guilder
    #[serde(rename = "533")]
    N533,
    /// Australian Dollar
    #[serde(rename = "036")]
    N036,
    /// Azerbaijanian Manat
    #[serde(rename = "031")]
    N031,
    /// Bahamian Dollar
    #[serde(rename = "044")]
    N044,
    /// Bahraini Dinar
    #[serde(rename = "048")]
    N048,
    /// Baht
    #[serde(rename = "764")]
    N764,
    /// Balboa
    #[serde(rename = "590")]
    N590,
    /// Barbados Dollar
    #[serde(rename = "052")]
    N052,
    /// Belarussian Ruble
    #[serde(rename = "112")]
    N112,
    /// Belgian Franc
    #[serde(rename = "056")]
    N056,
    /// Belize Dollar
    #[serde(rename = "084")]
    N084,
    /// Bermudian Dollar
    #[serde(rename = "060")]
    N060,
    /// Bolivar
    #[serde(rename = "862")]
    N862,
    /// Boliviano
    #[serde(rename = "068")]
    N068,
    /// Brazilian Real
    #[serde(rename = "986")]
    N986,
    /// Brunei Dollar
    #[serde(rename = "096")]
    N096,
    /// Burundi Franc
    #[serde(rename = "108")]
    N108,
    /// CFA Franc BCEAO+
    #[serde(rename = "952")]
    N952,
    /// CFA Franc BEAC#
    #[serde(rename = "950")]
    N950,
    /// CFP Franc
    #[serde(rename = "953")]
    N953,
    /// Canadian Dollar
    #[serde(rename = "124")]
    N124,
    /// Cape Verde Escudo
    #[serde(rename = "132")]
    N132,
    /// Cayman Islands Dollar
    #[serde(rename = "136")]
    N136,
    /// Cedi
    #[serde(rename = "288")]
    N288,
    /// Chilean Peso
    #[serde(rename = "152")]
    N152,
    /// Colombian Peso
    #[serde(rename = "170")]
    N170,
    /// Comoro Franc
    #[serde(rename = "174")]
    N174,
    /// Convertible Marks
    #[serde(rename = "977")]
    N977,
    /// Cordoba Oro
    #[serde(rename = "558")]
    N558,
    /// Costa Rican Colon
    #[serde(rename = "188")]
    N188,
    /// Cuban Peso
    #[serde(rename = "192")]
    N192,
    /// Cyprus Pound
    #[serde(rename = "196")]
    N196,
    /// Czech Koruna
    #[serde(rename = "203")]
    N203,
    /// Dalasi
    #[serde(rename = "270")]
    N270,
    /// Danish Krone
    #[serde(rename = "208")]
    N208,
    /// Denar
    #[serde(rename = "807")]
    N807,
    /// Deutsche Mark
    #[serde(rename = "280")]
    N280,
    /// Djibouti Franc
    #[serde(rename = "262")]
    N262,
    /// Dobra
    #[serde(rename = "678")]
    N678,
    /// Dominican Peso
    #[serde(rename = "214")]
    N214,
    /// Dong
    #[serde(rename = "704")]
    N704,
    /// Drachma
    #[serde(rename = "300")]
    N300,
    /// East Caribbean Dollar
    #[serde(rename = "951")]
    N951,
    /// Egyptian Pound
    #[serde(rename = "818")]
    N818,
    /// El Salvador Colon
    #[serde(rename = "222")]
    N222,
    /// Ethiopian Birr
    #[serde(rename = "230")]
    N230,
    /// Euro
    #[serde(rename = "978")]
    N978,
    /// Falkland Islands Pound
    #[serde(rename = "238")]
    N238,
    /// Fiji Dollar
    #[serde(rename = "242")]
    N242,
    /// Forint
    #[serde(rename = "348")]
    N348,
    /// Franc Congolais
    #[serde(rename = "976")]
    N976,
    /// French Franc
    #[serde(rename = "250")]
    N250,
    /// Gibraltar Pound
    #[serde(rename = "292")]
    N292,
    /// Gourde
    #[serde(rename = "332")]
    N332,
    /// Guarani
    #[serde(rename = "600")]
    N600,
    /// Guinea Franc
    #[serde(rename = "324")]
    N324,
    /// Guinea-Bissau Peso
    #[serde(rename = "624")]
    N624,
    /// Guyana Dollar
    #[serde(rename = "328")]
    N328,
    /// Hong Kong Dollar
    #[serde(rename = "344")]
    N344,
    /// Hryvnia
    #[serde(rename = "980")]
    N980,
    /// Iceland Krona
    #[serde(rename = "352")]
    N352,
    /// Indian Rupee
    #[serde(rename = "356")]
    N356,
    /// Iranian Rial
    #[serde(rename = "364")]
    N364,
    /// Iraqi Dinar
    #[serde(rename = "368")]
    N368,
    /// Irish Pound
    #[serde(rename = "372")]
    N372,
    /// Italian Lira
    #[serde(rename = "380")]
    N380,
    /// Jamaican Dollar
    #[serde(rename = "388")]
    N388,
    /// Jordanian Dinar
    #[serde(rename = "400")]
    N400,
    /// Kenyan Shilling
    #[serde(rename = "404")]
    N404,
    /// Kina
    #[serde(rename = "598")]
    N598,
    /// Kip
    #[serde(rename = "418")]
    N418,
    /// Kroon
    #[serde(rename = "233")]
    N233,
    /// Kuna
    #[serde(rename = "191")]
    N191,
    /// Kuwaiti Dinar
    #[serde(rename = "414")]
    N414,
    /// Kwacha
    #[serde(rename = "454")]
    N454,
    /// Kwacha
    #[serde(rename = "894")]
    N894,
    /// Kwanza Reajustado
    #[serde(rename = "982")]
    N982,
    /// Kyat
    #[serde(rename = "104")]
    N104,
    /// Lari
    #[serde(rename = "981")]
    N981,
    /// Latvian Lats
    #[serde(rename = "428")]
    N428,
    /// Lebanese Pound
    #[serde(rename = "422")]
    N422,
    /// Lek
    #[serde(rename = "008")]
    N008,
    /// Lempira
    #[serde(rename = "340")]
    N340,
    /// Leone
    #[serde(rename = "694")]
    N694,
    /// Leu
    #[serde(rename = "642")]
    N642,
    /// Lev
    #[serde(rename = "100")]
    N100,
    /// Liberian Dollar
    #[serde(rename = "430")]
    N430,
    /// Libyan Dinar
    #[serde(rename = "434")]
    N434,
    /// Lilangeni
    #[serde(rename = "748")]
    N748,
    /// Lithuanian Litas
    #[serde(rename = "440")]
    N440,
    /// Loti
    #[serde(rename = "426")]
    N426,
    /// Luxembourg Franc
    #[serde(rename = "442")]
    N442,
    /// Malagasy Franc
    #[serde(rename = "450")]
    N450,
    /// Malaysian Ringgit
    #[serde(rename = "458")]
    N458,
    /// Maltese Lira
    #[serde(rename = "470")]
    N470,
    /// Manat
    #[serde(rename = "795")]
    N795,
    /// Markka
    #[serde(rename = "246")]
    N246,
    /// Mauritius Rupee
    #[serde(rename = "480")]
    N480,
    /// Metical
    #[serde(rename = "508")]
    N508,
    /// Mexican Peso
    #[serde(rename = "484")]
    N484,
    /// Mexican Unidad de Inversion (UDI)
    #[serde(rename = "979")]
    N979,
    /// Moldovan Leu
    #[serde(rename = "498")]
    N498,
    /// Moroccan Dirham
    #[serde(rename = "504")]
    N504,
    /// Mvdol
    #[serde(rename = "984")]
    N984,
    /// Naira
    #[serde(rename = "566")]
    N566,
    /// Nakfa
    #[serde(rename = "232")]
    N232,
    /// Namibia Dollar
    #[serde(rename = "516")]
    N516,
    /// Nepalese Rupee
    #[serde(rename = "524")]
    N524,
    /// Netherlands Antillian Guilder
    #[serde(rename = "532")]
    N532,
    /// Netherlands Guilder
    #[serde(rename = "528")]
    N528,
    /// New Dinar
    #[serde(rename = "891")]
    N891,
    /// New Israeli Sheqel
    #[serde(rename = "376")]
    N376,
    /// New Kwanza
    #[serde(rename = "02")]
    N02,
    /// New Taiwan Dollar
    #[serde(rename = "901")]
    N901,
    /// New Zaire
    #[serde(rename = "180")]
    N180,
    /// New Zealand Dollar
    #[serde(rename = "554")]
    N554,
    /// Next day
    #[serde(rename = "997")]
    N997,
    /// Ngultrum
    #[serde(rename = "064")]
    N064,
    /// North Korean Won
    #[serde(rename = "408")]
    N408,
    /// Norwegian Krone
    #[serde(rename = "578")]
    N578,
    /// Nuevo Sol
    #[serde(rename = "604")]
    N604,
    /// Ouguiya
    #[serde(rename = "478")]
    N478,
    /// Pa'anga
    #[serde(rename = "776")]
    N776,
    /// Pakistan Rupee
    #[serde(rename = "586")]
    N586,
    /// Pataca
    #[serde(rename = "446")]
    N446,
    /// Peso Uruguayo
    #[serde(rename = "858")]
    N858,
    /// Philippine Peso
    #[serde(rename = "608")]
    N608,
    /// Portuguese Escudo
    #[serde(rename = "620")]
    N620,
    /// Pound Sterling
    #[serde(rename = "826")]
    N826,
    /// Pula
    #[serde(rename = "072")]
    N072,
    /// Qatari Rial
    #[serde(rename = "634")]
    N634,
    /// Quetzal
    #[serde(rename = "320")]
    N320,
    /// Rand
    #[serde(rename = "710")]
    N710,
    /// Rial Omani
    #[serde(rename = "512")]
    N512,
    /// Riel
    #[serde(rename = "116")]
    N116,
    /// Rufiyaa
    #[serde(rename = "462")]
    N462,
    /// Rupiah
    #[serde(rename = "360")]
    N360,
    /// Russian Ruble
    #[serde(rename = "643")]
    N643,
    /// Russian Ruble
    #[serde(rename = "810")]
    N810,
    /// Rwanda Franc
    #[serde(rename = "646")]
    N646,
    /// SDR
    #[serde(rename = "960")]
    N960,
    /// Same day
    #[serde(rename = "998")]
    N998,
    /// Saudi Riyal
    #[serde(rename = "682")]
    N682,
    /// Schilling
    #[serde(rename = "040")]
    N040,
    /// Seychelles Rupee
    #[serde(rename = "690")]
    N690,
    /// Singapore Dollar
    #[serde(rename = "702")]
    N702,
    /// Slovak Koruna
    #[serde(rename = "703")]
    N703,
    /// Solomon Islands Dollar
    #[serde(rename = "090")]
    N090,
    /// Som
    #[serde(rename = "417")]
    N417,
    /// Somali Shilling
    #[serde(rename = "706")]
    N706,
    /// Spanish Peseta
    #[serde(rename = "724")]
    N724,
    /// Sri Lanka Rupee
    #[serde(rename = "144")]
    N144,
    /// St Helena Pound
    #[serde(rename = "654")]
    N654,
    /// Sucre
    #[serde(rename = "218")]
    N218,
    /// Sudanese Dinar
    #[serde(rename = "736")]
    N736,
    /// Surinam Guilder
    #[serde(rename = "740")]
    N740,
    /// Swedish Krona
    #[serde(rename = "752")]
    N752,
    /// Swiss Franc
    #[serde(rename = "756")]
    N756,
    /// Syrian Pound
    #[serde(rename = "760")]
    N760,
    /// Tajik Ruble
    #[serde(rename = "762")]
    N762,
    /// Taka
    #[serde(rename = "050")]
    N050,
    /// Tala
    #[serde(rename = "882")]
    N882,
    /// Tanzanian Shilling
    #[serde(rename = "834")]
    N834,
    /// Tenge
    #[serde(rename = "398")]
    N398,
    /// Timor Escudo
    #[serde(rename = "626")]
    N626,
    /// Tolar
    #[serde(rename = "705")]
    N705,
    /// Trinidad and Tobago Dollar
    #[serde(rename = "780")]
    N780,
    /// Tugrik
    #[serde(rename = "496")]
    N496,
    /// Tunisian Dinar
    #[serde(rename = "788")]
    N788,
    /// Turkish Lira
    #[serde(rename = "792")]
    N792,
    /// UAE Dirham
    #[serde(rename = "784")]
    N784,
    /// US Dollar
    #[serde(rename = "840")]
    N840,
    /// Uganda Shilling
    #[serde(rename = "800")]
    N800,
    /// Unidad de Valor Constante (UVC)
    #[serde(rename = "983")]
    N983,
    /// Unidades de fomento
    #[serde(rename = "990")]
    N990,
    /// Uzbekistan Sum
    #[serde(rename = "860")]
    N860,
    /// Vatu
    #[serde(rename = "548")]
    N548,
    /// Won
    #[serde(rename = "410")]
    N410,
    /// Yemeni Rial
    #[serde(rename = "886")]
    N886,
    /// Yen
    #[serde(rename = "392")]
    N392,
    /// Yuan Renminbi
    #[serde(rename = "156")]
    N156,
    /// Zimbabwe Dollar
    #[serde(rename = "716")]
    N716,
    /// Zloty
    #[serde(rename = "985")]
    N985,
    /// financial Rand
    #[serde(rename = "991")]
    N991,
    /// Gold
    XAU,
    /// European Composite Unit (EURCO)
    XBA,
    /// European Monetary Unit (E.M.U.-6)
    XBB,
    /// European Unit of Account 9 (E.U.A.- 9)
    XBC,
    /// European Unit of Account 17 (E.U.A.- 17)
    XBD,
    /// Palladium
    XPD,
    /// Platinum
    XPT,
    /// Silver
    XAG,
    /// UIC-Franc
    XFU,
    /// Gold-Franc
    XFO,
    /// Codes specifically reserved for testing purposes
    XTS,
    /// Codes assigned for transactions where no currency is involved
    XXX,
    /// Gold
    #[serde(rename = "959")]
    N959,
    /// European Composite Unit (EURCO)
    #[serde(rename = "955")]
    N955,
    /// European Monetary Unit (E.M.U.-6)
    #[serde(rename = "956")]
    N956,
    /// European Unit of Account 9 (E.U.A.- 9)
    #[serde(rename = "957")]
    N957,
    /// European Unit of Account 17 (E.U.A.- 17)
    #[serde(rename = "958")]
    N958,
    /// Palladium
    #[serde(rename = "964")]
    N964,
    /// Platinum
    #[serde(rename = "962")]
    N962,
    /// Silver
    #[serde(rename = "961")]
    N961,
    /// Codes specifically reserved for testing purposes
    #[serde(rename = "963")]
    N963,
    /// Codes assigned for transactions where no currency is involved
    #[serde(rename = "999")]
    N999,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum StrikePriceDeterminationMethod {
    /// Fixed Strike
    #[serde(rename = "1")]
    Fixed,
    /// Strike set at expiration to underlying or other value (lookback floating)
    #[serde(rename = "2")]
    Expiration,
    /// Strike set to average of underlying settlement price across the life of the option
    #[serde(rename = "3")]
    Average,
    /// Strike set to optimal value
    #[serde(rename = "4")]
    Optimal,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum StrikePriceBoundaryMethod {
    /// Less than underlying price is in-the-money (ITM)
    #[serde(rename = "1")]
    LT,
    /// Less than or equal to the underlying price is in-the-money(ITM)
    #[serde(rename = "2")]
    LE,
    /// Equal to the underlying price is in-the-money(ITM)
    #[serde(rename = "3")]
    EQ,
    /// Greater than or equal to underlying price is in-the-money(ITM)
    #[serde(rename = "4")]
    GE,
    /// Greater than underlying is in-the-money(ITM)
    #[serde(rename = "5")]
    GT,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UnderlyingPriceDeterminationMethod {
    /// Regular
    #[serde(rename = "1")]
    Regular,
    /// Special reference
    #[serde(rename = "2")]
    Special,
    /// Optimal value (Lookback)
    #[serde(rename = "3")]
    Optimal,
    /// Average value (Asian option)
    #[serde(rename = "4")]
    Average,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ContractMultiplierUnit {
    /// Shares
    #[serde(rename = "0")]
    Shares,
    /// Hours
    #[serde(rename = "1")]
    Hours,
    /// Days
    #[serde(rename = "2")]
    Days,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum FlowScheduleType {
    /// NERC Eastern Off-Peak
    #[serde(rename = "0")]
    NERCEasternOffPeak,
    /// NERC Western Off-Peak
    #[serde(rename = "1")]
    NERCWesternOffPeak,
    /// NERC Calendar-All Days in month
    #[serde(rename = "2")]
    NERCCalendar,
    /// NERC Eastern Peak
    #[serde(rename = "3")]
    NERCEasternPeak,
    /// NERC Western Peak
    #[serde(rename = "4")]
    NERCWesternPeak,
    /// All times
    #[serde(rename = "5")]
    AllTimes,
    /// On peak
    #[serde(rename = "6")]
    OnPeak,
    /// Off peak
    #[serde(rename = "7")]
    OffPeak,
    /// Base
    #[serde(rename = "8")]
    Base,
    /// Block
    #[serde(rename = "9")]
    Block,
    /// Other
    #[serde(rename = "99")]
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UnitOfMeasure {
    /// Barrels
    #[serde(rename = "Bbl")]
    Barrels,
    /// Billion cubic feet
    #[serde(rename = "Bcf")]
    BillionCubicFeet,
    /// Bushels
    #[serde(rename = "Bu")]
    Bushels,
    /// Pounds
    #[serde(rename = "lbs")]
    Pounds,
    /// Gallons
    #[serde(rename = "Gal")]
    Gallons,
    /// Million Barrels (deprecated in FIX 5.0 SP1)
    #[serde(rename = "MMbbl")]
    MillionBarrels,
    /// One Million BTU
    #[serde(rename = "MMBtu")]
    OneMillionBTU,
    /// Megawatt hours
    #[serde(rename = "MWh")]
    MegawattHours,
    /// Troy Ounces
    #[serde(rename = "oz_tr")]
    TroyOunces,
    /// Metric Tons (aka Tonne)
    #[serde(rename = "t")]
    Tonne,
    /// Tons (US)
    #[serde(rename = "tn")]
    Tons,
    /// US Dollars
    #[serde(rename = "USD")]
    USDollars,
    /// Allowances
    #[serde(rename = "Alw")]
    Allowances,
    /// Cubic Meters
    #[serde(rename = "CBM")]
    CubicMeters,
    /// Certified Emissions Reduction
    #[serde(rename = "CER")]
    CertifiedEmissionsReduction,
    /// Principal with relation to debt instrument
    #[serde(rename = "PRINC")]
    PrincipalWithRelationToDebtInstrument,
    /// ClimateReserveTonnes (NOTE: This is added as a Variable Quantity UOM)
    #[serde(rename = "CRT")]
    ClimateReserveTonnes,
    /// Amount of currency
    #[serde(rename = "Ccy")]
    AmountOfCurrency,
    /// Board feet
    #[serde(rename = "BDFT")]
    BoardFeet,
    /// Index point
    #[serde(rename = "IPNT")]
    IndexPoint,
    /// Day
    #[serde(rename = "day")]
    Day,
    /// Hundredweight (US)
    #[serde(rename = "cwt")]
    Hundredweight,
    /// Grams
    #[serde(rename = "g")]
    Grams,
    /// Dry metric tons
    #[serde(rename = "dt")]
    DryMetricTons,
    /// Kilowatt hours
    #[serde(rename = "kWh")]
    KilowattHours,
    /// Environmental Offset
    #[serde(rename = "EnvOfst")]
    EnvironmentalOffset,
    /// Environmental Credit
    #[serde(rename = "EnvCrd")]
    EnvironmentalCredit,
    /// Kilowatt-Minute(electrical capacity)
    #[serde(rename = "kW-min")]
    KilowattMinute,
    /// therms
    #[serde(rename = "thm")]
    Therms,
    /// gigajoules
    #[serde(rename = "GJ")]
    Gigajoules,
    /// liters
    #[serde(rename = "L")]
    Liters,
    /// kiloliters
    #[serde(rename = "kL")]
    Kiloliters,
    /// Gross Tons(Elaboration: long tons or imperial tons, equal to 2240 lbs)
    #[serde(rename = "GT")]
    GrossTons,
    /// Kilograms
    #[serde(rename = "kg")]
    Kilograms,
    /// Metric tons
    #[serde(rename = "T")]
    MetricTons,
    /// Cooling degree day
    #[serde(rename = "CDD")]
    CoolingDegreeDay,
    /// Critical precipitation day
    #[serde(rename = "CPD")]
    CriticalPrecipitationDay,
    /// Environmental allowance certificates
    #[serde(rename = "EnvAllwnc")]
    EnvironmentalAllowanceCertificates,
    /// Heating degree day
    #[serde(rename = "HDD")]
    HeatingDegreeDay,
    /// Heat rate. The number of BTUs required to produce one kilowatt hour of electricity, typically 3,412.14 BTUs per 1 kWh
    #[serde(rename = "kHR")]
    HeatHate,
    /// Mega heat rate. The number of million BTUs required to produce one megawatt hour of electricity, typically 3.41214 million BTUs per 1 MWh
    #[serde(rename = "MHR")]
    MegaHeatRate,
    /// Kilowatt year (electrical capacity)
    #[serde(rename = "kW-a")]
    KilowattYear,
    /// Kilowatt day (electrical capacity)
    #[serde(rename = "kW-d")]
    KilowattDay,
    /// Kilowatt hour (electrical capacity)
    #[serde(rename = "kW-h")]
    KilowattHour,
    /// Kilowatt month (electrical capacity)
    #[serde(rename = "kW-M")]
    KilowattMonth,
    /// Megawatt year (electrical capacity)
    #[serde(rename = "MW-a")]
    MegawattYear,
    /// Megawatt day (electrical capacity)
    #[serde(rename = "MW-d")]
    MegawattDay,
    /// Megawatt hour (electrical capacity)
    #[serde(rename = "MW-h")]
    MegawattHour,
    /// Megawatt month (electrical capacity)
    #[serde(rename = "MW-M")]
    MegawattMonth,
    /// Megawatt minute (electrical capacity)
    #[serde(rename = "MW-min")]
    MegawattMinute,
    /// Tons of carbon dioxide
    #[serde(rename = "tnCO2")]
    TonsOfCarbonDioxide,
    /// Are
    #[serde(rename = "a")]
    Are,
    /// Acre
    #[serde(rename = "ac")]
    Acre,
    /// Centiliter
    #[serde(rename = "cL")]
    Centiliter,
    /// Centimeter
    #[serde(rename = "cM")]
    Centimeter,
    /// Diesel gallon equivalent
    #[serde(rename = "DGE")]
    DieselGallonEquivalent,
    /// Foot
    #[serde(rename = "ft")]
    Foot,
    /// GB Gallon
    #[serde(rename = "Gal_gb")]
    GBGallon,
    /// Gasonline gallon equivalent
    #[serde(rename = "GGE")]
    GasonlineGallonEquivalent,
    /// Hectare
    #[serde(rename = "ha")]
    Hectare,
    /// Inch
    #[serde(rename = "in")]
    Inch,
    /// Kilometer
    #[serde(rename = "kM")]
    Kilometer,
    /// Meter
    #[serde(rename = "M")]
    Meter,
    /// Mile
    #[serde(rename = "mi")]
    Mile,
    /// Milliliter
    #[serde(rename = "mL")]
    Milliliter,
    /// Millimeter
    #[serde(rename = "mM")]
    Millimeter,
    /// US ounce
    #[serde(rename = "oz")]
    USOunce,
    /// Piece
    #[serde(rename = "pc")]
    Piece,
    /// US Pint
    #[serde(rename = "pt")]
    USPint,
    /// GB pint
    #[serde(rename = "pt_gb")]
    GBPint,
    /// US Quart
    #[serde(rename = "qt")]
    USQuart,
    /// GB Quart
    #[serde(rename = "qt_gb")]
    GBQuart,
    /// Square centimeter
    #[serde(rename = "SqcM")]
    SquareCentimeter,
    /// Square foot
    #[serde(rename = "Sqft")]
    SquareFoot,
    /// Square inch
    #[serde(rename = "Sqin")]
    SquareInch,
    /// Square kilometer
    #[serde(rename = "SqkM")]
    SquareKilometer,
    /// Square meter
    #[serde(rename = "SqM")]
    SquareMeter,
    /// Square mile
    #[serde(rename = "Sqmi")]
    SquareMile,
    /// Square millimeter
    #[serde(rename = "SqmM")]
    SquareMillimeter,
    /// Square yard
    #[serde(rename = "Sqyd")]
    SquareYard,
    /// Yard
    #[serde(rename = "yd")]
    Yard,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SettlMethod {
    /// Election at exercise (The settlement method will be elected at the time of contract exercise)
    #[serde(rename = "E")]
    Election,
    /// Cash settlement required
    #[serde(rename = "C")]
    Cash,
    /// Physical settlement required
    #[serde(rename = "P")]
    Physical,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ExerciseStyle {
    /// European
    #[serde(rename = "0")]
    European,
    /// American
    #[serde(rename = "1")]
    American,
    /// Bermuda
    #[serde(rename = "2")]
    Bermuda,
    /// Other
    #[serde(rename = "99")]
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OptPayoutType {
    /// Vanilla
    #[serde(rename = "1")]
    Vanilla,
    /// Capped
    #[serde(rename = "2")]
    Capped,
    /// Digital (Binary)
    #[serde(rename = "3")]
    Digital,
    /// Asian
    #[serde(rename = "4")]
    Asian,
    /// Barrier
    #[serde(rename = "5")]
    Barrier,
    /// Digital Barrier
    #[serde(rename = "6")]
    DigitalBarrier,
    /// Lookback
    #[serde(rename = "7")]
    Lookback,
    /// Other path dependent
    #[serde(rename = "8")]
    OtherPathDependent,
    /// Other
    #[serde(rename = "99")]
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum PriceQuoteMethod {
    /// Standard, money per unit of a physical
    #[serde(rename = "STD")]
    Standard,
    /// Index
    #[serde(rename = "INX")]
    Index,
    /// Interest rate Index
    #[serde(rename = "INT")]
    InterestRateIndex,
    /// Percent of Par
    #[serde(rename = "PCTPAR")]
    PercentOfPar,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ValuationMethod {
    /// premium style
    EQTY,
    /// futures style mark-to-market
    FUT,
    /// futures style with an attached cash adjustment
    FUTDA,
    /// CDS style collateralization of market to market and coupon
    CDS,
    /// CDS in delivery - use recovery rate to calculate obligation
    CDSD,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ListMethod {
    /// pre-listed only
    #[serde(rename = "0")]
    PreListedOnly,
    /// user requested
    #[serde(rename = "1")]
    UserRequested,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum PutOrCall {
    /// Put
    #[serde(rename = "0")]
    Put,
    /// Call
    #[serde(rename = "1")]
    Call,
    /// Other
    #[serde(rename = "2")]
    Other,
    /// Chooser
    #[serde(rename = "3")]
    Chooser,
}

#[derive(Debug, Default)]
pub struct EncodedIssuer {
    // #[serde(rename = "348")]
    len: usize,
    // #[serde(rename = "349")]
    data: String,
}

impl data_field::DataField for EncodedIssuer {
    fn get_len(&self) -> &usize {
        &self.len
    }
    fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    fn get_data(&self) -> &str {
        &self.data
    }
    fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

impl<'de> Deserialize<'de> for EncodedIssuer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "349")
    }
}

impl Serialize for EncodedIssuer {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "349")
    }
}

#[derive(Debug, Default)]
pub struct EncodedSecurityDesc {
    // #[serde(rename = "350")]
    len: usize,
    // #[serde(rename = "351")]
    data: String,
}

impl data_field::DataField for EncodedSecurityDesc {
    fn get_len(&self) -> &usize {
        &self.len
    }
    fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    fn get_data(&self) -> &str {
        &self.data
    }
    fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

impl<'de> Deserialize<'de> for EncodedSecurityDesc {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "351")
    }
}

impl Serialize for EncodedSecurityDesc {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "351")
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum CPProgram {
    /// 3(a)(3)
    #[serde(rename = "1")]
    P3a3,
    /// 4(2)
    #[serde(rename = "2")]
    P4_2,
    /// Other
    #[serde(rename = "99")]
    Other,
    /// 3(a)(2)
    #[serde(rename = "3")]
    P3a2,
    /// 3(a)(3) and 3(c)(7)
    #[serde(rename = "4")]
    P3a3_3c7,
    /// 3(a)(4)
    #[serde(rename = "5")]
    P3a4,
    /// 3(a)(5)
    #[serde(rename = "6")]
    P3a5,
    /// 3(a)(7)
    #[serde(rename = "7")]
    P3a7,
    /// 3(c)(7)
    #[serde(rename = "8")]
    P3c7,
}
