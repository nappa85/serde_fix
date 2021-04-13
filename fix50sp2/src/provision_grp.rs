
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionGrp {
	/// NoProvisions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40090")]
	pub provisions: Option<fix_common::RepeatingValues<Provision>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Provision {
	/// Required if NoProvisions(40090) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40091")]
	pub provision_type: Option<ProvisionType>,
	/// ProvisionDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40092")]
	pub provision_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the instrument provisions.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40093")]
	pub provision_date_business_day_convention: Option<ProvisionDateBusinessDayConvention>,
	/// ProvisionDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40095")]
	pub provision_date_adjusted: Option<fix_common::LocalMktDate>,
	/// Conditionally required when ProvisionDateTenorUnit(40097) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40096")]
	pub provision_date_tenor_period: Option<i32>,
	/// Conditionally required when ProvisionDateTenorPeriod(40096) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40097")]
	pub provision_date_tenor_unit: Option<ProvisionDateTenorUnit>,
	/// ProvisionCalculationAgent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40098")]
	pub provision_calculation_agent: Option<ProvisionCalculationAgent>,
	/// ProvisionOptionSinglePartyBuyerSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40099")]
	pub provision_option_single_party_buyer_side: Option<ProvisionOptionSinglePartyBuyerSide>,
	/// ProvisionOptionSinglePartySellerSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40100")]
	pub provision_option_single_party_seller_side: Option<ProvisionOptionSinglePartySellerSide>,
	/// ProvisionOptionExerciseStyle
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40101")]
	pub provision_option_exercise_style: Option<ProvisionOptionExerciseStyle>,
	/// ProvisionOptionExerciseMultipleNotional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40102")]
	pub provision_option_exercise_multiple_notional: Option<f64>,
	/// ProvisionOptionExerciseMinimumNotional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40103")]
	pub provision_option_exercise_minimum_notional: Option<f64>,
	/// ProvisionOptionExerciseMaximumNotional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40104")]
	pub provision_option_exercise_maximum_notional: Option<f64>,
	/// ProvisionOptionMinimumNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40105")]
	pub provision_option_minimum_number: Option<i32>,
	/// ProvisionOptionMaximumNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40106")]
	pub provision_option_maximum_number: Option<i32>,
	/// ProvisionOptionExerciseConfirmation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40107")]
	pub provision_option_exercise_confirmation: Option<fix_common::Boolean>,
	/// ProvisionCashSettlMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40108")]
	pub provision_cash_settl_method: Option<ProvisionCashSettlMethod>,
	/// ProvisionCashSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40109")]
	pub provision_cash_settl_currency: Option<String>,
	/// ProvisionCashSettlCurrency2
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40110")]
	pub provision_cash_settl_currency_2: Option<String>,
	/// ProvisionCashSettlQuoteType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40111")]
	pub provision_cash_settl_quote_type: Option<ProvisionCashSettlQuoteType>,
	/// ProvisionText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40113")]
	pub provision_text: Option<String>,
	/// Must be set if EncodedProvisionText(40987) field is specified and must immediately precede it.
	#[serde(rename = "40986")]
	/// Encoded (non-ASCII characters) representation of the ProvisionText(40113) field in the encoded format specified via the MessageEncoding(347)
	/// field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(alias = "40987")]
	pub encoded_provision_text: Option<fix_common::EncodedText<40987>>,
	/// ProvisionBreakFeeElection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42707")]
	pub provision_break_fee_election: Option<ProvisionBreakFeeElection>,
	/// ProvisionBreakFeeRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42708")]
	pub provision_break_fee_rate: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionType {
	/// Mandatory early termination
	#[serde(rename = "0")]
	MandatoryEarlyTermination,
	/// Optional early termination
	#[serde(rename = "1")]
	OptionalEarlyTermination,
	/// Cancelable
	#[serde(rename = "2")]
	Cancelable,
	/// Extendible
	#[serde(rename = "3")]
	Extendible,
	/// Mutual early termination
	#[serde(rename = "4")]
	MutualEarlyTermination,
	/// Evergreen
	#[serde(rename = "5")]
	Evergreen,
	/// Callable
	#[serde(rename = "6")]
	Callable,
	/// Puttable
	#[serde(rename = "7")]
	Puttable,
}

impl Default for ProvisionType {
	fn default() -> Self {
		ProvisionType::MandatoryEarlyTermination
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionDateBusinessDayConvention {
	/// Not applicable
	#[serde(rename = "0")]
	NotApplicable,
	/// None (current day)
	#[serde(rename = "1")]
	None,
	/// Following day
	#[serde(rename = "2")]
	FollowingDay,
	/// Floating rate note
	#[serde(rename = "3")]
	FloatingRateNote,
	/// Modified following day
	#[serde(rename = "4")]
	ModifiedFollowingDay,
	/// Preceding day
	#[serde(rename = "5")]
	PrecedingDay,
	/// Modified preceding day
	#[serde(rename = "6")]
	ModifiedPrecedingDay,
	/// Nearest day
	#[serde(rename = "7")]
	NearestDay,
}

impl Default for ProvisionDateBusinessDayConvention {
	fn default() -> Self {
		ProvisionDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionDateTenorUnit {
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

impl Default for ProvisionDateTenorUnit {
	fn default() -> Self {
		ProvisionDateTenorUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCalculationAgent {
	/// Exercising party
	#[serde(rename = "0")]
	ExercisingParty,
	/// Non-exercising party
	#[serde(rename = "1")]
	NonExercisingParty,
	/// As specified in the master agreement
	#[serde(rename = "2")]
	AsSpecifiedInTheMasterAgreement,
	/// As specified in the standard terms supplement
	#[serde(rename = "3")]
	AsSpecifiedInTheStandardTermsSupplement,
}

impl Default for ProvisionCalculationAgent {
	fn default() -> Self {
		ProvisionCalculationAgent::ExercisingParty
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionOptionSinglePartyBuyerSide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

impl Default for ProvisionOptionSinglePartyBuyerSide {
	fn default() -> Self {
		ProvisionOptionSinglePartyBuyerSide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionOptionSinglePartySellerSide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

impl Default for ProvisionOptionSinglePartySellerSide {
	fn default() -> Self {
		ProvisionOptionSinglePartySellerSide::Buy
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionOptionExerciseStyle {
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

impl Default for ProvisionOptionExerciseStyle {
	fn default() -> Self {
		ProvisionOptionExerciseStyle::European
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCashSettlMethod {
	/// Cash price
	#[serde(rename = "0")]
	CashPrice,
	/// Cash price alternate
	#[serde(rename = "1")]
	CashPriceAlternate,
	/// Par yield curve adjusted
	#[serde(rename = "2")]
	ParYieldCurveAdjusted,
	/// Zero coupon yield curve adjusted
	#[serde(rename = "3")]
	ZeroCouponYieldCurveAdjusted,
	/// Par yield curve unadjusted
	#[serde(rename = "4")]
	ParYieldCurveUnadjusted,
	/// Cross currency
	#[serde(rename = "5")]
	CrossCurrency,
	/// Collateralized price
	#[serde(rename = "6")]
	CollateralizedPrice,
}

impl Default for ProvisionCashSettlMethod {
	fn default() -> Self {
		ProvisionCashSettlMethod::CashPrice
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCashSettlQuoteType {
	/// Bid
	#[serde(rename = "0")]
	Bid,
	/// Mid
	#[serde(rename = "1")]
	Mid,
	/// Offer
	#[serde(rename = "2")]
	Offer,
	/// Exercising party pays (See 2000 ISDA Definitions, Section 17.2, Certain Definitions Relating to Cash Settlement, paragraph
	/// (j) for definition of "exercising party pays")
	#[serde(rename = "3")]
	ExercisingPartyPaysForDefinitionOfExercisingPartyPays,
}

impl Default for ProvisionCashSettlQuoteType {
	fn default() -> Self {
		ProvisionCashSettlQuoteType::Bid
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionBreakFeeElection {
	/// Flat fee
	#[serde(rename = "0")]
	FlatFee,
	/// Amortized fee
	#[serde(rename = "1")]
	AmortizedFee,
	/// Funding fee
	#[serde(rename = "2")]
	FundingFee,
	/// Flat fee and funding fee
	#[serde(rename = "3")]
	FlatFeeAndFundingFee,
	/// Amortized fee and funding fee
	#[serde(rename = "4")]
	AmortizedFeeAndFundingFee,
}

impl Default for ProvisionBreakFeeElection {
	fn default() -> Self {
		ProvisionBreakFeeElection::FlatFee
	}
}
