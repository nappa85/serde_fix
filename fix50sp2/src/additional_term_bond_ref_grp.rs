
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdditionalTermBondRefGrp {
	/// NoAdditionalTermBondRefs
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40000")]
	pub additional_term_bond_refs: Option<fix_common::RepeatingValues<AdditionalTermBondRef>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdditionalTermBondRef {
	/// Required if NoAdditionalTermBondRefs(40000) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40001")]
	pub additional_term_bond_security_id: Option<String>,
	/// Conditionally required when AdditionalTermBondSecurityID(40001) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40002")]
	pub additional_term_bond_security_id_source: Option<AdditionalTermBondSecurityIDSource>,
	/// AdditionalTermBondDesc
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40003")]
	pub additional_term_bond_desc: Option<String>,
	/// Must be set if EncodedAdditionalTermBondDesc(40005) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40004")]
	pub encoded_additional_term_bond_desc_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the AdditionalTermBondDesc(40003) field in the encoded format specified via
	/// the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40005")]
	pub encoded_additional_term_bond_desc: Option<String>,
	/// AdditionalTermBondCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40006")]
	pub additional_term_bond_currency: Option<String>,
	/// AdditionalTermBondIssuer
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40007")]
	pub additional_term_bond_issuer: Option<String>,
	/// Must be set if EncodedAdditionalTermBondIssuer(40009) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40008")]
	pub encoded_additional_term_bond_issuer_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the AdditionalTermBondIssuer(40007) field in the encoded format specified
	/// via the MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40009")]
	pub encoded_additional_term_bond_issuer: Option<String>,
	/// AdditionalTermBondSeniority
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40010")]
	pub additional_term_bond_seniority: Option<AdditionalTermBondSeniority>,
	/// AdditionalTermBondCouponType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40011")]
	pub additional_term_bond_coupon_type: Option<AdditionalTermBondCouponType>,
	/// AdditionalTermBondCouponRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40012")]
	pub additional_term_bond_coupon_rate: Option<f32>,
	/// AdditionalTermBondMaturityDate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40013")]
	pub additional_term_bond_maturity_date: Option<fix_common::LocalMktDate>,
	/// AdditionalTermBondParValue
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40014")]
	pub additional_term_bond_par_value: Option<f64>,
	/// AdditionalTermBondCurrentTotalIssuedAmount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40015")]
	pub additional_term_bond_current_total_issued_amount: Option<f64>,
	/// Conditionally required when AdditionalTermBondCouponFrequencyUnit(40017) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40016")]
	pub additional_term_bond_coupon_frequency_period: Option<i32>,
	/// Conditionally required when AdditionalTermBondCouponFrequencyPeriod(40016) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40017")]
	pub additional_term_bond_coupon_frequency_unit: Option<AdditionalTermBondCouponFrequencyUnit>,
	/// AdditionalTermBondDayCount
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40018")]
	pub additional_term_bond_day_count: Option<AdditionalTermBondDayCount>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AdditionalTermBondSecurityIDSource {
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
pub enum AdditionalTermBondSeniority {
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
pub enum AdditionalTermBondCouponType {
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
pub enum AdditionalTermBondCouponFrequencyUnit {
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
pub enum AdditionalTermBondDayCount {
	/// 1/1 (If parties specify the Day Count Fraction to be 1/1 then in calculating the applicable amount, 1 is simply input into
	/// the calculation as the relevant Day Count Fraction.)
	#[serde(rename = "0")]
	N0,
	/// 30/360 (30U/360 or Bond Basis) (Mainly used in the US with the following date adjustment rules: (1) If the investment is End-Of-Month
	/// and Date1 is the last day of February and Date2 is the last day of February, then change Date2 to 30; (2) If the investment
	/// is End-Of-Month and Date1 is the last day of February, then change Date1 to 30; (3) If Date2 is 31 and Date1 is 30 or 31,
	/// then change Date2 to 30; (4) If Date1 is 31, then change Date1 to 30.)
	#[serde(rename = "1")]
	N1,
	/// 30/360 (SIA) (A variant of "30/360" - when Date1 and Date2 are both Feb. 28th or 29th convert them to 30th using the same
	/// logic in the conversion of 31st to 30th.)
	#[serde(rename = "2")]
	N2,
	/// 30/360M (Commonly used day count convention for US mortgage backed securities. Feb 28th (or 29th in a leap year) is always
	/// considered as a 30th for a start date. As a comparison, in the regular 30/360 day count as used by most US agency and corporate
	/// bonds, a start date of Feb 28th (or 29th in a leap year) is still considered as the 28th (or 29th) day of a month of 30 days.)
	#[serde(rename = "3")]
	N3,
	/// 30E/360 (Eurobond Basis) (Also known as 30/360.ISMA, 30S/360, or Special German. Date adjustment rules are: (1) If Date1 falls
	/// on the 31st, then change it to the 30th; (2) If Date2 falls on the 31st, then change it to the 30th.)
	#[serde(rename = "4")]
	N4,
	/// 30E/360 (ISDA) (Date adjustment rules are: (1) if Date1 is the last day of the month, then change Date1 to 30; (2) if D2 is
	/// the last day of the month (unless Date2 is the maturity date and Date2 is in February), then change Date2 to 30.)
	#[serde(rename = "5")]
	N5,
	/// Act/360 (The actual number of days between Date1 and Date2, divided by 360.)
	#[serde(rename = "6")]
	N6,
	/// Act/365 (FIXED) (The actual number of days between Date1 and Date2, divided by 365.)
	#[serde(rename = "7")]
	N7,
	/// Act/Act (AFB) (The actual number of days between Date1 and Date2, the denominator is either 365 (if the calculation period
	/// does not contain the 29th February) or 366 (if the calculation period includes 29th February).)
	#[serde(rename = "8")]
	N8,
	/// Act/Act ICMA (The denominator is the actual number of days in the coupon period multiplied by the number of coupon periods
	/// in the year. Assumes that regular coupons alwaysfall on the same day of the month where possible.)
	#[serde(rename = "9")]
	N9,
	/// Act/Act (ICMA Ultimo) (The Act/Act (ICMA Ultimo) differs from Act/Act (ICMA) method only that it assumes that regular coupons
	/// always fall on the last day of the month.)
	#[serde(rename = "10")]
	N10,
	/// Act/Act ISDA (The denominator varies depending on whether a portion of the relevant calculation period falls within a leap
	/// year. For the portion of the calculation period falling in a leap year, the denominator is 366 and for the portion falling
	/// outside a leap year, the denominator is 365.)
	#[serde(rename = "11")]
	N11,
	/// BUS/252 (Used for Brazilian Real swaps which is based on business days instead of calendar days. The number of business days
	/// divied by 252)
	#[serde(rename = "12")]
	N12,
	/// 30E+/360 (Variation on 30E/360. Date adjustment rules: (1) If Date1 falls on the 31st, then change it to the 30th; (2) If
	/// Date2 falls on the 31st, then change it to 1 and increase Month2 by one, i.e. next month)
	#[serde(rename = "13")]
	N13,
	/// Act/365L (The number of days in a period equal to the actual number of days. The number of days in a year is 365, or if the
	/// period ends in a leap year 366. Used for Sterling floating rate notes. May also be referred to as ISMA-Year.)
	#[serde(rename = "14")]
	N14,
	/// NL365 (The number of days in a period equal to the actual number of days, with the exception of leap days (29th February)
	/// which are ignored. The number of days in a year is 365, even in a leap year.)
	#[serde(rename = "15")]
	N15,
	/// NL360 (This is the same as Act/360, with the exception of leap days (29th February) which are ignored.)
	#[serde(rename = "16")]
	N16,
	/// Act/364 (The actual number of days between Date1 and Date2, divided by 364.)
	#[serde(rename = "17")]
	N17,
	/// 30/365
	#[serde(rename = "18")]
	N18,
	/// 30/Actual
	#[serde(rename = "19")]
	N19,
	/// 30/360 (ICMA or basis rule)
	#[serde(rename = "20")]
	N20,
	/// 30E2/360 (Eurobond basis model two)
	#[serde(rename = "21")]
	N21,
	/// 30E3/360 (Eurobond basis model three)
	#[serde(rename = "22")]
	N22,
	/// Other
	#[serde(rename = "99")]
	N99,
}
