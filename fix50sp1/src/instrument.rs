
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Instrument {
	/// Common, "human understood" representation of the security. <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> value can be specified if no symbol exists (e.g. non-exchange traded Collective Investment Vehicles). Use "[N/A]" for products
	/// which do not have a symbol.
	#[serde(rename = "55")]
	pub symbol: String,
	/// Used in Fixed Income with a value of "WI" to indicate "When Issued" for a security to be reissued under an old CUSIP or ISIN
	/// or with a value of "CD" to indicate a EUCP with lump-sum interest rather than discount price.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "65")]
	pub symbol_sfx: Option<String>,
	/// Takes precedence in identifying security to counterparty over <a href="tag_455_SecurityAltID.html" target="bottom">SecurityAltID&nbsp;(455)</a> block. Requires <a href="tag_22_SecurityIDSource.html" target="bottom">SecurityIDSource&nbsp;(22)</a> if specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "48")]
	pub security_id: Option<String>,
	/// Required if <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "22")]
	pub security_id_source: Option<SecurityIDSource>,
	/// NoSecurityAltID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "454")]
	pub security_alt_id: Option<fix_common::RepeatingValues<SecurityAltI>>,
	/// Indicates the type of product the security is associated with (high-level category)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "460")]
	pub product: Option<Product>,
	/// Identifies an entire suite of products for a given market. In Futures this may be "interest rates", "agricultural", "equity
	/// indexes", etc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1227")]
	pub product_complex: Option<String>,
	/// An exchange specific name assigned to a group of related securities which may be concurrently affected by market events and
	/// actions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1151")]
	pub security_group: Option<String>,
	/// Indicates the type of security using ISO 10962 standard, Classification of Financial Instruments (CFI code) values. It is
	/// recommended that <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> be used instead of <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> for non-Fixed Income instruments.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "461")]
	pub cfi_code: Option<String>,
	/// It is recommended that <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> be used instead of <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> for non-Fixed Income instruments. Required for Fixed Income. Refer to Volume 7 - Fixed Income Futures and Options should be
	/// specified using the <a href="tag_461_CFICode.html" target="bottom">CFICode&nbsp;(461)</a> field instead of <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> (Refer to Volume 7 - Recommendations and Guidelines for Futures and Options Markets.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "167")]
	pub security_type: Option<SecurityType>,
	/// Sub-type qualification/identification of the <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> (e.g. for SecurityType="MLEG"). If specified, <a href="tag_167_SecurityType.html" target="bottom">SecurityType&nbsp;(167)</a> is required.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "762")]
	pub security_sub_type: Option<String>,
	/// Specifies the month and year of maturity. Applicable for standardized derivatives which are typically only referenced b y
	/// month and year (e.g. S&amp;P futures). Note <a href="tag_541_MaturityDate.html" target="bottom">MaturityDate&nbsp;(541)</a> (a full date) can also be specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "200")]
	pub maturity_month_year: Option<fix_common::MonthYear>,
	/// Specifies date of maturity (a full date). Note that standardized derivatives which are typically only referenced by month
	/// and year (e.g. S&amp;P futures).may use <a href="tag_200_MaturityMonthYear.html" target="bottom">MaturityMonthYear&nbsp;(200)</a> and/or this field. When using MaturityMonthYear, it is recommended that markets and sell sides report the <a href="tag_541_MaturityDate.html" target="bottom">MaturityDate&nbsp;(541)</a> on all outbound messages as a means of data enrichment.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "541")]
	pub maturity_date: Option<fix_common::LocalMktDate>,
	/// MaturityTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1079")]
	pub maturity_time: Option<fix_common::TZTimeOnly>,
	/// Indicator to determine if <a href="block_Instrument.html" target="main">Instrument</a> is Settle on Open.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "966")]
	pub settle_on_open_flag: Option<String>,
	/// InstrmtAssignmentMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1049")]
	pub instrmt_assignment_method: Option<InstrmtAssignmentMethod>,
	/// Gives the current state of the instrument
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "965")]
	pub security_status: Option<SecurityStatus>,
	/// Date interest is to be paid. Used in identifying Corporate Bond issues.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "224")]
	pub coupon_payment_date: Option<fix_common::LocalMktDate>,
	/// Date instrument was issued. For Fixed Income IOIs for new issues, specifies the issue date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "225")]
	pub issue_date: Option<fix_common::LocalMktDate>,
	/// (Deprecated in FIX.4.4)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "239")]
	pub repo_collateral_security_type: Option<RepoCollateralSecurityType>,
	/// (Deprecated in FIX.4.4)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "226")]
	pub repurchase_term: Option<i32>,
	/// (Deprecated in FIX.4.4)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "227")]
	pub repurchase_rate: Option<f32>,
	/// For Fixed Income: Amortization Factor for deriving Current face from Original face for ABS or MBS securities, note the fraction
	/// may be greater than, equal to or less than 1. In TIPS securities this is the Inflation index. Qty * Factor * Price = Gross
	/// Trade Amount. For Derivatives: Contract Value Factor by which price must be adjusted to determine the true nominal value of
	/// one futures/options contract. (Qty * Price) * Factor = Nominal Value.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "228")]
	pub factor: Option<f64>,
	/// CreditRating
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "255")]
	pub credit_rating: Option<String>,
	/// The location at which records of ownership are maintained for this instrument, and at which ownership changes must be recorded.
	/// Can be used in conjunction with ISIN to address ISIN uniqueness issues.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "543")]
	pub instr_registry: Option<String>,
	/// ISO Country code of instrument issue (e.g. the country portion typically used in ISIN). Can be used in conjunction with non-ISIN <a href="tag_48_SecurityID.html" target="bottom">SecurityID&nbsp;(48)</a> (e.g. CUSIP for Municipal Bonds without ISIN) to provide uniqueness.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "470")]
	pub country_of_issue: Option<CountryOfIssue>,
	/// A two-character state or province abbreviation.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "471")]
	pub state_or_province_of_issue: Option<String>,
	/// The three-character IATA code for a locale (e.g. airport code for Municipal Bonds).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "472")]
	pub locale_of_issue: Option<String>,
	/// (Deprecated in FIX.4.4)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "240")]
	pub redemption_date: Option<fix_common::LocalMktDate>,
	/// Used for derivatives, such as options and covered warrants
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "202")]
	pub strike_price: Option<f64>,
	/// Used for derivatives
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "947")]
	pub strike_currency: Option<StrikeCurrency>,
	/// Used for derivatives. Multiplier applied to the strike price for the purpose of calculating the settlement value.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "967")]
	pub strike_multiplier: Option<f64>,
	/// Used for derivatives. The number of shares/units for the financial instrument involved in the option trade.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "968")]
	pub strike_value: Option<f64>,
	/// Used for derivatives, such as options and covered warrants to indicate a versioning of the contract when required due to corporate
	/// actions to the underlying. Should not be used to indicate type of option - use the CFICode[461] for this purpose.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "206")]
	pub opt_attribute: Option<char>,
	/// For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g.
	/// contracts vs. shares) amount.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "231")]
	pub contract_multiplier: Option<f64>,
	/// Minimum price increment for the instrument. Could also be used to represent tick value.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "969")]
	pub min_price_increment: Option<f64>,
	/// Minimum price increment amount associated with the <a href="tag_969_MinPriceIncrement.html" target="bottom">MinPriceIncrement&nbsp;(969)</a> . For listed derivatives, the value can be calculated by multiplying MinPriceIncrement by <a href="tag_231_ContractMultiplier.html" target="bottom">ContractValueFactor&nbsp;(231)</a>
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1146")]
	pub min_price_increment_amount: Option<f64>,
	/// Used to indicate the size of the underlying commodity on which the contract is based (e.g., 2500 lbs of lean cattle, 1000
	/// barrels of crude oil, 1000 bushels of corn, etc.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "996")]
	pub unit_of_measure: Option<UnitOfMeasure>,
	/// UnitOfMeasureQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1147")]
	pub unit_of_measure_qty: Option<f64>,
	/// PriceUnitOfMeasure
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1191")]
	pub price_unit_of_measure: Option<PriceUnitOfMeasure>,
	/// PriceUnitOfMeasureQty
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1192")]
	pub price_unit_of_measure_qty: Option<f64>,
	/// Settlement method for a contract. Can be used as an alternative to CFI Code value
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1193")]
	pub settl_method: Option<SettlMethod>,
	/// Type of exercise of a derivatives security
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1194")]
	pub exercise_style: Option<ExerciseStyle>,
	/// Cash amount indicating the pay out associated with an option. For binary options this is a fixed amount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1195")]
	pub opt_pay_amount: Option<f64>,
	/// Method for price quotation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1196")]
	pub price_quote_method: Option<PriceQuoteMethod>,
	/// For futures, indicates type of valuation method applied
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1197")]
	pub futures_valuation_method: Option<FuturesValuationMethod>,
	/// Indicates whether the instruments are pre-listed only or can also be defined via user request
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1198")]
	pub list_method: Option<ListMethod>,
	/// Used to express the ceiling price of a capped call
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1199")]
	pub cap_price: Option<f64>,
	/// Used to express the floor price of a capped put
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "1200")]
	pub floor_price: Option<f64>,
	/// Used to express option right
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "201")]
	pub put_or_call: Option<PutOrCall>,
	/// Used to indicate if a security has been defined as flexible according to "non-standard" means. Analog to CFICode Standard/Non-standard
	/// indicator
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1244")]
	pub flexible_indicator: Option<fix_common::Boolean>,
	/// Used to indicate if a product or group of product supports the creation of flexible securities
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1242")]
	pub flex_product_eligibility_indicator: Option<fix_common::Boolean>,
	/// Used to indicate a time unit for the contract (e.g., days, weeks, months, etc.)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "997")]
	pub time_unit: Option<TimeUnit>,
	/// For Fixed Income.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "223")]
	pub coupon_rate: Option<f32>,
	/// Can be used to identify the security.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "207")]
	pub security_exchange: Option<String>,
	/// Position Limit for the instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "970")]
	pub position_limit: Option<i32>,
	/// Near-term Position Limit for the instrument.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "971")]
	pub nt_position_limit: Option<i32>,
	/// Issuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "106")]
	pub issuer: Option<String>,
	/// Must be set if <a href="tag_349_EncodedIssuer.html" target="bottom">EncodedIssuer&nbsp;(349)</a> field is specified and must immediately precede it.
	#[serde(rename = "348")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_106_Issuer.html" target="bottom">Issuer&nbsp;(106)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "349")]
	pub encoded_issuer: Option<fix_common::EncodedText<349>>,
	/// SecurityDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "107")]
	pub security_desc: Option<String>,
	/// Must be set if <a href="tag_351_EncodedSecurityDesc.html" target="bottom">EncodedSecurityDesc&nbsp;(351)</a> field is specified and must immediately precede it.
	#[serde(rename = "350")]
	/// Encoded (non-ASCII characters) representation of the <a href="tag_107_SecurityDesc.html" target="bottom">SecurityDesc&nbsp;(107)</a> field in the encoded format specified via the <a href="tag_347_MessageEncoding.html" target="bottom">MessageEncoding&nbsp;(347)</a> field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "351")]
	pub encoded_security_desc: Option<fix_common::EncodedText<351>>,
	/// Embedded XML document describing security.
	#[serde(flatten)]
	pub security_xml: Option<super::security_xml::SecurityXML>,
	/// Identifies MBS / ABS pool
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "691")]
	pub pool: Option<String>,
	/// Must be present for MBS/TBA
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "667")]
	pub contract_settl_month: Option<fix_common::MonthYear>,
	/// The program under which a commercial paper is issued
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "875")]
	pub cp_program: Option<CPProgram>,
	/// The registration type of a commercial paper issuance
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "876")]
	pub cp_reg_type: Option<String>,
	/// NoEvents
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "864")]
	pub events: Option<fix_common::RepeatingValues<Event>>,
	/// If different from IssueDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "873")]
	pub dated_date: Option<fix_common::LocalMktDate>,
	/// If different from <a href="tag_225_IssueDate.html" target="bottom">IssueDate&nbsp;(225)</a> and DatedDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "874")]
	pub interest_accrual_date: Option<fix_common::LocalMktDate>,
	/// Used to identify the parties listing a specific instrument
	#[serde(flatten)]
	pub instrument_parties: Option<super::instrument_parties::InstrumentParties>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityAltI {
	/// SecurityAltID
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "455")]
	pub security_alt_id: Option<String>,
	/// SecurityAltIDSource
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "456")]
	pub security_alt_id_source: Option<SecurityAltIDSource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Event {
	/// EventType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "865")]
	pub event_type: Option<EventType>,
	/// EventDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "866")]
	pub event_date: Option<fix_common::LocalMktDate>,
	/// EventPx
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "867")]
	pub event_px: Option<f64>,
	/// EventText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "868")]
	pub event_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
	ClearingHouseClearingOrganization,
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
}

impl Default for SecurityIDSource {
	fn default() -> Self {
		SecurityIDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

impl Default for Product {
	fn default() -> Self {
		Product::Agency
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityType {
	/// Future
	#[serde(rename = "FUT")]
	Fut,
	/// Option
	#[serde(rename = "OPT")]
	Opt,
	/// US Treasury Note (Deprecated Value Use TNOTE)
	#[serde(rename = "UST")]
	Ust,
	/// US Treasury Bill (Deprecated Value Use TBILL)
	#[serde(rename = "USTB")]
	Ustb,
	/// Euro Supranational Coupons *
	#[serde(rename = "EUSUPRA")]
	Eusupra,
	/// Federal Agency Coupon
	#[serde(rename = "FAC")]
	Fac,
	/// Federal Agency Discount Note
	#[serde(rename = "FADN")]
	Fadn,
	/// Private Export Funding *
	#[serde(rename = "PEF")]
	Pef,
	/// USD Supranational Coupons *
	#[serde(rename = "SUPRA")]
	Supra,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	Corp,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	Cpp,
	/// Convertible Bond
	#[serde(rename = "CB")]
	Cb,
	/// Dual Currency
	#[serde(rename = "DUAL")]
	Dual,
	/// Euro Corporate Bond
	#[serde(rename = "EUCORP")]
	Eucorp,
	/// Indexed Linked
	#[serde(rename = "XLINKD")]
	Xlinkd,
	/// Structured Notes
	#[serde(rename = "STRUCT")]
	Struct,
	/// Yankee Corporate Bond
	#[serde(rename = "YANK")]
	Yank,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	For,
	/// Common Stock
	#[serde(rename = "CS")]
	Cs,
	/// Preferred Stock
	#[serde(rename = "PS")]
	Ps,
	/// Repurchase
	#[serde(rename = "REPO")]
	Repo,
	/// Forward
	#[serde(rename = "FORWARD")]
	Forward,
	/// Buy Sellback
	#[serde(rename = "BUYSELL")]
	Buysell,
	/// Securities Loan
	#[serde(rename = "SECLOAN")]
	Secloan,
	/// Securities Pledge
	#[serde(rename = "SECPLEDGE")]
	Secpledge,
	/// Brady Bond
	#[serde(rename = "BRADY")]
	Brady,
	/// Euro Sovereigns *
	#[serde(rename = "EUSOV")]
	Eusov,
	/// US Treasury Bond
	#[serde(rename = "TBOND")]
	Tbond,
	/// Interest Strip From Any Bond Or Note
	#[serde(rename = "TINT")]
	Tint,
	/// Treasury Inflation Protected Securities
	#[serde(rename = "TIPS")]
	Tips,
	/// Principal Strip Of A Callable Bond Or Note
	#[serde(rename = "TCAL")]
	Tcal,
	/// Principal Strip From A Non-Callable Bond Or Note
	#[serde(rename = "TPRN")]
	Tprn,
	/// US Treasury Note
	#[serde(rename = "TNOTE")]
	Tnote,
	/// US Treasury Bill
	#[serde(rename = "TBILL")]
	Tbill,
	/// Term Loan
	#[serde(rename = "TERM")]
	Term,
	/// Revolver Loan
	#[serde(rename = "RVLV")]
	Rvlv,
	/// Revolver/Term Loan
	#[serde(rename = "RVLVTRM")]
	Rvlvtrm,
	/// Bridge Loan
	#[serde(rename = "BRIDGE")]
	Bridge,
	/// Letter Of Credit
	#[serde(rename = "LOFC")]
	Lofc,
	/// Swing Line Facility
	#[serde(rename = "SWING")]
	Swing,
	/// Debtor In Possession
	#[serde(rename = "DINP")]
	Dinp,
	/// Defaulted
	#[serde(rename = "DEFLTED")]
	Deflted,
	/// Withdrawn
	#[serde(rename = "WITHDRN")]
	Withdrn,
	/// Replaced
	#[serde(rename = "REPLACD")]
	Replacd,
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
	Ba,
	/// Bank Notes
	#[serde(rename = "BN")]
	Bn,
	/// Bill Of Exchanges
	#[serde(rename = "BOX")]
	Box,
	/// Certificate Of Deposit
	#[serde(rename = "CD")]
	Cd,
	/// Call Loans
	#[serde(rename = "CL")]
	Cl,
	/// Commercial Paper
	#[serde(rename = "CP")]
	Cp,
	/// Deposit Notes
	#[serde(rename = "DN")]
	Dn,
	/// Euro Certificate Of Deposit
	#[serde(rename = "EUCD")]
	Eucd,
	/// Euro Commercial Paper
	#[serde(rename = "EUCP")]
	Eucp,
	/// Liquidity Note
	#[serde(rename = "LQN")]
	Lqn,
	/// Medium Term Notes
	#[serde(rename = "MTN")]
	Mtn,
	/// Overnight
	#[serde(rename = "ONITE")]
	Onite,
	/// Promissory Note
	#[serde(rename = "PN")]
	Pn,
	/// Plazos Fijos
	#[serde(rename = "PZFJ")]
	Pzfj,
	/// Short Term Loan Note
	#[serde(rename = "STN")]
	Stn,
	/// Time Deposit
	#[serde(rename = "TD")]
	Td,
	/// Extended Comm Note
	#[serde(rename = "XCN")]
	Xcn,
	/// Yankee Certificate Of Deposit
	#[serde(rename = "YCD")]
	Ycd,
	/// Asset-backed Securities
	#[serde(rename = "ABS")]
	Abs,
	/// Corp. Mortgage-backed Securities
	#[serde(rename = "CMBS")]
	Cmbs,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	Cmo,
	/// IOETTE Mortgage
	#[serde(rename = "IET")]
	Iet,
	/// Mortgage-backed Securities
	#[serde(rename = "MBS")]
	Mbs,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	Mio,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	Mpo,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	Mpp,
	/// Miscellaneous Pass-through
	#[serde(rename = "MPT")]
	Mpt,
	/// Pfandbriefe *
	#[serde(rename = "PFAND")]
	Pfand,
	/// To Be Announced
	#[serde(rename = "TBA")]
	Tba,
	/// Other Anticipation Notes (BAN, GAN, etc.)
	#[serde(rename = "AN")]
	An,
	/// Certificate Of Obligation
	#[serde(rename = "COFO")]
	Cofo,
	/// Certificate Of Participation
	#[serde(rename = "COFP")]
	Cofp,
	/// General Obligation Bonds
	#[serde(rename = "GO")]
	Go,
	/// Mandatory Tender
	#[serde(rename = "MT")]
	Mt,
	/// Revenue Anticipation Note
	#[serde(rename = "RAN")]
	Ran,
	/// Revenue Bonds
	#[serde(rename = "REV")]
	Rev,
	/// Special Assessment
	#[serde(rename = "SPCLA")]
	Spcla,
	/// Special Obligation
	#[serde(rename = "SPCLO")]
	Spclo,
	/// Special Tax
	#[serde(rename = "SPCLT")]
	Spclt,
	/// Tax Anticipation Note
	#[serde(rename = "TAN")]
	Tan,
	/// Tax Allocation
	#[serde(rename = "TAXA")]
	Taxa,
	/// Tax Exempt Commercial Paper
	#[serde(rename = "TECP")]
	Tecp,
	/// Tax Revenue Anticipation Note
	#[serde(rename = "TRAN")]
	Tran,
	/// Variable Rate Demand Note
	#[serde(rename = "VRDN")]
	Vrdn,
	/// Warrant
	#[serde(rename = "WAR")]
	War,
	/// Mutual Fund
	#[serde(rename = "MF")]
	Mf,
	/// Multileg Instrument
	#[serde(rename = "MLEG")]
	Mleg,
	/// No Security Type
	#[serde(rename = "NONE")]
	None,
	/// Options on Futures
	#[serde(rename = "OOF")]
	Oof,
	/// Options on Physical
	#[serde(rename = "OOP")]
	Oop,
	/// Wildcard Entry (was "?" in 4.4, used on Security Definition Request message)
	#[serde(rename = "WLD")]
	Wld,
	/// Cash
	#[serde(rename = "CASH")]
	Cash,
	/// Interest Rate Swap
	#[serde(rename = "IRS")]
	Irs,
	/// Bank Depository Note
	#[serde(rename = "BDN")]
	Bdn,
	/// Canadian Money Markets
	#[serde(rename = "CAMM")]
	Camm,
	/// Canadian Treasury Notes
	#[serde(rename = "CAN")]
	Can,
	/// Canadian Treasury Bills
	#[serde(rename = "CTB")]
	Ctb,
	/// Credit Default Swap
	#[serde(rename = "CDS")]
	Cds,
	/// Canadian Mortgage Bonds
	#[serde(rename = "CMB")]
	Cmb,
	/// Euro Corporate Floating Rate Notes
	#[serde(rename = "EUFRN")]
	Eufrn,
	/// US Corporate Floating Rate Notes
	#[serde(rename = "FRN")]
	Frn,
	/// Canadian Provincial Bonds
	#[serde(rename = "PROV")]
	Prov,
	/// Secured Liquidity Note
	#[serde(rename = "SLQN")]
	Slqn,
	/// Treasury Bill - non US
	#[serde(rename = "TB")]
	Tb,
	/// Term Liquidity Note
	#[serde(rename = "TLQN")]
	Tlqn,
	/// Taxable Municipal CP
	#[serde(rename = "TMCP")]
	Tmcp,
}

impl Default for SecurityType {
	fn default() -> Self {
		SecurityType::Fut
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum InstrmtAssignmentMethod {
	/// Random
	#[serde(rename = "R")]
	Random,
	/// ProRata
	#[serde(rename = "P")]
	ProRata,
}

impl Default for InstrmtAssignmentMethod {
	fn default() -> Self {
		InstrmtAssignmentMethod::Random
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityStatus {
	/// Active
	#[serde(rename = "1")]
	Active,
	/// Inactive
	#[serde(rename = "2")]
	Inactive,
}

impl Default for SecurityStatus {
	fn default() -> Self {
		SecurityStatus::Active
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum RepoCollateralSecurityType {
	/// Future
	#[serde(rename = "FUT")]
	Fut,
	/// Option
	#[serde(rename = "OPT")]
	Opt,
	/// US Treasury Note (Deprecated Value Use TNOTE)
	#[serde(rename = "UST")]
	Ust,
	/// US Treasury Bill (Deprecated Value Use TBILL)
	#[serde(rename = "USTB")]
	Ustb,
	/// Euro Supranational Coupons *
	#[serde(rename = "EUSUPRA")]
	Eusupra,
	/// Federal Agency Coupon
	#[serde(rename = "FAC")]
	Fac,
	/// Federal Agency Discount Note
	#[serde(rename = "FADN")]
	Fadn,
	/// Private Export Funding *
	#[serde(rename = "PEF")]
	Pef,
	/// USD Supranational Coupons *
	#[serde(rename = "SUPRA")]
	Supra,
	/// Corporate Bond
	#[serde(rename = "CORP")]
	Corp,
	/// Corporate Private Placement
	#[serde(rename = "CPP")]
	Cpp,
	/// Convertible Bond
	#[serde(rename = "CB")]
	Cb,
	/// Dual Currency
	#[serde(rename = "DUAL")]
	Dual,
	/// Euro Corporate Bond
	#[serde(rename = "EUCORP")]
	Eucorp,
	/// Indexed Linked
	#[serde(rename = "XLINKD")]
	Xlinkd,
	/// Structured Notes
	#[serde(rename = "STRUCT")]
	Struct,
	/// Yankee Corporate Bond
	#[serde(rename = "YANK")]
	Yank,
	/// Foreign Exchange Contract
	#[serde(rename = "FOR")]
	For,
	/// Common Stock
	#[serde(rename = "CS")]
	Cs,
	/// Preferred Stock
	#[serde(rename = "PS")]
	Ps,
	/// Repurchase
	#[serde(rename = "REPO")]
	Repo,
	/// Forward
	#[serde(rename = "FORWARD")]
	Forward,
	/// Buy Sellback
	#[serde(rename = "BUYSELL")]
	Buysell,
	/// Securities Loan
	#[serde(rename = "SECLOAN")]
	Secloan,
	/// Securities Pledge
	#[serde(rename = "SECPLEDGE")]
	Secpledge,
	/// Brady Bond
	#[serde(rename = "BRADY")]
	Brady,
	/// Euro Sovereigns *
	#[serde(rename = "EUSOV")]
	Eusov,
	/// US Treasury Bond
	#[serde(rename = "TBOND")]
	Tbond,
	/// Interest Strip From Any Bond Or Note
	#[serde(rename = "TINT")]
	Tint,
	/// Treasury Inflation Protected Securities
	#[serde(rename = "TIPS")]
	Tips,
	/// Principal Strip Of A Callable Bond Or Note
	#[serde(rename = "TCAL")]
	Tcal,
	/// Principal Strip From A Non-Callable Bond Or Note
	#[serde(rename = "TPRN")]
	Tprn,
	/// US Treasury Note
	#[serde(rename = "TNOTE")]
	Tnote,
	/// US Treasury Bill
	#[serde(rename = "TBILL")]
	Tbill,
	/// Term Loan
	#[serde(rename = "TERM")]
	Term,
	/// Revolver Loan
	#[serde(rename = "RVLV")]
	Rvlv,
	/// Revolver/Term Loan
	#[serde(rename = "RVLVTRM")]
	Rvlvtrm,
	/// Bridge Loan
	#[serde(rename = "BRIDGE")]
	Bridge,
	/// Letter Of Credit
	#[serde(rename = "LOFC")]
	Lofc,
	/// Swing Line Facility
	#[serde(rename = "SWING")]
	Swing,
	/// Debtor In Possession
	#[serde(rename = "DINP")]
	Dinp,
	/// Defaulted
	#[serde(rename = "DEFLTED")]
	Deflted,
	/// Withdrawn
	#[serde(rename = "WITHDRN")]
	Withdrn,
	/// Replaced
	#[serde(rename = "REPLACD")]
	Replacd,
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
	Ba,
	/// Bank Notes
	#[serde(rename = "BN")]
	Bn,
	/// Bill Of Exchanges
	#[serde(rename = "BOX")]
	Box,
	/// Certificate Of Deposit
	#[serde(rename = "CD")]
	Cd,
	/// Call Loans
	#[serde(rename = "CL")]
	Cl,
	/// Commercial Paper
	#[serde(rename = "CP")]
	Cp,
	/// Deposit Notes
	#[serde(rename = "DN")]
	Dn,
	/// Euro Certificate Of Deposit
	#[serde(rename = "EUCD")]
	Eucd,
	/// Euro Commercial Paper
	#[serde(rename = "EUCP")]
	Eucp,
	/// Liquidity Note
	#[serde(rename = "LQN")]
	Lqn,
	/// Medium Term Notes
	#[serde(rename = "MTN")]
	Mtn,
	/// Overnight
	#[serde(rename = "ONITE")]
	Onite,
	/// Promissory Note
	#[serde(rename = "PN")]
	Pn,
	/// Plazos Fijos
	#[serde(rename = "PZFJ")]
	Pzfj,
	/// Short Term Loan Note
	#[serde(rename = "STN")]
	Stn,
	/// Time Deposit
	#[serde(rename = "TD")]
	Td,
	/// Extended Comm Note
	#[serde(rename = "XCN")]
	Xcn,
	/// Yankee Certificate Of Deposit
	#[serde(rename = "YCD")]
	Ycd,
	/// Asset-backed Securities
	#[serde(rename = "ABS")]
	Abs,
	/// Corp. Mortgage-backed Securities
	#[serde(rename = "CMBS")]
	Cmbs,
	/// Collateralized Mortgage Obligation
	#[serde(rename = "CMO")]
	Cmo,
	/// IOETTE Mortgage
	#[serde(rename = "IET")]
	Iet,
	/// Mortgage-backed Securities
	#[serde(rename = "MBS")]
	Mbs,
	/// Mortgage Interest Only
	#[serde(rename = "MIO")]
	Mio,
	/// Mortgage Principal Only
	#[serde(rename = "MPO")]
	Mpo,
	/// Mortgage Private Placement
	#[serde(rename = "MPP")]
	Mpp,
	/// Miscellaneous Pass-through
	#[serde(rename = "MPT")]
	Mpt,
	/// Pfandbriefe *
	#[serde(rename = "PFAND")]
	Pfand,
	/// To Be Announced
	#[serde(rename = "TBA")]
	Tba,
	/// Other Anticipation Notes (BAN, GAN, etc.)
	#[serde(rename = "AN")]
	An,
	/// Certificate Of Obligation
	#[serde(rename = "COFO")]
	Cofo,
	/// Certificate Of Participation
	#[serde(rename = "COFP")]
	Cofp,
	/// General Obligation Bonds
	#[serde(rename = "GO")]
	Go,
	/// Mandatory Tender
	#[serde(rename = "MT")]
	Mt,
	/// Revenue Anticipation Note
	#[serde(rename = "RAN")]
	Ran,
	/// Revenue Bonds
	#[serde(rename = "REV")]
	Rev,
	/// Special Assessment
	#[serde(rename = "SPCLA")]
	Spcla,
	/// Special Obligation
	#[serde(rename = "SPCLO")]
	Spclo,
	/// Special Tax
	#[serde(rename = "SPCLT")]
	Spclt,
	/// Tax Anticipation Note
	#[serde(rename = "TAN")]
	Tan,
	/// Tax Allocation
	#[serde(rename = "TAXA")]
	Taxa,
	/// Tax Exempt Commercial Paper
	#[serde(rename = "TECP")]
	Tecp,
	/// Tax Revenue Anticipation Note
	#[serde(rename = "TRAN")]
	Tran,
	/// Variable Rate Demand Note
	#[serde(rename = "VRDN")]
	Vrdn,
	/// Warrant
	#[serde(rename = "WAR")]
	War,
	/// Mutual Fund
	#[serde(rename = "MF")]
	Mf,
	/// Multileg Instrument
	#[serde(rename = "MLEG")]
	Mleg,
	/// No Security Type
	#[serde(rename = "NONE")]
	None,
	/// Options on Futures
	#[serde(rename = "OOF")]
	Oof,
	/// Options on Physical
	#[serde(rename = "OOP")]
	Oop,
	/// Wildcard Entry (was "?" in 4.4, used on Security Definition Request message)
	#[serde(rename = "WLD")]
	Wld,
	/// Cash
	#[serde(rename = "CASH")]
	Cash,
	/// Interest Rate Swap
	#[serde(rename = "IRS")]
	Irs,
	/// Bank Depository Note
	#[serde(rename = "BDN")]
	Bdn,
	/// Canadian Money Markets
	#[serde(rename = "CAMM")]
	Camm,
	/// Canadian Treasury Notes
	#[serde(rename = "CAN")]
	Can,
	/// Canadian Treasury Bills
	#[serde(rename = "CTB")]
	Ctb,
	/// Credit Default Swap
	#[serde(rename = "CDS")]
	Cds,
	/// Canadian Mortgage Bonds
	#[serde(rename = "CMB")]
	Cmb,
	/// Euro Corporate Floating Rate Notes
	#[serde(rename = "EUFRN")]
	Eufrn,
	/// US Corporate Floating Rate Notes
	#[serde(rename = "FRN")]
	Frn,
	/// Canadian Provincial Bonds
	#[serde(rename = "PROV")]
	Prov,
	/// Secured Liquidity Note
	#[serde(rename = "SLQN")]
	Slqn,
	/// Treasury Bill - non US
	#[serde(rename = "TB")]
	Tb,
	/// Term Liquidity Note
	#[serde(rename = "TLQN")]
	Tlqn,
	/// Taxable Municipal CP
	#[serde(rename = "TMCP")]
	Tmcp,
}

impl Default for RepoCollateralSecurityType {
	fn default() -> Self {
		RepoCollateralSecurityType::Fut
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
	CocosIslands,
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
	CongoTheDemocraticRepublicOfThe,
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
	IranIslamicRepublicOf,
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
	KoreaDemocraticPeopleSRepublicOf,
	/// KOREA, REPUBLIC OF
	#[serde(rename = "KR")]
	KoreaRepublicOf,
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
	MacedoniaTheFormerYugoslavRepublicOf,
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
	MicronesiaFederatedStatesOf,
	/// MOLDOVA, REPUBLIC OF
	#[serde(rename = "MD")]
	MoldovaRepublicOf,
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
	PalestinianTerritoryOccupied,
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
	TaiwanProvinceOfChina,
	/// TAJIKISTAN
	#[serde(rename = "TJ")]
	Tajikistan,
	/// TANZANIA, UNITED REPUBLIC OF
	#[serde(rename = "TZ")]
	TanzaniaUnitedRepublicOf,
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

impl Default for CountryOfIssue {
	fn default() -> Self {
		CountryOfIssue::Afghanistan
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum StrikeCurrency {
	/// Afghani
	#[serde(rename = "AFA")]
	Afa,
	/// Algerian Dinar
	#[serde(rename = "DZD")]
	Dzd,
	/// Andorran Peseta
	#[serde(rename = "ADP")]
	Adp,
	/// Argentine Peso
	#[serde(rename = "ARS")]
	Ars,
	/// Armenian Dram
	#[serde(rename = "AMD")]
	Amd,
	/// Aruban Guilder
	#[serde(rename = "AWG")]
	Awg,
	/// Australian Dollar
	#[serde(rename = "AUD")]
	Aud,
	/// Azerbaijanian Manat
	#[serde(rename = "AZM")]
	Azm,
	/// Bahamian Dollar
	#[serde(rename = "BSD")]
	Bsd,
	/// Bahraini Dinar
	#[serde(rename = "BHD")]
	Bhd,
	/// Baht
	#[serde(rename = "THB")]
	Thb,
	/// Balboa
	#[serde(rename = "PAB")]
	Pab,
	/// Barbados Dollar
	#[serde(rename = "BBD")]
	Bbd,
	/// Belarussian Ruble
	#[serde(rename = "BYB")]
	Byb,
	/// Belgian Franc
	#[serde(rename = "BEF")]
	Bef,
	/// Belize Dollar
	#[serde(rename = "BZD")]
	Bzd,
	/// Bermudian Dollar
	#[serde(rename = "BMD")]
	Bmd,
	/// Bolivar
	#[serde(rename = "VEB")]
	Veb,
	/// Boliviano
	#[serde(rename = "BOB")]
	Bob,
	/// Brazilian Real
	#[serde(rename = "BRL")]
	Brl,
	/// Brunei Dollar
	#[serde(rename = "BND")]
	Bnd,
	/// Burundi Franc
	#[serde(rename = "BIF")]
	Bif,
	/// CFA Franc BCEAO+
	#[serde(rename = "XOF")]
	Xof,
	/// CFA Franc BEAC#
	#[serde(rename = "XAF")]
	Xaf,
	/// CFP Franc
	#[serde(rename = "XPF")]
	Xpf,
	/// Canadian Dollar
	#[serde(rename = "CAD")]
	Cad,
	/// Cape Verde Escudo
	#[serde(rename = "CVE")]
	Cve,
	/// Cayman Islands Dollar
	#[serde(rename = "KYD")]
	Kyd,
	/// Cedi
	#[serde(rename = "GHC")]
	Ghc,
	/// Chilean Peso
	#[serde(rename = "CLP")]
	Clp,
	/// Colombian Peso
	#[serde(rename = "COP")]
	Cop,
	/// Comoro Franc
	#[serde(rename = "KMF")]
	Kmf,
	/// Convertible Marks
	#[serde(rename = "BAM")]
	Bam,
	/// Cordoba Oro
	#[serde(rename = "NIO")]
	Nio,
	/// Costa Rican Colon
	#[serde(rename = "CRC")]
	Crc,
	/// Cuban Peso
	#[serde(rename = "CUP")]
	Cup,
	/// Cyprus Pound
	#[serde(rename = "CYP")]
	Cyp,
	/// Czech Koruna
	#[serde(rename = "CZK")]
	Czk,
	/// Dalasi
	#[serde(rename = "GMD")]
	Gmd,
	/// Danish Krone
	#[serde(rename = "DKK")]
	Dkk,
	/// Denar
	#[serde(rename = "MKD")]
	Mkd,
	/// Deutsche Mark
	#[serde(rename = "DEM")]
	Dem,
	/// Djibouti Franc
	#[serde(rename = "DJF")]
	Djf,
	/// Dobra
	#[serde(rename = "STD")]
	Std,
	/// Dominican Peso
	#[serde(rename = "DOP")]
	Dop,
	/// Dong
	#[serde(rename = "VND")]
	Vnd,
	/// Drachma
	#[serde(rename = "GRD")]
	Grd,
	/// East Caribbean Dollar
	#[serde(rename = "XCD")]
	Xcd,
	/// Egyptian Pound
	#[serde(rename = "EGP")]
	Egp,
	/// El Salvador Colon
	#[serde(rename = "SVC")]
	Svc,
	/// Ethiopian Birr
	#[serde(rename = "ETB")]
	Etb,
	/// Euro
	#[serde(rename = "EUR")]
	Eur,
	/// Falkland Islands Pound
	#[serde(rename = "FKP")]
	Fkp,
	/// Fiji Dollar
	#[serde(rename = "FJD")]
	Fjd,
	/// Forint
	#[serde(rename = "HUF")]
	Huf,
	/// Franc Congolais
	#[serde(rename = "CDF")]
	Cdf,
	/// French Franc
	#[serde(rename = "FRF")]
	Frf,
	/// Gibraltar Pound
	#[serde(rename = "GIP")]
	Gip,
	/// Gourde
	#[serde(rename = "HTG")]
	Htg,
	/// Guarani
	#[serde(rename = "PYG")]
	Pyg,
	/// Guinea Franc
	#[serde(rename = "GNF")]
	Gnf,
	/// Guinea-Bissau Peso
	#[serde(rename = "GWP")]
	Gwp,
	/// Guyana Dollar
	#[serde(rename = "GYD")]
	Gyd,
	/// Hong Kong Dollar
	#[serde(rename = "HKD")]
	Hkd,
	/// Hryvnia
	#[serde(rename = "UAH")]
	Uah,
	/// Iceland Krona
	#[serde(rename = "ISK")]
	Isk,
	/// Indian Rupee
	#[serde(rename = "INR")]
	Inr,
	/// Iranian Rial
	#[serde(rename = "IRR")]
	Irr,
	/// Iraqi Dinar
	#[serde(rename = "IQD")]
	Iqd,
	/// Irish Pound
	#[serde(rename = "IEP")]
	Iep,
	/// Italian Lira
	#[serde(rename = "ITL")]
	Itl,
	/// Jamaican Dollar
	#[serde(rename = "JMD")]
	Jmd,
	/// Jordanian Dinar
	#[serde(rename = "JOD")]
	Jod,
	/// Kenyan Shilling
	#[serde(rename = "KES")]
	Kes,
	/// Kina
	#[serde(rename = "PGK")]
	Pgk,
	/// Kip
	#[serde(rename = "LAK")]
	Lak,
	/// Kroon
	#[serde(rename = "EEK")]
	Eek,
	/// Kuna
	#[serde(rename = "HRK")]
	Hrk,
	/// Kuwaiti Dinar
	#[serde(rename = "KWD")]
	Kwd,
	/// Kwacha
	#[serde(rename = "MWK")]
	Mwk,
	/// Kwacha
	#[serde(rename = "ZMK")]
	Zmk,
	/// Kwanza Reajustado
	#[serde(rename = "AOR")]
	Aor,
	/// Kyat
	#[serde(rename = "MMK")]
	Mmk,
	/// Lari
	#[serde(rename = "GEL")]
	Gel,
	/// Latvian Lats
	#[serde(rename = "LVL")]
	Lvl,
	/// Lebanese Pound
	#[serde(rename = "LBP")]
	Lbp,
	/// Lek
	#[serde(rename = "ALL")]
	All,
	/// Lempira
	#[serde(rename = "HNL")]
	Hnl,
	/// Leone
	#[serde(rename = "SLL")]
	Sll,
	/// Leu
	#[serde(rename = "ROL")]
	Rol,
	/// Lev
	#[serde(rename = "BGL")]
	Bgl,
	/// Liberian Dollar
	#[serde(rename = "LRD")]
	Lrd,
	/// Libyan Dinar
	#[serde(rename = "LYD")]
	Lyd,
	/// Lilangeni
	#[serde(rename = "SZL")]
	Szl,
	/// Lithuanian Litas
	#[serde(rename = "LTL")]
	Ltl,
	/// Loti
	#[serde(rename = "LSL")]
	Lsl,
	/// Luxembourg Franc
	#[serde(rename = "LUF")]
	Luf,
	/// Malagasy Franc
	#[serde(rename = "MGF")]
	Mgf,
	/// Malaysian Ringgit
	#[serde(rename = "MYR")]
	Myr,
	/// Maltese Lira
	#[serde(rename = "MTL")]
	Mtl,
	/// Manat
	#[serde(rename = "TMM")]
	Tmm,
	/// Markka
	#[serde(rename = "FIM")]
	Fim,
	/// Mauritius Rupee
	#[serde(rename = "MUR")]
	Mur,
	/// Metical
	#[serde(rename = "MZM")]
	Mzm,
	/// Mexican Peso
	#[serde(rename = "MXN")]
	Mxn,
	/// Mexican Unidad de Inversion (UDI)
	#[serde(rename = "MXV")]
	Mxv,
	/// Moldovan Leu
	#[serde(rename = "MDL")]
	Mdl,
	/// Moroccan Dirham
	#[serde(rename = "MAD")]
	Mad,
	/// Mvdol
	#[serde(rename = "BOV")]
	Bov,
	/// Naira
	#[serde(rename = "NGN")]
	Ngn,
	/// Nakfa
	#[serde(rename = "ERN")]
	Ern,
	/// Namibia Dollar
	#[serde(rename = "NAD")]
	Nad,
	/// Nepalese Rupee
	#[serde(rename = "NPR")]
	Npr,
	/// Netherlands Antillian Guilder
	#[serde(rename = "ANG")]
	Ang,
	/// Netherlands Guilder
	#[serde(rename = "NLG")]
	Nlg,
	/// New Dinar
	#[serde(rename = "YUM")]
	Yum,
	/// New Israeli Sheqel
	#[serde(rename = "ILS")]
	Ils,
	/// New Kwanza
	#[serde(rename = "AON")]
	Aon,
	/// New Taiwan Dollar
	#[serde(rename = "TWD")]
	Twd,
	/// New Zaire
	#[serde(rename = "ZRN")]
	Zrn,
	/// New Zealand Dollar
	#[serde(rename = "NZD")]
	Nzd,
	/// Next day
	#[serde(rename = "USN")]
	Usn,
	/// Ngultrum
	#[serde(rename = "BTN")]
	Btn,
	/// North Korean Won
	#[serde(rename = "KPW")]
	Kpw,
	/// Norwegian Krone
	#[serde(rename = "NOK")]
	Nok,
	/// Nuevo Sol
	#[serde(rename = "PEN")]
	Pen,
	/// Ouguiya
	#[serde(rename = "MRO")]
	Mro,
	/// Pa'anga
	#[serde(rename = "TOP")]
	Top,
	/// Pakistan Rupee
	#[serde(rename = "PKR")]
	Pkr,
	/// Pataca
	#[serde(rename = "MOP")]
	Mop,
	/// Peso Uruguayo
	#[serde(rename = "UYU")]
	Uyu,
	/// Philippine Peso
	#[serde(rename = "PHP")]
	Php,
	/// Portuguese Escudo
	#[serde(rename = "PTE")]
	Pte,
	/// Pound Sterling
	#[serde(rename = "GBP")]
	Gbp,
	/// Pula
	#[serde(rename = "BWP")]
	Bwp,
	/// Qatari Rial
	#[serde(rename = "QAR")]
	Qar,
	/// Quetzal
	#[serde(rename = "GTQ")]
	Gtq,
	/// Rand
	#[serde(rename = "ZAR")]
	Zar,
	/// Rial Omani
	#[serde(rename = "OMR")]
	Omr,
	/// Riel
	#[serde(rename = "KHR")]
	Khr,
	/// Rufiyaa
	#[serde(rename = "MVR")]
	Mvr,
	/// Rupiah
	#[serde(rename = "IDR")]
	Idr,
	/// Russian Ruble
	#[serde(rename = "RUB")]
	Rub,
	/// Russian Ruble
	#[serde(rename = "RUR")]
	Rur,
	/// Rwanda Franc
	#[serde(rename = "RWF")]
	Rwf,
	/// SDR
	#[serde(rename = "XDR")]
	Xdr,
	/// Same day
	#[serde(rename = "USS")]
	Uss,
	/// Saudi Riyal
	#[serde(rename = "SAR")]
	Sar,
	/// Schilling
	#[serde(rename = "ATS")]
	Ats,
	/// Seychelles Rupee
	#[serde(rename = "SCR")]
	Scr,
	/// Singapore Dollar
	#[serde(rename = "SGD")]
	Sgd,
	/// Slovak Koruna
	#[serde(rename = "SKK")]
	Skk,
	/// Solomon Islands Dollar
	#[serde(rename = "SBD")]
	Sbd,
	/// Som
	#[serde(rename = "KGS")]
	Kgs,
	/// Somali Shilling
	#[serde(rename = "SOS")]
	Sos,
	/// Spanish Peseta
	#[serde(rename = "ESP")]
	Esp,
	/// Sri Lanka Rupee
	#[serde(rename = "LKR")]
	Lkr,
	/// St Helena Pound
	#[serde(rename = "SHP")]
	Shp,
	/// Sucre
	#[serde(rename = "ECS")]
	Ecs,
	/// Sudanese Dinar
	#[serde(rename = "SDD")]
	Sdd,
	/// Surinam Guilder
	#[serde(rename = "SRG")]
	Srg,
	/// Swedish Krona
	#[serde(rename = "SEK")]
	Sek,
	/// Swiss Franc
	#[serde(rename = "CHF")]
	Chf,
	/// Syrian Pound
	#[serde(rename = "SYP")]
	Syp,
	/// Tajik Ruble
	#[serde(rename = "TJR")]
	Tjr,
	/// Taka
	#[serde(rename = "BDT")]
	Bdt,
	/// Tala
	#[serde(rename = "WST")]
	Wst,
	/// Tanzanian Shilling
	#[serde(rename = "TZS")]
	Tzs,
	/// Tenge
	#[serde(rename = "KZT")]
	Kzt,
	/// Timor Escudo
	#[serde(rename = "TPE")]
	Tpe,
	/// Tolar
	#[serde(rename = "SIT")]
	Sit,
	/// Trinidad and Tobago Dollar
	#[serde(rename = "TTD")]
	Ttd,
	/// Tugrik
	#[serde(rename = "MNT")]
	Mnt,
	/// Tunisian Dinar
	#[serde(rename = "TND")]
	Tnd,
	/// Turkish Lira
	#[serde(rename = "TRL")]
	Trl,
	/// UAE Dirham
	#[serde(rename = "AED")]
	Aed,
	/// US Dollar
	#[serde(rename = "USD")]
	Usd,
	/// Uganda Shilling
	#[serde(rename = "UGX")]
	Ugx,
	/// Unidad de Valor Constante (UVC)
	#[serde(rename = "ECV")]
	Ecv,
	/// Unidades de fomento
	#[serde(rename = "CLF")]
	Clf,
	/// Uzbekistan Sum
	#[serde(rename = "UZS")]
	Uzs,
	/// Vatu
	#[serde(rename = "VUV")]
	Vuv,
	/// Won
	#[serde(rename = "KRW")]
	Krw,
	/// Yemeni Rial
	#[serde(rename = "YER")]
	Yer,
	/// Yen
	#[serde(rename = "JPY")]
	Jpy,
	/// Yuan Renminbi
	#[serde(rename = "CNY")]
	Cny,
	/// Zimbabwe Dollar
	#[serde(rename = "ZWD")]
	Zwd,
	/// Zloty
	#[serde(rename = "PLN")]
	Pln,
	/// financial Rand
	#[serde(rename = "ZAL")]
	Zal,
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
	#[serde(rename = "XAU")]
	Xau,
	/// European Composite Unit (EURCO)
	#[serde(rename = "XBA")]
	Xba,
	/// European Monetary Unit (E.M.U.-6)
	#[serde(rename = "XBB")]
	Xbb,
	/// European Unit of Account 9 (E.U.A.- 9)
	#[serde(rename = "XBC")]
	Xbc,
	/// European Unit of Account 17 (E.U.A.- 17)
	#[serde(rename = "XBD")]
	Xbd,
	/// Palladium
	#[serde(rename = "XPD")]
	Xpd,
	/// Platinum
	#[serde(rename = "XPT")]
	Xpt,
	/// Silver
	#[serde(rename = "XAG")]
	Xag,
	/// UIC-Franc
	#[serde(rename = "XFU")]
	Xfu,
	/// Gold-Franc
	#[serde(rename = "XFO")]
	Xfo,
	/// Codes specifically reserved for testing purposes
	#[serde(rename = "XTS")]
	Xts,
	/// Codes assigned for transactions where no currency is involved
	#[serde(rename = "XXX")]
	Xxx,
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

impl Default for StrikeCurrency {
	fn default() -> Self {
		StrikeCurrency::Afa
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
	/// pounds
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
	MetricTons,
	/// Tons (US)
	#[serde(rename = "tn")]
	Tons,
	/// US Dollars
	#[serde(rename = "USD")]
	UsDollars,
}

impl Default for UnitOfMeasure {
	fn default() -> Self {
		UnitOfMeasure::Barrels
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceUnitOfMeasure {
	/// Barrels
	#[serde(rename = "Bbl")]
	Barrels,
	/// Billion cubic feet
	#[serde(rename = "Bcf")]
	BillionCubicFeet,
	/// Bushels
	#[serde(rename = "Bu")]
	Bushels,
	/// pounds
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
	MetricTons,
	/// Tons (US)
	#[serde(rename = "tn")]
	Tons,
	/// US Dollars
	#[serde(rename = "USD")]
	UsDollars,
}

impl Default for PriceUnitOfMeasure {
	fn default() -> Self {
		PriceUnitOfMeasure::Barrels
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SettlMethod {
	/// Cash settlement required
	#[serde(rename = "C")]
	CashSettlementRequired,
	/// Physical settlement required
	#[serde(rename = "P")]
	PhysicalSettlementRequired,
}

impl Default for SettlMethod {
	fn default() -> Self {
		SettlMethod::CashSettlementRequired
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
}

impl Default for ExerciseStyle {
	fn default() -> Self {
		ExerciseStyle::European
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceQuoteMethod {
	/// Standard, money per unit of a physical
	#[serde(rename = "STD")]
	StandardMoneyPerUnitOfAPhysical,
	/// Index
	#[serde(rename = "INX")]
	Index,
	/// Interest rate Index
	#[serde(rename = "INT")]
	InterestRateIndex,
}

impl Default for PriceQuoteMethod {
	fn default() -> Self {
		PriceQuoteMethod::StandardMoneyPerUnitOfAPhysical
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum FuturesValuationMethod {
	/// premium style
	#[serde(rename = "EQTY")]
	PremiumStyle,
	/// futures style mark-to-market
	#[serde(rename = "FUT")]
	FuturesStyleMarkToMarket,
	/// futures style with an attached cash adjustment
	#[serde(rename = "FUTDA")]
	FuturesStyleWithAnAttachedCashAdjustment,
}

impl Default for FuturesValuationMethod {
	fn default() -> Self {
		FuturesValuationMethod::PremiumStyle
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ListMethod {
	/// pre-listed only
	#[serde(rename = "0")]
	PreListedOnly,
	/// user requested
	#[serde(rename = "1")]
	UserRequested,
}

impl Default for ListMethod {
	fn default() -> Self {
		ListMethod::PreListedOnly
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
}

impl Default for TimeUnit {
	fn default() -> Self {
		TimeUnit::Hour
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CPProgram {
	/// 3(a)(3)
	#[serde(rename = "1")]
	N3,
	/// 4(2)
	#[serde(rename = "2")]
	N4,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for CPProgram {
	fn default() -> Self {
		CPProgram::N3
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum SecurityAltIDSource {
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
	ClearingHouseClearingOrganization,
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
}

impl Default for SecurityAltIDSource {
	fn default() -> Self {
		SecurityAltIDSource::Cusip
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum EventType {
	/// Put
	#[serde(rename = "1")]
	Put,
	/// Call
	#[serde(rename = "2")]
	Call,
	/// Tender
	#[serde(rename = "3")]
	Tender,
	/// Sinking Fund Call
	#[serde(rename = "4")]
	SinkingFundCall,
	/// Activation
	#[serde(rename = "5")]
	Activation,
	/// Inactiviation
	#[serde(rename = "6")]
	Inactiviation,
	/// Last Eligible Trade Date
	#[serde(rename = "7")]
	LastEligibleTradeDate,
	/// Swap start date
	#[serde(rename = "8")]
	SwapStartDate,
	/// Swap end date
	#[serde(rename = "9")]
	SwapEndDate,
	/// Swap roll date
	#[serde(rename = "10")]
	SwapRollDate,
	/// Swap next start date
	#[serde(rename = "11")]
	SwapNextStartDate,
	/// Swap next roll date
	#[serde(rename = "12")]
	SwapNextRollDate,
	/// First delivery date
	#[serde(rename = "13")]
	FirstDeliveryDate,
	/// Last delivery date
	#[serde(rename = "14")]
	LastDeliveryDate,
	/// Initiatl inventory due date
	#[serde(rename = "15")]
	InitiatlInventoryDueDate,
	/// Final inventory due date
	#[serde(rename = "16")]
	FinalInventoryDueDate,
	/// First intent date
	#[serde(rename = "17")]
	FirstIntentDate,
	/// Last intent date
	#[serde(rename = "18")]
	LastIntentDate,
	/// Position removal date
	#[serde(rename = "19")]
	PositionRemovalDate,
	/// Other
	#[serde(rename = "99")]
	Other,
}

impl Default for EventType {
	fn default() -> Self {
		EventType::Put
	}
}
