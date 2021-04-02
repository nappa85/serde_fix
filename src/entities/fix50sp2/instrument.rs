
use serde::{Serialize, Deserialize};

use crate::entities::{Boolean, LocalMktDate, MonthYear, TZTimeOnly};

use super::{additional_term_grp::AdditionalTermGrp, asset_attribute_grp::AssetAttributeGrp, cash_settl_term_grp::CashSettlTermGrp, complex_events::ComplexEvents, currency::Currency, date_adjustment::DateAdjustment, evnt_grp::EvntGrps, extraordinary_event_grp::ExtraordinaryEventGrp, instrument_parties::InstrumentParties, market_disruption::MarketDisruption, option_exercise::OptionExercise, physical_settl_term_grp::PhysicalSettlTermGrp, pricing_date_time::PricingDateTime, protection_term_grp::ProtectionTermGrp, provision_grp::ProvisionGrp, sec_alt_id_grp::SecAltIDGrp, secondary_asset_grp::SecondaryAssetGrp, security_xml::SecurityXML, stream_grp::StreamGrp};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Instrument {
	/// Common, "human understood" representation of the security. <a href="tag_48_SecurityID_.html">SecurityID&nbsp;(48)</a> value can be specified if no symbol exists (e.g. non-exchange traded Collective Investment Vehicles). Use "[N/A]" for products
	/// which do not have a symbol.
	#[serde(rename = "55")]
	pub symbol: Option<String>,
	/// Used in Fixed Income with a value of "WI" to indicate "When Issued" for a security to be reissued under an old CUSIP or ISIN
	/// or with a value of "CD" to indicate a EUCP with lump-sum interest rather than discount price.
	#[serde(rename = "65")]
	pub symbol_sfx: Option<String>,
	/// Takes precedence in identifying security to counterparty over <a href="tag_455_SecurityAltID_.html">SecurityAltID&nbsp;(455)</a> block. Requires <a href="tag_22_SecurityIDSource_.html">SecurityIDSource&nbsp;(22)</a> if specified.
	#[serde(rename = "48")]
	pub security_id: Option<String>,
	/// Required if <a href="tag_48_SecurityID_.html">SecurityID&nbsp;(48)</a> is specified.
	#[serde(rename = "22")]
	pub security_id_source: Option<SecurityIDSource>,
	/// Number of alternate Security Identifiers
	#[serde(flatten)]
	pub sec_alt_id_grp: Option<SecAltIDGrp>,
	/// Indicates the type of product the security is associated with (high-level category)
	#[serde(rename = "460")]
	pub product: Option<Product>,
	/// Identifies an entire suite of products for a given market. In Futures this may be "interest rates", "agricultural", "equity
	/// indexes", etc
	#[serde(rename = "1227")]
	pub product_complex: Option<String>,
	/// An exchange specific name assigned to a group of related securities which may be concurrently affected by market events and
	/// actions.
	#[serde(rename = "1151")]
	pub security_group: Option<String>,
	/// Indicates the type of security using ISO 10962 standard, Classification of Financial Instruments (CFI code) values. It is
	/// recommended that <a href="tag_461_CFICode_.html">CFICode&nbsp;(461)</a> be used instead of <a href="tag_167_SecurityType_.html">SecurityType&nbsp;(167)</a> for non-Fixed Income instruments.
	#[serde(rename = "461")]
	pub cfi_code: Option<String>,
	/// It is recommended that <a href="tag_461_CFICode_.html">CFICode&nbsp;(461)</a> be used instead of <a href="tag_167_SecurityType_.html">SecurityType&nbsp;(167)</a> for non-Fixed Income instruments. Required for Fixed Income. Refer to Volume 7 - Fixed Income Futures and Options should be
	/// specified using the <a href="tag_461_CFICode_.html">CFICode&nbsp;(461)</a> field instead of <a href="tag_167_SecurityType_.html">SecurityType&nbsp;(167)</a> (Refer to Volume 7 - Recommendations and Guidelines for Futures and Options Markets.)
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// Sub-type qualification/identification of the <a href="tag_167_SecurityType_.html">SecurityType&nbsp;(167)</a> (e.g. for SecurityType="MLEG"). If specified, <a href="tag_167_SecurityType_.html">SecurityType&nbsp;(167)</a> is required.
	#[serde(rename = "762")]
	pub security_sub_type: Option<String>,
	/// Specifies the month and year of maturity. Applicable for standardized derivatives which are typically only referenced b y
	/// month and year (e.g. S&amp;P futures). Note <a href="tag_541_MaturityDate_.html">MaturityDate&nbsp;(541)</a> (a full date) can also be specified.
	#[serde(rename = "200")]
	pub maturity_month_year: Option<MonthYear>,
	/// Specifies date of maturity (a full date). Note that standardized derivatives which are typically only referenced by month
	/// and year (e.g. S&amp;P futures).may use <a href="tag_200_MaturityMonthYear_.html">MaturityMonthYear&nbsp;(200)</a> and/or this field. When using MaturityMonthYear, it is recommended that markets and sell sides report the <a href="tag_541_MaturityDate_.html">MaturityDate&nbsp;(541)</a> on all outbound messages as a means of data enrichment.
	#[serde(rename = "541")]
	pub maturity_date: Option<LocalMktDate>,
	/// For NDFs this represents the fixing time of the contract. It is optional to specify the fixing time.
	#[serde(rename = "1079")]
	pub maturity_time: Option<TZTimeOnly>,
	/// Indicator to determine if <a href="block_Instrument_.html">Instrument</a> is Settle on Open.
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
	pub repo_collateral_security_type: Option<SecurityType>,
	/// (Deprecated in FIX.4.4)
	#[serde(rename = "226")]
	pub repurchase_term: Option<i32>,
	/// (Deprecated in FIX.4.4)
	#[serde(rename = "227")]
	pub repurchase_rate: Option<f32>,
	/// For Fixed Income: Amortization Factor for deriving Current face from Original face for ABS or MBS securities, note the fraction
	/// may be greater than, equal to or less than 1. In TIPS securities this is the Inflation index. Qty * Factor * Price = Gross
	/// Trade Amount. For Derivatives: Contract Value Factor by which price must be adjusted to determine the true nominal value of
	/// one futures/options contract. (Qty * Price) * Factor = Nominal Value.
	#[serde(rename = "228")]
	pub factor: Option<f64>,
	/// CreditRating
	#[serde(rename = "255")]
	pub credit_rating: Option<String>,
	/// The location at which records of ownership are maintained for this instrument, and at which ownership changes must be recorded.
	/// Can be used in conjunction with ISIN to address ISIN uniqueness issues.
	#[serde(rename = "543")]
	pub instr_registry: Option<String>,
	/// ISO Country code of instrument issue (e.g. the country portion typically used in ISIN). Can be used in conjunction with non-ISIN <a href="tag_48_SecurityID_.html">SecurityID&nbsp;(48)</a> (e.g. CUSIP for Municipal Bonds without ISIN) to provide uniqueness.
	#[serde(rename = "470")]
	pub country_of_issue: Option<CountryOfIssue>,
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
	/// Used for derivatives, such as options and covered warrants to indicate a versioning of the contract when required due to corporate
	/// actions to the underlying. Should not be used to indicate type of option - use the CFICode[461] for this purpose.
	#[serde(rename = "206")]
	pub opt_attribute: Option<char>,
	/// For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g.
	/// contracts vs. shares) amount.
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
	/// Minimum price increment amount associated with the <a href="tag_969_MinPriceIncrement_.html">MinPriceIncrement&nbsp;(969)</a> . For listed derivatives, the value can be calculated by multiplying MinPriceIncrement by <a href="tag_231_ContractMultiplier_.html">ContractValueFactor&nbsp;(231)</a>
	#[serde(rename = "1146")]
	pub min_price_increment_amount: Option<f64>,
	/// Used to indicate the size of the underlying commodity on which the contract is based (e.g., 2500 lbs of lean cattle, 1000
	/// barrels of crude oil, 1000 bushels of corn, etc.)
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
	/// Used to indicate if a security has been defined as flexible according to "non-standard" means. Analog to CFICode Standard/Non-standard
	/// indicator
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
	#[serde(rename = "348")]
	pub encoded_issuer_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the Issuer(106) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(rename = "349")]
	pub encoded_issuer: Option<String>,
	/// SecurityDesc
	#[serde(rename = "107")]
	pub security_desc: Option<String>,
	/// Must be set if EncodedSecurityDesc(351) field is specified and must immediately precede it.
	#[serde(rename = "350")]
	pub encoded_security_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the SecurityDesc(107) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(rename = "351")]
	pub encoded_security_desc: Option<String>,
	/// Embedded XML document describing the instrument.
	#[serde(flatten)]
	pub security_xml: Option<SecurityXML>,
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
	pub evnt_grp: Option<EvntGrps>,
	/// If different from IssueDate
	#[serde(rename = "873")]
	pub dated_date: Option<LocalMktDate>,
	/// If different from <a href="tag_225_IssueDate_.html">IssueDate&nbsp;(225)</a> and DatedDate
	#[serde(rename = "874")]
	pub interest_accrual_date: Option<LocalMktDate>,
	/// Used to identify the parties related to a specific instrument.
	#[serde(flatten)]
	pub instrument_parties: Option<InstrumentParties>,
	/// ComplexEvents
	#[serde(flatten)]
	pub complex_events: Option<ComplexEvents>,
	/// PriceQuoteCurrency
	#[serde(rename = "1524")]
	pub price_quote_currency: Option<Currency>,
	/// ObligationType
	#[serde(rename = "1739")]
	pub obligation_type: Option<ObligationType>,
	/// ShortSaleRestriction
	#[serde(rename = "1687")]
	pub short_sale_restriction: Option<ShortSaleRestriction>,
	/// UnitOfMeasureCurrency
	#[serde(rename = "1716")]
	pub unit_of_measure_currency: Option<Currency>,
	/// PriceUnitOfMeasureCurrency
	#[serde(rename = "1717")]
	pub price_unit_of_measure_currency: Option<Currency>,
	/// <p>Spread table code referred by the security or symbol</p>
	#[serde(rename = "1787")]
	pub ref_tick_table_id: Option<i32>,
	/// Required if AssetSubClass(1939) is specified.
	#[serde(rename = "1938")]
	pub asset_class: Option<AssetClass>,
	/// Required if AssetType(1940) is specified.
	#[serde(rename = "1939")]
	pub asset_sub_class: Option<AssetSubClass>,
	/// Required if AssetSubType(2735) is specified.
	#[serde(rename = "1940")]
	pub asset_type: Option<String>,
	/// SecondaryAssetGrp
	#[serde(flatten)]
	pub secondary_asset_grp: Option<SecondaryAssetGrp>,
	/// SwapClass
	#[serde(rename = "1941")]
	pub swap_class: Option<SwapClass>,
	/// Conditionally required when MthToDefault(1943) is specified.
	#[serde(rename = "1942")]
	pub nth_to_default: Option<i32>,
	/// MthToDefault
	#[serde(rename = "1943")]
	pub mth_to_default: Option<i32>,
	/// SettledEntityMatrixSource
	#[serde(rename = "1944")]
	pub settled_entity_matrix_source: Option<String>,
	/// SettledEntityMatrixPublicationDate
	#[serde(rename = "1945")]
	pub settled_entity_matrix_publication_date: Option<LocalMktDate>,
	/// CouponType
	#[serde(rename = "1946")]
	pub coupon_type: Option<CouponType>,
	/// TotalIssuedAmount
	#[serde(rename = "1947")]
	pub total_issued_amount: Option<f64>,
	/// Conditionally required when CouponFrequencyUnit(1949) is specified.
	#[serde(rename = "1948")]
	pub coupon_frequency_period: Option<i32>,
	/// Conditionally required when CouponFrequencyPeriod(1948) is specified.
	#[serde(rename = "1949")]
	pub coupon_frequency_unit: Option<CouponFrequencyUnit>,
	/// CouponDayCount
	#[serde(rename = "1950")]
	pub coupon_day_count: Option<CouponDayCount>,
	/// ConvertibleBondEquityID
	#[serde(rename = "1951")]
	pub convertible_bond_equity_id: Option<String>,
	/// Conditionally required when ConvertibleBondEquityID(1951) is specified.
	#[serde(rename = "1952")]
	pub convertible_bond_equity_id_source: Option<ConvertibleBondEquityIDSource>,
	/// ContractPriceRefMonth
	#[serde(rename = "1953")]
	pub contract_price_ref_month: Option<MonthYear>,
	/// LienSeniority
	#[serde(rename = "1954")]
	pub lien_seniority: Option<LienSeniority>,
	/// LoanFacility
	#[serde(rename = "1955")]
	pub loan_facility: Option<LoanFacility>,
	/// ReferenceEntityType
	#[serde(rename = "1956")]
	pub reference_entity_type: Option<ReferenceEntityType>,
	/// IndexSeries
	#[serde(rename = "1957")]
	pub index_series: Option<i32>,
	/// IndexAnnexVersion
	#[serde(rename = "1958")]
	pub index_annex_version: Option<i32>,
	/// IndexAnnexDate
	#[serde(rename = "1959")]
	pub index_annex_date: Option<LocalMktDate>,
	/// IndexAnnexSource
	#[serde(rename = "1960")]
	pub index_annex_source: Option<String>,
	/// DateAdjustment
	#[serde(flatten)]
	pub date_adjustment: Option<DateAdjustment>,
	/// StreamGrp
	#[serde(flatten)]
	pub stream_grp: Option<StreamGrp>,
	/// ProvisionGrp
	#[serde(flatten)]
	pub provision_grp: Option<ProvisionGrp>,
	/// AdditionalTermGrp
	#[serde(flatten)]
	pub additional_term_grp: Option<AdditionalTermGrp>,
	/// ProtectionTermGrp
	#[serde(flatten)]
	pub protection_term_grp: Option<ProtectionTermGrp>,
	/// CashSettlTermGrp
	#[serde(flatten)]
	pub cash_settl_term_grp: Option<CashSettlTermGrp>,
	/// PhysicalSettlTermGrp
	#[serde(flatten)]
	pub physical_settl_term_grp: Option<PhysicalSettlTermGrp>,
	/// AssetAttributeGrp
	#[serde(flatten)]
	pub asset_attribute_grp: Option<AssetAttributeGrp>,
	/// SwapSubClass
	#[serde(rename = "1575")]
	pub swap_sub_class: Option<SwapSubClass>,
	/// SettlRateIndex
	#[serde(rename = "1577")]
	pub settl_rate_index: Option<String>,
	/// SettlRateIndexLocation
	#[serde(rename = "1580")]
	pub settl_rate_index_location: Option<String>,
	/// OptionExpirationDesc
	#[serde(rename = "1581")]
	pub option_expiration_desc: Option<String>,
	/// Must be set if EncodedOptionExpirationDesc(1697) field is specified and must immediately precede it.
	#[serde(rename = "1678")]
	pub encoded_option_expiration_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the OptionExpirationDesc(1581) field in the encoded format specified via
	/// the MessageEncoding(347) field.
	#[serde(rename = "1697")]
	pub encoded_option_expiration_desc: Option<String>,
	/// StrikeUnitOfMeasure
	#[serde(rename = "1698")]
	pub strike_unit_of_measure: Option<UnitOfMeasure>,
	/// StrikeIndex
	#[serde(rename = "1866")]
	pub strike_index: Option<String>,
	/// StrikeIndexSpread
	#[serde(rename = "2001")]
	pub strike_index_spread: Option<f64>,
	/// ValuationSource
	#[serde(rename = "2002")]
	pub valuation_source: Option<String>,
	/// ValuationReferenceModel
	#[serde(rename = "2140")]
	pub valuation_reference_model: Option<String>,
	/// StrategyType
	#[serde(rename = "2141")]
	pub strategy_type: Option<StrategyType>,
	/// CommonPricingIndicator
	#[serde(rename = "2142")]
	pub common_pricing_indicator: Option<Boolean>,
	/// SettlDisruptionProvision
	#[serde(rename = "2143")]
	pub settl_disruption_provision: Option<SettlDisruptionProvision>,
	/// InstrumentRoundingDirection
	#[serde(rename = "2144")]
	pub instrument_rounding_direction: Option<InstrumentRoundingDirection>,
	/// InstrumentRoundingPrecision
	#[serde(rename = "2145")]
	pub instrument_rounding_precision: Option<i32>,
	/// PricingDateTime
	#[serde(flatten)]
	pub pricing_date_time: Option<PricingDateTime>,
	/// MarketDisruption
	#[serde(flatten)]
	pub market_disruption: Option<MarketDisruption>,
	/// OptionExercise
	#[serde(flatten)]
	pub option_exercise: Option<OptionExercise>,
	/// TradingUnitPeriodMultiplier
	#[serde(rename = "2353")]
	pub trading_unit_period_multiplier: Option<i32>,
	/// AssetGroup
	#[serde(rename = "2210")]
	pub asset_group: Option<AssetGroup>,
	/// OrigStrikePrice
	#[serde(rename = "2578")]
	pub orig_strike_price: Option<f64>,
	/// StrikePricePrecision
	#[serde(rename = "2577")]
	pub strike_price_precision: Option<i32>,
	/// SettlSubMethod
	#[serde(rename = "2579")]
	pub settl_sub_method: Option<SettlSubMethod>,
	/// BlockTradeEligibilityIndicator
	#[serde(rename = "2575")]
	pub block_trade_eligibility_indicator: Option<Boolean>,
	/// LowExercisePriceOptionIndicator
	#[serde(rename = "2574")]
	pub low_exercise_price_option_indicator: Option<Boolean>,
	/// InstrumentPricePrecision
	#[serde(rename = "2576")]
	pub instrument_price_precision: Option<i32>,
	/// StrikeIndexCurvePoint
	#[serde(rename = "2600")]
	pub strike_index_curve_point: Option<String>,
	/// StrikeIndexQuote
	#[serde(rename = "2601")]
	pub strike_index_quote: Option<StrikeIndexQuote>,
	/// ExtraordinaryEventGrp
	#[serde(flatten)]
	pub extraordinary_event_grp: Option<ExtraordinaryEventGrp>,
	/// ExtraordinaryEventAdjustmentMethod
	#[serde(rename = "2602")]
	pub extraordinary_event_adjustment_method: Option<ExtraordinaryEventAdjustmentMethod>,
	/// ExchangeLookAlike
	#[serde(rename = "2603")]
	pub exchange_look_alike: Option<Boolean>,
	/// Used to express in-the-moneyness behavior in general terms for the option without the use of StrikePrice(202) and PutOrCall(201).
	#[serde(rename = "2681")]
	pub in_the_money_condition: Option<InTheMoneyCondition>,
	/// ContraryInstructionEligibilityIndicator
	#[serde(rename = "2685")]
	pub contrary_instruction_eligibility_indicator: Option<Boolean>,
	/// FinancialInstrumentFullName
	#[serde(rename = "2714")]
	pub financial_instrument_full_name: Option<String>,
	/// Must be set if EncodedFinancialInstrumentFullName(2716) field is specified and must immediately precede it.
	#[serde(rename = "2715")]
	pub encoded_financial_instrument_full_name_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the FinancialInstrumentFullName(2714) field in the encoded format specified
	/// via the MessageEncoding(347) field.
	#[serde(rename = "2716")]
	pub encoded_financial_instrument_full_name: Option<String>,
	/// AssetSubType
	#[serde(rename = "2735")]
	pub asset_sub_type: Option<String>,
	/// FinancialInstrumentShortName
	#[serde(rename = "2737")]
	pub financial_instrument_short_name: Option<String>,
	/// ReturnTrigger
	#[serde(rename = "2753")]
	pub return_trigger: Option<ReturnTrigger>,
	/// DeliveryRouteOrCharter
	#[serde(rename = "2752")]
	pub delivery_route_or_charter: Option<String>,
	/// CouponOtherDayCount
	#[serde(rename = "2879")]
	pub coupon_other_day_count: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN
	#[serde(rename = "4")]
	Isin,
	/// RIC
	#[serde(rename = "5")]
	Ric,
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
	ConsolidatedTapeAssociation,
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
	IsdaFpMlProductSpecification,
	/// Option Price Reporting Authority
	#[serde(rename = "J")]
	OptionPriceReportingAuthority,
	/// ISDA/FpML Product URL (URL in SecurityID)
	#[serde(rename = "K")]
	IsdaFpMlProductUrl,
	/// Letter of Credit
	#[serde(rename = "L")]
	LetterOfCredit,
	/// Marketplace-assigned Identifier
	#[serde(rename = "M")]
	MarketplaceAssignedIdentifier,
	/// Markit RED entity CLIP
	#[serde(rename = "N")]
	MarkitRedEntityClip,
	/// Markit RED pair CLIP
	#[serde(rename = "P")]
	MarkitRedPairClip,
	/// CFTC commodity code
	#[serde(rename = "Q")]
	CftcCommodityCode,
	/// ISDA Commodity Reference Price
	#[serde(rename = "R")]
	IsdaCommodityReferencePrice,
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
	FidessaInstrumentMnemonic,
	/// Index name
	#[serde(rename = "W")]
	IndexName,
	/// Uniform Symbol (UMTF Symbol)
	#[serde(rename = "X")]
	UniformSymbol,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Product {
	/// AGENCY
	#[serde(rename = "1")]
	Agency,
	/// COMMODITY
	#[serde(rename = "2")]
	Commodity,
	/// CORPORATE
	#[serde(rename = "3")]
	Corporate,
	/// CURRENCY
	#[serde(rename = "4")]
	Currency,
	/// EQUITY
	#[serde(rename = "5")]
	Equity,
	/// GOVERNMENT
	#[serde(rename = "6")]
	Government,
	/// INDEX
	#[serde(rename = "7")]
	Index,
	/// LOAN
	#[serde(rename = "8")]
	Loan,
	/// MONEYMARKET
	#[serde(rename = "9")]
	Moneymarket,
	/// MORTGAGE
	#[serde(rename = "10")]
	Mortgage,
	/// MUNICIPAL
	#[serde(rename = "11")]
	Municipal,
	/// OTHER
	#[serde(rename = "12")]
	Other,
	/// FINANCING
	#[serde(rename = "13")]
	Financing,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityType {
	/// Future
	#[serde(rename = "FUT")]
	Future,
	/// Option
	#[serde(rename = "OPT")]
	Option,
	/// US Treasury Note (Deprecated Value Use TNOTE)
	#[serde(rename = "UST")]
	UsTreasuryNoteDeprecated,
	/// US Treasury Bill (Deprecated Value Use TBILL)
	#[serde(rename = "USTB")]
	UsTreasuryBillDeprecated,
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
	UsdSupranationalCoupons,
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
	UsTreasuryBond,
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
	UsTreasuryNote,
	/// US Treasury Bill
	#[serde(rename = "TBILL")]
	UsTreasuryBill,
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
	/// Amended &amp; Restated
	#[serde(rename = "AMENDED")]
	Amended,
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
	AssetBackedSecurities,
	/// Corp. Mortgage-backed Securities
	#[serde(rename = "CMBS")]
	Corp,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	CollateralizedMortgageObligation,
	/// IOETTE Mortgage
	#[serde(rename = "IET")]
	IoetteMortgage,
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
	UsCorporateFloatingRateNotes,
	/// Canadian Provincial Bonds
	#[serde(rename = "PROV")]
	CanadianProvincialBonds,
	/// Secured Liquidity Note
	#[serde(rename = "SLQN")]
	SecuredLiquidityNote,
	/// Treasury Bill - non US
	#[serde(rename = "TB")]
	TreasuryBill,
	/// Term Liquidity Note
	#[serde(rename = "TLQN")]
	TermLiquidityNote,
	/// Taxable Municipal CP
	#[serde(rename = "TMCP")]
	TaxableMunicipalCp,
	/// Wildcard entry for use on Security Definition Request
	#[serde(rename = "?")]
	WildcardEntryForUseOnSecurityDefinitionRequest,
	/// Options on Combo
	#[serde(rename = "OOC")]
	OptionsOnCombo,
	/// Non-deliverable forward
	#[serde(rename = "FXNDF")]
	NonDeliverableForward,
	/// FX Spot
	#[serde(rename = "FXSPOT")]
	FxSpot,
	/// FX Forward
	#[serde(rename = "FXFWD")]
	FxForward,
	/// FX Swap
	#[serde(rename = "FXSWAP")]
	FxSwap,
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
	/// Cap (In an interest rate cap, the buyer receives payments at the end of each period in which the rate indec exceeds the agreed
	/// strike rate)
	#[serde(rename = "CAP")]
	Cap,
	/// Collar (In an interest rate collar, this is a combination of a cap and a floor)
	#[serde(rename = "CLLR")]
	Collar,
	/// Exotic
	#[serde(rename = "EXOTIC")]
	Exotic,
	/// Floor (In an interest rate floor, the buyer receives payments at the end of each period in which the rate index is below the
	/// agreed strike rate)
	#[serde(rename = "FLR")]
	Floor,
	/// Forward Rate Agreement
	#[serde(rename = "FRA")]
	ForwardRateAgreement,
	/// Loan/lease
	#[serde(rename = "LOANLEASE")]
	LoanLease,
	/// Spot forward
	#[serde(rename = "SPOTFWD")]
	SpotForward,
	/// Transmission
	#[serde(rename = "XMISSION")]
	Transmission,
	/// General type for a contract based on an established index
	#[serde(rename = "INDEX")]
	GeneralTypeForAContractBasedOnAnEstablishedIndex,
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
	#[serde(rename = "FWDSWAP ")]
	ForwardsOnASwap,
	/// Forward Freight Agreement
	#[serde(rename = "FWDFRTAGMT ")]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InstrmtAssignmentMethod {
	/// Random
	#[serde(rename = "R")]
	Random,
	/// ProRata
	#[serde(rename = "P")]
	ProRata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecurityStatus {
	/// Active
	#[serde(rename = "1")]
	Active,
	/// Inactive
	#[serde(rename = "2")]
	Inactive,
	/// Active, closing orders only
	#[serde(rename = "3")]
	ActiveClosing,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	/// Senior Non-Preferred
	#[serde(rename = "SN")]
	SeniorNonPreferred,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CountryOfIssue {
	/// AFGHANISTAN
	#[serde(rename = "AF")]
	Afghanistan,
	/// ALBANIA
	#[serde(rename = "AL")]
	Albania,
	/// ALGERIA
	#[serde(rename = "DZ")]
	Algeria,
	/// AMERICAN SAMOA
	#[serde(rename = "AS")]
	AmericanSamoa,
	/// ANDORRA
	#[serde(rename = "AD")]
	Andorra,
	/// ANGOLA
	#[serde(rename = "AO")]
	Angola,
	/// ANGUILLA
	#[serde(rename = "AI")]
	Anguilla,
	/// ANTARCTICA
	#[serde(rename = "AQ")]
	Antarctica,
	/// ANTIGUA AND BARBUDA
	#[serde(rename = "AG")]
	AntiguaAndBarbuda,
	/// ARGENTINA
	#[serde(rename = "AR")]
	Argentina,
	/// ARMENIA
	#[serde(rename = "AM")]
	Armenia,
	/// ARUBA
	#[serde(rename = "AW")]
	Aruba,
	/// AUSTRALIA
	#[serde(rename = "AU")]
	Australia,
	/// AUSTRIA
	#[serde(rename = "AT")]
	Austria,
	/// AZERBAIJAN
	#[serde(rename = "AZ")]
	Azerbaijan,
	/// BAHAMAS
	#[serde(rename = "BS")]
	Bahamas,
	/// BAHRAIN
	#[serde(rename = "BH")]
	Bahrain,
	/// BANGLADESH
	#[serde(rename = "BD")]
	Bangladesh,
	/// BARBADOS
	#[serde(rename = "BB")]
	Barbados,
	/// BELARUS
	#[serde(rename = "BY")]
	Belarus,
	/// BELGIUM
	#[serde(rename = "BE")]
	Belgium,
	/// BELIZE
	#[serde(rename = "BZ")]
	Belize,
	/// BENIN
	#[serde(rename = "BJ")]
	Benin,
	/// BERMUDA
	#[serde(rename = "BM")]
	Bermuda,
	/// BHUTAN
	#[serde(rename = "BT")]
	Bhutan,
	/// BOLIVIA
	#[serde(rename = "BO")]
	Bolivia,
	/// BOSNIA AND HERZEGOVINA
	#[serde(rename = "BA")]
	BosniaAndHerzegovina,
	/// BOTSWANA
	#[serde(rename = "BW")]
	Botswana,
	/// BOUVET ISLAND
	#[serde(rename = "BV")]
	BouvetIsland,
	/// BRAZIL
	#[serde(rename = "BR")]
	Brazil,
	/// BRITISH INDIAN OCEAN TERRITORY
	#[serde(rename = "IO")]
	BritishIndianOceanTerritory,
	/// BRUNEI DARUSSALAM
	#[serde(rename = "BN")]
	BruneiDarussalam,
	/// BULGARIA
	#[serde(rename = "BG")]
	Bulgaria,
	/// BURKINA FASO
	#[serde(rename = "BF")]
	BurkinaFaso,
	/// BURUNDI
	#[serde(rename = "BI")]
	Burundi,
	/// CAMBODIA
	#[serde(rename = "KH")]
	Cambodia,
	/// CAMEROON
	#[serde(rename = "CM")]
	Cameroon,
	/// CANADA
	#[serde(rename = "CA")]
	Canada,
	/// CAPE VERDE
	#[serde(rename = "CV")]
	CapeVerde,
	/// CAYMAN ISLANDS
	#[serde(rename = "KY")]
	CaymanIslands,
	/// CENTRAL AFRICAN REPUBLIC
	#[serde(rename = "CF")]
	CentralAfricanRepublic,
	/// CHAD
	#[serde(rename = "TD")]
	Chad,
	/// CHILE
	#[serde(rename = "CL")]
	Chile,
	/// CHINA
	#[serde(rename = "CN")]
	China,
	/// CHRISTMAS ISLAND
	#[serde(rename = "CX")]
	ChristmasIsland,
	/// COCOS (KEELING) ISLANDS
	#[serde(rename = "CC")]
	Cocos,
	/// COLOMBIA
	#[serde(rename = "CO")]
	Colombia,
	/// COMOROS
	#[serde(rename = "KM")]
	Comoros,
	/// CONGO
	#[serde(rename = "CG")]
	Congo,
	/// CONGO, THE DEMOCRATIC REPUBLIC OF THE
	#[serde(rename = "CD")]
	DemocraticRepublicOfCongo,
	/// COOK ISLANDS
	#[serde(rename = "CK")]
	CookIslands,
	/// COSTA RICA
	#[serde(rename = "CR")]
	CostaRica,
	/// COTE D'IVOIRE
	#[serde(rename = "CI")]
	CoteDIvoire,
	/// CROATIA
	#[serde(rename = "HR")]
	Croatia,
	/// CUBA
	#[serde(rename = "CU")]
	Cuba,
	/// CYPRUS
	#[serde(rename = "CY")]
	Cyprus,
	/// CZECH REPUBLIC
	#[serde(rename = "CZ")]
	CzechRepublic,
	/// DENMARK
	#[serde(rename = "DK")]
	Denmark,
	/// DJIBOUTI
	#[serde(rename = "DJ")]
	Djibouti,
	/// DOMINICA
	#[serde(rename = "DM")]
	Dominica,
	/// DOMINICAN REPUBLIC
	#[serde(rename = "DO")]
	DominicanRepublic,
	/// ECUADOR
	#[serde(rename = "EC")]
	Ecuador,
	/// EGYPT
	#[serde(rename = "EG")]
	Egypt,
	/// EL SALVADOR
	#[serde(rename = "SV")]
	ElSalvador,
	/// EQUATORIAL GUINEA
	#[serde(rename = "GQ")]
	EquatorialGuinea,
	/// ERITREA
	#[serde(rename = "ER")]
	Eritrea,
	/// ESTONIA
	#[serde(rename = "EE")]
	Estonia,
	/// ETHIOPIA
	#[serde(rename = "ET")]
	Ethiopia,
	/// FALKLAND ISLANDS (MALVINAS)
	#[serde(rename = "FK")]
	FalklandIslands,
	/// FAROE ISLANDS
	#[serde(rename = "FO")]
	FaroeIslands,
	/// FIJI
	#[serde(rename = "FJ")]
	Fiji,
	/// FINLAND
	#[serde(rename = "FI")]
	Finland,
	/// FRANCE
	#[serde(rename = "FR")]
	France,
	/// FRENCH GUIANA
	#[serde(rename = "GF")]
	FrenchGuiana,
	/// FRENCH POLYNESIA
	#[serde(rename = "PF")]
	FrenchPolynesia,
	/// FRENCH SOUTHERN TERRITORIES
	#[serde(rename = "TF")]
	FrenchSouthernTerritories,
	/// GABON
	#[serde(rename = "GA")]
	Gabon,
	/// GAMBIA
	#[serde(rename = "GM")]
	Gambia,
	/// GEORGIA
	#[serde(rename = "GE")]
	Georgia,
	/// GERMANY
	#[serde(rename = "DE")]
	Germany,
	/// GHANA
	#[serde(rename = "GH")]
	Ghana,
	/// GIBRALTAR
	#[serde(rename = "GI")]
	Gibraltar,
	/// GREECE
	#[serde(rename = "GR")]
	Greece,
	/// GREENLAND
	#[serde(rename = "GL")]
	Greenland,
	/// GRENADA
	#[serde(rename = "GD")]
	Grenada,
	/// GUADELOUPE
	#[serde(rename = "GP")]
	Guadeloupe,
	/// GUAM
	#[serde(rename = "GU")]
	Guam,
	/// GUATEMALA
	#[serde(rename = "GT")]
	Guatemala,
	/// GUINEA
	#[serde(rename = "GN")]
	Guinea,
	/// GUINEA-BISSAU
	#[serde(rename = "GW")]
	GuineaBissau,
	/// GUYANA
	#[serde(rename = "GY")]
	Guyana,
	/// HAITI
	#[serde(rename = "HT")]
	Haiti,
	/// HEARD ISLAND AND MCDONALD ISLANDS
	#[serde(rename = "HM")]
	HeardIslandAndMcdonaldIslands,
	/// HOLY SEE (VATICAN CITY STATE)
	#[serde(rename = "VA")]
	HolySee,
	/// HONDURAS
	#[serde(rename = "HN")]
	Honduras,
	/// HONG KONG
	#[serde(rename = "HK")]
	HongKong,
	/// HUNGARY
	#[serde(rename = "HU")]
	Hungary,
	/// ICELAND
	#[serde(rename = "IS")]
	Iceland,
	/// INDIA
	#[serde(rename = "IN")]
	India,
	/// INDONESIA
	#[serde(rename = "ID")]
	Indonesia,
	/// IRAN, ISLAMIC REPUBLIC OF
	#[serde(rename = "IR")]
	Iran,
	/// IRAQ
	#[serde(rename = "IQ")]
	Iraq,
	/// IRELAND
	#[serde(rename = "IE")]
	Ireland,
	/// ISRAEL
	#[serde(rename = "IL")]
	Israel,
	/// ITALY
	#[serde(rename = "IT")]
	Italy,
	/// JAMAICA
	#[serde(rename = "JM")]
	Jamaica,
	/// JAPAN
	#[serde(rename = "JP")]
	Japan,
	/// JORDAN
	#[serde(rename = "JO")]
	Jordan,
	/// KAZAKHSTAN
	#[serde(rename = "KZ")]
	Kazakhstan,
	/// KENYA
	#[serde(rename = "KE")]
	Kenya,
	/// KIRIBATI
	#[serde(rename = "KI")]
	Kiribati,
	/// KOREA, DEMOCRATIC PEOPLE'S REPUBLIC OF
	#[serde(rename = "KP")]
	Korea,
	/// KOREA, REPUBLIC OF
	#[serde(rename = "KR")]
	RepublicOfKorea,
	/// KUWAIT
	#[serde(rename = "KW")]
	Kuwait,
	/// KYRGYZSTAN
	#[serde(rename = "KG")]
	Kyrgyzstan,
	/// LAO PEOPLE'S DEMOCRATIC REPUBLIC
	#[serde(rename = "LA")]
	LaoPeopleSDemocraticRepublic,
	/// LATVIA
	#[serde(rename = "LV")]
	Latvia,
	/// LEBANON
	#[serde(rename = "LB")]
	Lebanon,
	/// LESOTHO
	#[serde(rename = "LS")]
	Lesotho,
	/// LIBERIA
	#[serde(rename = "LR")]
	Liberia,
	/// LIBYAN ARAB JAMAHIRIYA
	#[serde(rename = "LY")]
	LibyanArabJamahiriya,
	/// LIECHTENSTEIN
	#[serde(rename = "LI")]
	Liechtenstein,
	/// LITHUANIA
	#[serde(rename = "LT")]
	Lithuania,
	/// LUXEMBOURG
	#[serde(rename = "LU")]
	Luxembourg,
	/// MACAO
	#[serde(rename = "MO")]
	Macao,
	/// MACEDONIA, THE FORMER YUGOSLAV REPUBLIC OF
	#[serde(rename = "MK")]
	Macedonia,
	/// MADAGASCAR
	#[serde(rename = "MG")]
	Madagascar,
	/// MALAWI
	#[serde(rename = "MW")]
	Malawi,
	/// MALAYSIA
	#[serde(rename = "MY")]
	Malaysia,
	/// MALDIVES
	#[serde(rename = "MV")]
	Maldives,
	/// MALI
	#[serde(rename = "ML")]
	Mali,
	/// MALTA
	#[serde(rename = "MT")]
	Malta,
	/// MARSHALL ISLANDS
	#[serde(rename = "MH")]
	MarshallIslands,
	/// MARTINIQUE
	#[serde(rename = "MQ")]
	Martinique,
	/// MAURITANIA
	#[serde(rename = "MR")]
	Mauritania,
	/// MAURITIUS
	#[serde(rename = "MU")]
	Mauritius,
	/// MAYOTTE
	#[serde(rename = "YT")]
	Mayotte,
	/// MEXICO
	#[serde(rename = "MX")]
	Mexico,
	/// MICRONESIA, FEDERATED STATES OF
	#[serde(rename = "FM")]
	Micronesia,
	/// MOLDOVA, REPUBLIC OF
	#[serde(rename = "MD")]
	Moldova,
	/// MONACO
	#[serde(rename = "MC")]
	Monaco,
	/// MONGOLIA
	#[serde(rename = "MN")]
	Mongolia,
	/// MONTSERRAT
	#[serde(rename = "MS")]
	Montserrat,
	/// MOROCCO
	#[serde(rename = "MA")]
	Morocco,
	/// MOZAMBIQUE
	#[serde(rename = "MZ")]
	Mozambique,
	/// MYANMAR
	#[serde(rename = "MM")]
	Myanmar,
	/// NAMIBIA
	#[serde(rename = "NA")]
	Namibia,
	/// NAURU
	#[serde(rename = "NR")]
	Nauru,
	/// NEPAL
	#[serde(rename = "NP")]
	Nepal,
	/// NETHERLANDS
	#[serde(rename = "NL")]
	Netherlands,
	/// NETHERLANDS ANTILLES
	#[serde(rename = "AN")]
	NetherlandsAntilles,
	/// NEW CALEDONIA
	#[serde(rename = "NC")]
	NewCaledonia,
	/// NEW ZEALAND
	#[serde(rename = "NZ")]
	NewZealand,
	/// NICARAGUA
	#[serde(rename = "NI")]
	Nicaragua,
	/// NIGER
	#[serde(rename = "NE")]
	Niger,
	/// NIGERIA
	#[serde(rename = "NG")]
	Nigeria,
	/// NIUE
	#[serde(rename = "NU")]
	Niue,
	/// NORFOLK ISLAND
	#[serde(rename = "NF")]
	NorfolkIsland,
	/// NORTHERN MARIANA ISLANDS
	#[serde(rename = "MP")]
	NorthernMarianaIslands,
	/// NORWAY
	#[serde(rename = "NO")]
	Norway,
	/// OMAN
	#[serde(rename = "OM")]
	Oman,
	/// PAKISTAN
	#[serde(rename = "PK")]
	Pakistan,
	/// PALAU
	#[serde(rename = "PW")]
	Palau,
	/// PALESTINIAN TERRITORY, OCCUPIED
	#[serde(rename = "PS")]
	PalestinianTerritory,
	/// PANAMA
	#[serde(rename = "PA")]
	Panama,
	/// PAPUA NEW GUINEA
	#[serde(rename = "PG")]
	PapuaNewGuinea,
	/// PARAGUAY
	#[serde(rename = "PY")]
	Paraguay,
	/// PERU
	#[serde(rename = "PE")]
	Peru,
	/// PHILIPPINES
	#[serde(rename = "PH")]
	Philippines,
	/// PITCAIRN
	#[serde(rename = "PN")]
	Pitcairn,
	/// POLAND
	#[serde(rename = "PL")]
	Poland,
	/// PORTUGAL
	#[serde(rename = "PT")]
	Portugal,
	/// PUERTO RICO
	#[serde(rename = "PR")]
	PuertoRico,
	/// QATAR
	#[serde(rename = "QA")]
	Qatar,
	/// REUNION
	#[serde(rename = "RE")]
	Reunion,
	/// ROMANIA
	#[serde(rename = "RO")]
	Romania,
	/// RUSSIAN FEDERATION
	#[serde(rename = "RU")]
	RussianFederation,
	/// RWANDA
	#[serde(rename = "RW")]
	Rwanda,
	/// SAINT HELENA
	#[serde(rename = "SH")]
	SaintHelena,
	/// SAINT KITTS AND NEVIS
	#[serde(rename = "KN")]
	SaintKittsAndNevis,
	/// SAINT LUCIA
	#[serde(rename = "LC")]
	SaintLucia,
	/// SAINT PIERRE AND MIQUELON
	#[serde(rename = "PM")]
	SaintPierreAndMiquelon,
	/// SAINT VINCENT AND THE GRENADINES
	#[serde(rename = "VC")]
	SaintVincentAndTheGrenadines,
	/// SAMOA
	#[serde(rename = "WS")]
	Samoa,
	/// SAN MARINO
	#[serde(rename = "SM")]
	SanMarino,
	/// SAO TOME AND PRINCIPE
	#[serde(rename = "ST")]
	SaoTomeAndPrincipe,
	/// SAUDI ARABIA
	#[serde(rename = "SA")]
	SaudiArabia,
	/// SENEGAL
	#[serde(rename = "SN")]
	Senegal,
	/// SEYCHELLES
	#[serde(rename = "SC")]
	Seychelles,
	/// SIERRA LEONE
	#[serde(rename = "SL")]
	SierraLeone,
	/// SINGAPORE
	#[serde(rename = "SG")]
	Singapore,
	/// SLOVAKIA
	#[serde(rename = "SK")]
	Slovakia,
	/// SLOVENIA
	#[serde(rename = "SI")]
	Slovenia,
	/// SOLOMON ISLANDS
	#[serde(rename = "SB")]
	SolomonIslands,
	/// SOMALIA
	#[serde(rename = "SO")]
	Somalia,
	/// SOUTH AFRICA
	#[serde(rename = "ZA")]
	SouthAfrica,
	/// SOUTH GEORGIA AND THE SOUTH SANDWICH ISLANDS
	#[serde(rename = "GS")]
	SouthGeorgiaAndTheSouthSandwichIslands,
	/// SPAIN
	#[serde(rename = "ES")]
	Spain,
	/// SRI LANKA
	#[serde(rename = "LK")]
	SriLanka,
	/// SUDAN
	#[serde(rename = "SD")]
	Sudan,
	/// SURINAME
	#[serde(rename = "SR")]
	Suriname,
	/// SVALBARD AND JAN MAYEN
	#[serde(rename = "SJ")]
	SvalbardAndJanMayen,
	/// SWAZILAND
	#[serde(rename = "SZ")]
	Swaziland,
	/// SWEDEN
	#[serde(rename = "SE")]
	Sweden,
	/// SWITZERLAND
	#[serde(rename = "CH")]
	Switzerland,
	/// SYRIAN ARAB REPUBLIC
	#[serde(rename = "SY")]
	SyrianArabRepublic,
	/// TAIWAN, PROVINCE OF CHINA
	#[serde(rename = "TW")]
	Taiwan,
	/// TAJIKISTAN
	#[serde(rename = "TJ")]
	Tajikistan,
	/// TANZANIA, UNITED REPUBLIC OF
	#[serde(rename = "TZ")]
	Tanzania,
	/// THAILAND
	#[serde(rename = "TH")]
	Thailand,
	/// TIMOR-LESTE
	#[serde(rename = "TL")]
	TimorLeste,
	/// TOGO
	#[serde(rename = "TG")]
	Togo,
	/// TOKELAU
	#[serde(rename = "TK")]
	Tokelau,
	/// TONGA
	#[serde(rename = "TO")]
	Tonga,
	/// TRINIDAD AND TOBAGO
	#[serde(rename = "TT")]
	TrinidadAndTobago,
	/// TUNISIA
	#[serde(rename = "TN")]
	Tunisia,
	/// TURKEY
	#[serde(rename = "TR")]
	Turkey,
	/// TURKMENISTAN
	#[serde(rename = "TM")]
	Turkmenistan,
	/// TURKS AND CAICOS ISLANDS
	#[serde(rename = "TC")]
	TurksAndCaicosIslands,
	/// TUVALU
	#[serde(rename = "TV")]
	Tuvalu,
	/// UGANDA
	#[serde(rename = "UG")]
	Uganda,
	/// UKRAINE
	#[serde(rename = "UA")]
	Ukraine,
	/// UNITED ARAB EMIRATES
	#[serde(rename = "AE")]
	UnitedArabEmirates,
	/// UNITED KINGDOM
	#[serde(rename = "GB")]
	UnitedKingdom,
	/// UNITED STATES
	#[serde(rename = "US")]
	UnitedStates,
	/// UNITED STATES MINOR OUTLYING ISLANDS
	#[serde(rename = "UM")]
	UnitedStatesMinorOutlyingIslands,
	/// URUGUAY
	#[serde(rename = "UY")]
	Uruguay,
	/// UZBEKISTAN
	#[serde(rename = "UZ")]
	Uzbekistan,
	/// VANUATU
	#[serde(rename = "VU")]
	Vanuatu,
	/// VENEZUELA
	#[serde(rename = "VE")]
	Venezuela,
	/// VIET NAM
	#[serde(rename = "VN")]
	VietNam,
	/// VIRGIN ISLANDS, BRITISH
	#[serde(rename = "VG")]
	VirginIslandsBritish,
	/// VIRGIN ISLANDS, U.S.
	#[serde(rename = "VI")]
	VirginIslandsUS,
	/// WALLIS AND FUTUNA
	#[serde(rename = "WF")]
	WallisAndFutuna,
	/// WESTERN SAHARA
	#[serde(rename = "EH")]
	WesternSahara,
	/// YEMEN
	#[serde(rename = "YE")]
	Yemen,
	/// YUGOSLAVIA
	#[serde(rename = "YU")]
	Yugoslavia,
	/// ZAMBIA
	#[serde(rename = "ZM")]
	Zambia,
	/// ZIMBABWE
	#[serde(rename = "ZW")]
	Zimbabwe,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StrikePriceDeterminationMethod {
	/// Fixed Strike
	#[serde(rename = "1")]
	FixedStrike,
	/// Strike set at expiration to underlying or other value (lookback floating)
	#[serde(rename = "2")]
	StrikeSetAtExpirationToUnderlyingOrOtherValue,
	/// Strike set to average of underlying settlement price across the life of the option
	#[serde(rename = "3")]
	StrikeSetToAverageOfUnderlyingSettlementPriceAcrossTheLifeOfTheOption,
	/// Strike set to optimal value
	#[serde(rename = "4")]
	StrikeSetToOptimalValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StrikePriceBoundaryMethod {
	/// Less than underlying price is in-the-money (ITM)
	#[serde(rename = "1")]
	LessThanUnderlyingPriceIsInTheMoney,
	/// Less than or equal to the underlying price is in-the-money(ITM)
	#[serde(rename = "2")]
	LessThanOrEqualToTheUnderlyingPriceIsInTheMoneyItm,
	/// Equal to the underlying price is in-the-money(ITM)
	#[serde(rename = "3")]
	EqualToTheUnderlyingPriceIsInTheMoneyItm,
	/// Greater than or equal to underlying price is in-the-money(ITM)
	#[serde(rename = "4")]
	GreaterThanOrEqualToUnderlyingPriceIsInTheMoneyItm,
	/// Greater than underlying is in-the-money(ITM)
	#[serde(rename = "5")]
	GreaterThanUnderlyingIsInTheMoneyItm,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingPriceDeterminationMethod {
	/// Regular
	#[serde(rename = "1")]
	Regular,
	/// Special reference
	#[serde(rename = "2")]
	SpecialReference,
	/// Optimal value (Lookback)
	#[serde(rename = "3")]
	OptimalValue,
	/// Average value (Asian option)
	#[serde(rename = "4")]
	AverageValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlowScheduleType {
	/// NERC Eastern Off-Peak
	#[serde(rename = "0")]
	NercEasternOffPeak,
	/// NERC Western Off-Peak
	#[serde(rename = "1")]
	NercWesternOffPeak,
	/// NERC Calendar-All Days in month
	#[serde(rename = "2")]
	NercCalendarAllDaysInMonth,
	/// NERC Eastern Peak
	#[serde(rename = "3")]
	NercEasternPeak,
	/// NERC Western Peak
	#[serde(rename = "4")]
	NercWesternPeak,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
	OneMillionBtu,
	/// Megawatt hours
	#[serde(rename = "MWh")]
	MegawattHours,
	/// Troy Ounces
	#[serde(rename = "oz_tr")]
	TroyOunces,
	/// Metric Tons (aka Tonne)
	#[serde(rename = "t")]
	MetricTonsTonne,
	/// Tons (US)
	#[serde(rename = "tn")]
	Tons,
	/// US Dollars
	#[serde(rename = "USD")]
	UsDollars,
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
	KilowattMinuteElectricalCapacity,
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
	GrossTonsElaboration,
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
	HeatRate,
	/// Mega heat rate. The number of million BTUs required to produce one megawatt hour of electricity, typically 3.41214 million
	/// BTUs per 1 MWh
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
	GbGallon,
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
	UsOunce,
	/// Piece
	#[serde(rename = "pc")]
	Piece,
	/// US Pint
	#[serde(rename = "pt")]
	UsPint,
	/// GB pint
	#[serde(rename = "pt_gb")]
	GbPint,
	/// US Quart
	#[serde(rename = "qt")]
	UsQuart,
	/// GB Quart
	#[serde(rename = "qt_gb")]
	GbQuart,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlMethod {
	/// Election at exercise (The settlement method will be elected at the time of contract exercise)
	#[serde(rename = "E")]
	ElectionAtExercise,
	/// Cash settlement required
	#[serde(rename = "C")]
	CashSettlementRequired,
	/// Physical settlement required
	#[serde(rename = "P")]
	PhysicalSettlementRequired,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ValuationMethod {
	/// premium style
	#[serde(rename = "EQTY")]
	PremiumStyle,
	/// futures style mark-to-market
	#[serde(rename = "FUT")]
	FuturesStyleMarkToMarket,
	/// futures style with an attached cash adjustment
	#[serde(rename = "FUTDA")]
	FuturesStyleWithAnAttachedCashAdjustment,
	/// CDS style collateralization of market to market and coupon
	#[serde(rename = "CDS")]
	CdsStyleCollateralizationOfMarketToMarketAndCoupon,
	/// CDS in delivery - use recovery rate to calculate obligation
	#[serde(rename = "CDSD")]
	CdsInDelivery,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListMethod {
	/// pre-listed only
	#[serde(rename = "0")]
	PreListedOnly,
	/// user requested
	#[serde(rename = "1")]
	UserRequested,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TimeUnit {
	/// Hour
	#[serde(rename = "H")]
	Hour,
	/// Minute
	#[serde(rename = "Min")]
	Minute,
	/// Second
	#[serde(rename = "S")]
	Second,
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
	/// Quarter
	#[serde(rename = "Q")]
	Quarter,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CPProgram {
	/// 3(a)(3)
	#[serde(rename = "1")]
	N3A,
	/// 4(2)
	#[serde(rename = "2")]
	N42,
	/// Other
	#[serde(rename = "99")]
	Other,
	/// 3(a)(2)
	#[serde(rename = "3")]
	N3A2,
	/// 3(a)(3) and 3(c)(7)
	#[serde(rename = "4")]
	N3A3,
	/// 3(a)(4)
	#[serde(rename = "5")]
	N3A4,
	/// 3(a)(5)
	#[serde(rename = "6")]
	N3A5,
	/// 3(a)(7)
	#[serde(rename = "7")]
	N3A7,
	/// 3(c)(7)
	#[serde(rename = "8")]
	N3C7,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ObligationType {
	/// Bond
	#[serde(rename = "0")]
	Bond,
	/// Convertible bond
	#[serde(rename = "1")]
	ConvertibleBond,
	/// Mortgage
	#[serde(rename = "2")]
	Mortgage,
	/// Loan
	#[serde(rename = "3")]
	Loan,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ShortSaleRestriction {
	/// No restrictions
	#[serde(rename = "0")]
	NoRestrictions,
	/// Security not shortable
	#[serde(rename = "1")]
	SecurityNotShortable,
	/// Security not shortable at or below best bid
	#[serde(rename = "2")]
	SecurityNotShortableAtOrBelowBestBid,
	/// Security is not shortable without pre-borrow
	#[serde(rename = "3")]
	SecurityIsNotShortableWithoutPreBorrow,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AssetClass {
	/// Interest rate
	#[serde(rename = "1")]
	InterestRate,
	/// Currency
	#[serde(rename = "2")]
	Currency,
	/// Credit
	#[serde(rename = "3")]
	Credit,
	/// Equity
	#[serde(rename = "4")]
	Equity,
	/// Commodity
	#[serde(rename = "5")]
	Commodity,
	/// Other
	#[serde(rename = "6")]
	Other,
	/// Cash
	#[serde(rename = "7")]
	Cash,
	/// Debt
	#[serde(rename = "8")]
	Debt,
	/// Fund (Such as mutual fund, collective investment vehicle, investment program, specialized account program.)
	#[serde(rename = "9")]
	Fund,
	/// Loan facility
	#[serde(rename = "10")]
	LoanFacility,
	/// Index
	#[serde(rename = "11")]
	Index,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AssetSubClass {
	/// Single currency
	#[serde(rename = "1")]
	SingleCurrency,
	/// Cross currency
	#[serde(rename = "2")]
	CrossCurrency,
	/// Basket [for multi-currency]
	#[serde(rename = "3")]
	Basket,
	/// Single name
	#[serde(rename = "4")]
	SingleName,
	/// Credit index
	#[serde(rename = "5")]
	CreditIndex,
	/// Index tranche
	#[serde(rename = "6")]
	IndexTranche,
	/// Credit basket
	#[serde(rename = "7")]
	CreditBasket,
	/// Exotic
	#[serde(rename = "8")]
	Exotic,
	/// Common
	#[serde(rename = "9")]
	Common,
	/// Preferred
	#[serde(rename = "10")]
	Preferred,
	/// Equity index
	#[serde(rename = "11")]
	EquityIndex,
	/// Equity basket
	#[serde(rename = "12")]
	EquityBasket,
	/// Metals
	#[serde(rename = "13")]
	Metals,
	/// Bullion
	#[serde(rename = "14")]
	Bullion,
	/// Energy
	#[serde(rename = "15")]
	Energy,
	/// Commodity index
	#[serde(rename = "16")]
	CommodityIndex,
	/// Agricultural
	#[serde(rename = "17")]
	Agricultural,
	/// Environmental
	#[serde(rename = "18")]
	Environmental,
	/// Freight
	#[serde(rename = "19")]
	Freight,
	/// Government
	#[serde(rename = "20")]
	Government,
	/// Agency
	#[serde(rename = "21")]
	Agency,
	/// Corporate
	#[serde(rename = "22")]
	Corporate,
	/// Financing
	#[serde(rename = "23")]
	Financing,
	/// Money market
	#[serde(rename = "24")]
	MoneyMarket,
	/// Mortgage
	#[serde(rename = "25")]
	Mortgage,
	/// Municipal
	#[serde(rename = "26")]
	Municipal,
	/// Mutual fund
	#[serde(rename = "27")]
	MutualFund,
	/// Collective investment vehicle
	#[serde(rename = "28")]
	CollectiveInvestmentVehicle,
	/// Investment program (generalized fund for major investors)
	#[serde(rename = "29")]
	InvestmentProgram,
	/// Specialized account (specialized fund setup for a particular account or group of accounts.)
	#[serde(rename = "30")]
	SpecializedAccount,
	/// Term loan
	#[serde(rename = "31")]
	TermLoan,
	/// Bridge loan
	#[serde(rename = "32")]
	BridgeLoan,
	/// Letter of credit
	#[serde(rename = "33")]
	LetterOfCredit,
	/// Dividend index
	#[serde(rename = "34")]
	DividendIndex,
	/// Stock dividend
	#[serde(rename = "35")]
	StockDividend,
	/// Exchange traded fund
	#[serde(rename = "36")]
	ExchangeTradedFund,
	/// Volatility index
	#[serde(rename = "37")]
	VolatilityIndex,
	/// FX cross rates
	#[serde(rename = "38")]
	FxCrossRates,
	/// FX emerging markets
	#[serde(rename = "39")]
	FxEmergingMarkets,
	/// FX Majors
	#[serde(rename = "40")]
	FxMajors,
	/// Fertilizer
	#[serde(rename = "41")]
	Fertilizer,
	/// Industrial product
	#[serde(rename = "42")]
	IndustrialProduct,
	/// Inflation
	#[serde(rename = "43")]
	Inflation,
	/// Paper
	#[serde(rename = "44")]
	Paper,
	/// Polypropylene
	#[serde(rename = "45")]
	Polypropylene,
	/// Official economic statistics
	#[serde(rename = "46")]
	OfficialEconomicStatistics,
	/// Other C10
	#[serde(rename = "47")]
	OtherC10,
	/// Other
	#[serde(rename = "48")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SwapClass {
	/// Basis swap
	#[serde(rename = "BS")]
	BasisSwap,
	/// Index swap
	#[serde(rename = "IX")]
	IndexSwap,
	/// Broad-based security swap
	#[serde(rename = "BB")]
	BroadBasedSecuritySwap,
	/// Basket swap
	#[serde(rename = "SK")]
	BasketSwap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouponType {
	/// Zero
	#[serde(rename = "0")]
	Zero,
	/// Fixed rate
	#[serde(rename = "1")]
	FixedRate,
	/// Floating rate
	#[serde(rename = "2")]
	FloatingRate,
	/// Structured
	#[serde(rename = "3")]
	Structured,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouponFrequencyUnit {
	/// Day
	#[serde(rename = "D")]
	Day,
	/// Week
	#[serde(rename = "Wk")]
	Week,
	/// Month
	#[serde(rename = "Mo")]
	Month,
	/// Year
	#[serde(rename = "Yr")]
	Year,
	/// Hour
	#[serde(rename = "H")]
	Hour,
	/// Minute
	#[serde(rename = "Min")]
	Minute,
	/// Second
	#[serde(rename = "S")]
	Second,
	/// Term
	#[serde(rename = "T")]
	Term,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouponDayCount {
	/// 1/1 (If parties specify the Day Count Fraction to be 1/1 then in calculating the applicable amount, 1 is simply input into
	/// the calculation as the relevant Day Count Fraction.)
	#[serde(rename = "0")]
	N11,
	/// 30/360 (30U/360 or Bond Basis) (Mainly used in the US with the following date adjustment rules: (1) If the investment is End-Of-Month
	/// and Date1 is the last day of February and Date2 is the last day of February, then change Date2 to 30; (2) If the investment
	/// is End-Of-Month and Date1 is the last day of February, then change Date1 to 30; (3) If Date2 is 31 and Date1 is 30 or 31,
	/// then change Date2 to 30; (4) If Date1 is 31, then change Date1 to 30.)
	#[serde(rename = "1")]
	N30360,
	/// 30/360 (SIA) (A variant of "30/360" - when Date1 and Date2 are both Feb. 28th or 29th convert them to 30th using the same
	/// logic in the conversion of 31st to 30th.)
	#[serde(rename = "2")]
	N30360Sia,
	/// 30/360M (Commonly used day count convention for US mortgage backed securities. Feb 28th (or 29th in a leap year) is always
	/// considered as a 30th for a start date. As a comparison, in the regular 30/360 day count as used by most US agency and corporate
	/// bonds, a start date of Feb 28th (or 29th in a leap year) is still considered as the 28th (or 29th) day of a month of 30 days.)
	#[serde(rename = "3")]
	N30360M,
	/// 30E/360 (Eurobond Basis) (Also known as 30/360.ISMA, 30S/360, or Special German. Date adjustment rules are: (1) If Date1 falls
	/// on the 31st, then change it to the 30th; (2) If Date2 falls on the 31st, then change it to the 30th.)
	#[serde(rename = "4")]
	N30E360,
	/// 30E/360 (ISDA) (Date adjustment rules are: (1) if Date1 is the last day of the month, then change Date1 to 30; (2) if D2 is
	/// the last day of the month (unless Date2 is the maturity date and Date2 is in February), then change Date2 to 30.)
	#[serde(rename = "5")]
	N30E360Isda,
	/// Act/360 (The actual number of days between Date1 and Date2, divided by 360.)
	#[serde(rename = "6")]
	Act360,
	/// Act/365 (FIXED) (The actual number of days between Date1 and Date2, divided by 365.)
	#[serde(rename = "7")]
	Act365,
	/// Act/Act (AFB) (The actual number of days between Date1 and Date2, the denominator is either 365 (if the calculation period
	/// does not contain the 29th February) or 366 (if the calculation period includes 29th February).)
	#[serde(rename = "8")]
	ActAct,
	/// Act/Act ICMA (The denominator is the actual number of days in the coupon period multiplied by the number of coupon periods
	/// in the year. Assumes that regular coupons alwaysfall on the same day of the month where possible.)
	#[serde(rename = "9")]
	ActActIcmaUltimo,
	/// Act/Act (ICMA Ultimo) (The Act/Act (ICMA Ultimo) differs from Act/Act (ICMA) method only that it assumes that regular coupons
	/// always fall on the last day of the month.)
	#[serde(rename = "10")]
	ActActIcma,
	/// Act/Act ISDA (The denominator varies depending on whether a portion of the relevant calculation period falls within a leap
	/// year. For the portion of the calculation period falling in a leap year, the denominator is 366 and for the portion falling
	/// outside a leap year, the denominator is 365.)
	#[serde(rename = "11")]
	ActActIsda,
	/// BUS/252 (Used for Brazilian Real swaps which is based on business days instead of calendar days. The number of business days
	/// divied by 252)
	#[serde(rename = "12")]
	Bus252,
	/// 30E+/360 (Variation on 30E/360. Date adjustment rules: (1) If Date1 falls on the 31st, then change it to the 30th; (2) If
	/// Date2 falls on the 31st, then change it to 1 and increase Month2 by one, i.e. next month)
	#[serde(rename = "13")]
	N30E,
	/// Act/365L (The number of days in a period equal to the actual number of days. The number of days in a year is 365, or if the
	/// period ends in a leap year 366. Used for Sterling floating rate notes. May also be referred to as ISMA-Year.)
	#[serde(rename = "14")]
	Act365L,
	/// NL365 (The number of days in a period equal to the actual number of days, with the exception of leap days (29th February)
	/// which are ignored. The number of days in a year is 365, even in a leap year.)
	#[serde(rename = "15")]
	Nl365,
	/// NL360 (This is the same as Act/360, with the exception of leap days (29th February) which are ignored.)
	#[serde(rename = "16")]
	Nl360,
	/// Act/364 (The actual number of days between Date1 and Date2, divided by 364.)
	#[serde(rename = "17")]
	Act364,
	/// 30/365
	#[serde(rename = "18")]
	N30365,
	/// 30/Actual
	#[serde(rename = "19")]
	N30Actual,
	/// 30/360 (ICMA or basis rule)
	#[serde(rename = "20")]
	N30360Icma,
	/// 30E2/360 (Eurobond basis model two)
	#[serde(rename = "21")]
	N30E2360,
	/// 30E3/360 (Eurobond basis model three)
	#[serde(rename = "22")]
	N30E3360,
	/// Other
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConvertibleBondEquityIDSource {
	/// CUSIP
	#[serde(rename = "1")]
	Cusip,
	/// SEDOL
	#[serde(rename = "2")]
	Sedol,
	/// QUIK
	#[serde(rename = "3")]
	Quik,
	/// ISIN
	#[serde(rename = "4")]
	Isin,
	/// RIC
	#[serde(rename = "5")]
	Ric,
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
	ConsolidatedTapeAssociation,
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
	IsdaFpMlProductSpecification,
	/// Option Price Reporting Authority
	#[serde(rename = "J")]
	OptionPriceReportingAuthority,
	/// ISDA/FpML Product URL (URL in SecurityID)
	#[serde(rename = "K")]
	IsdaFpMlProductUrl,
	/// Letter of Credit
	#[serde(rename = "L")]
	LetterOfCredit,
	/// Marketplace-assigned Identifier
	#[serde(rename = "M")]
	MarketplaceAssignedIdentifier,
	/// Markit RED entity CLIP
	#[serde(rename = "N")]
	MarkitRedEntityClip,
	/// Markit RED pair CLIP
	#[serde(rename = "P")]
	MarkitRedPairClip,
	/// CFTC commodity code
	#[serde(rename = "Q")]
	CftcCommodityCode,
	/// ISDA Commodity Reference Price
	#[serde(rename = "R")]
	IsdaCommodityReferencePrice,
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
	FidessaInstrumentMnemonic,
	/// Index name
	#[serde(rename = "W")]
	IndexName,
	/// Uniform Symbol (UMTF Symbol)
	#[serde(rename = "X")]
	UniformSymbol,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LienSeniority {
	/// Unknown
	#[serde(rename = "0")]
	Unknown,
	/// First lien
	#[serde(rename = "1")]
	FirstLien,
	/// Second lien
	#[serde(rename = "2")]
	SecondLien,
	/// Third lien
	#[serde(rename = "3")]
	ThirdLien,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LoanFacility {
	/// Bridge loan
	#[serde(rename = "0")]
	BridgeLoan,
	/// Letter of credit
	#[serde(rename = "1")]
	LetterOfCredit,
	/// Revolving loan
	#[serde(rename = "2")]
	RevolvingLoan,
	/// Swingline funding
	#[serde(rename = "3")]
	SwinglineFunding,
	/// Term loan
	#[serde(rename = "4")]
	TermLoan,
	/// Trade claim
	#[serde(rename = "5")]
	TradeClaim,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReferenceEntityType {
	/// Asian
	#[serde(rename = "1")]
	Asian,
	/// Australian and New Zealand
	#[serde(rename = "2")]
	AustralianAndNewZealand,
	/// European emerging markets
	#[serde(rename = "3")]
	EuropeanEmergingMarkets,
	/// Japanese
	#[serde(rename = "4")]
	Japanese,
	/// North American high yield
	#[serde(rename = "5")]
	NorthAmericanHighYield,
	/// North American insurance
	#[serde(rename = "6")]
	NorthAmericanInsurance,
	/// North American investment grade
	#[serde(rename = "7")]
	NorthAmericanInvestmentGrade,
	/// Singaporean
	#[serde(rename = "8")]
	Singaporean,
	/// Western European
	#[serde(rename = "9")]
	WesternEuropean,
	/// Western European insurance
	#[serde(rename = "10")]
	WesternEuropeanInsurance,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SwapSubClass {
	/// Amortizing notional schedule
	#[serde(rename = "AMTZ")]
	AmortizingNotionalSchedule,
	/// Compounding
	#[serde(rename = "COMP")]
	Compounding,
	/// Constant notional schedule
	#[serde(rename = "CNST")]
	ConstantNotionalSchedule,
	/// Accreting notional schedule
	#[serde(rename = "ACRT")]
	AccretingNotionalSchedule,
	/// Custom notional schedule
	#[serde(rename = "CUST")]
	CustomNotionalSchedule,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StrategyType {
	/// Straddle
	#[serde(rename = "STD")]
	Straddle,
	/// Strangle
	#[serde(rename = "STG")]
	Strangle,
	/// Butterfly
	#[serde(rename = "BF")]
	Butterfly,
	/// Condor
	#[serde(rename = "CNDR")]
	Condor,
	/// Callable inversible snowball
	#[serde(rename = "CISN")]
	CallableInversibleSnowball,
	/// Other
	#[serde(rename = "OTHER")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlDisruptionProvision {
	/// Negotiation
	#[serde(rename = "1")]
	Negotiation,
	/// Cancellation and payment
	#[serde(rename = "2")]
	CancellationAndPayment,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InstrumentRoundingDirection {
	/// Round to nearest
	#[serde(rename = "0")]
	RoundToNearest,
	/// Round down
	#[serde(rename = "1")]
	RoundDown,
	/// Round up
	#[serde(rename = "2")]
	RoundUp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AssetGroup {
	/// Financials (A categorization which usually includes rates, foreign exchange, credit, bonds and equity products or assets)
	#[serde(rename = "1")]
	Financials,
	/// Commodities (A categorization which usually includes hard commodities such as agricultural, metals, freight, energy products
	/// or assets.)
	#[serde(rename = "2")]
	Commodities,
	/// Alternative investments (A categorization which usually includes weather, housing, and commodity indices products or assets.)
	#[serde(rename = "3")]
	AlternativeInvestments,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SettlSubMethod {
	/// Shares
	#[serde(rename = "1")]
	Shares,
	/// Derivatives
	#[serde(rename = "2")]
	Derivatives,
	/// Payment vs payment
	#[serde(rename = "3")]
	PaymentVsPayment,
	/// Notional
	#[serde(rename = "4")]
	Notional,
	/// Cascade
	#[serde(rename = "5")]
	Cascade,
	/// Repurchase
	#[serde(rename = "6")]
	Repurchase,
	/// Other
	#[serde(rename = "99")]
	Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StrikeIndexQuote {
	/// Bid
	#[serde(rename = "0")]
	Bid,
	/// Mid
	#[serde(rename = "1")]
	Mid,
	/// Offer
	#[serde(rename = "2")]
	Offer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExtraordinaryEventAdjustmentMethod {
	/// Calculation agent
	#[serde(rename = "0")]
	CalculationAgent,
	/// Options exchange
	#[serde(rename = "1")]
	OptionsExchange,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InTheMoneyCondition {
	/// Standard in-the-money
	#[serde(rename = "0")]
	StandardInTheMoney,
	/// At-the-money is in-the-money
	#[serde(rename = "1")]
	AtTheMoneyIsInTheMoney,
	/// At-the-money call is in-the-money
	#[serde(rename = "2")]
	AtTheMoneyCallIsInTheMoney,
	/// At-the-money put is in-the-money
	#[serde(rename = "3")]
	AtTheMoneyPutIsInTheMoney,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReturnTrigger {
	/// Dividend
	#[serde(rename = "1")]
	Dividend,
	/// Variance
	#[serde(rename = "2")]
	Variance,
	/// Volatility
	#[serde(rename = "3")]
	Volatility,
	/// Total return
	#[serde(rename = "4")]
	TotalReturn,
	/// Contract for difference
	#[serde(rename = "5")]
	ContractForDifference,
	/// Credit default
	#[serde(rename = "6")]
	CreditDefault,
	/// Spread bet
	#[serde(rename = "7")]
	SpreadBet,
	/// Price
	#[serde(rename = "8")]
	Price,
	/// Forward price of underlying instrument
	#[serde(rename = "9")]
	ForwardPriceOfUnderlyingInstrument,
	/// Other
	#[serde(rename = "99")]
	Other,
}
