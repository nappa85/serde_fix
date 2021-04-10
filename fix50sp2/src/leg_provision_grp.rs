
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionGrp {
	/// NoLegProvisions
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40448")]
	pub leg_provisions: Option<crate::entities::RepeatingValues<LegProvision>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvision {
	/// Required if NoLegProvisions(40448) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40449")]
	pub leg_provision_type: Option<LegProvisionType>,
	/// LegProvisionDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40450")]
	pub leg_provision_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the instrument's leg provision.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40451")]
	pub leg_provision_date_business_day_convention: Option<LegProvisionDateBusinessDayConvention>,
	/// LegProvisionDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40453")]
	pub leg_provision_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// Conditionally required when LegProvisionDateTenorUnit(40455) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40454")]
	pub leg_provision_date_tenor_period: Option<i32>,
	/// Conditionally required when LegProvisionDateTenorPeriod(40454) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40455")]
	pub leg_provision_date_tenor_unit: Option<LegProvisionDateTenorUnit>,
	/// LegProvisionCalculationAgent
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40456")]
	pub leg_provision_calculation_agent: Option<LegProvisionCalculationAgent>,
	/// LegProvisionOptionSinglePartyBuyerSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40457")]
	pub leg_provision_option_single_party_buyer_side: Option<LegProvisionOptionSinglePartyBuyerSide>,
	/// LegProvisionOptionSinglePartySellerSide
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40458")]
	pub leg_provision_option_single_party_seller_side: Option<LegProvisionOptionSinglePartySellerSide>,
	/// LegProvisionOptionExerciseStyle
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40459")]
	pub leg_provision_option_exercise_style: Option<LegProvisionOptionExerciseStyle>,
	/// LegProvisionOptionExerciseMultipleNotional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40460")]
	pub leg_provision_option_exercise_multiple_notional: Option<f64>,
	/// LegProvisionOptionExerciseMinimumNotional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40461")]
	pub leg_provision_option_exercise_minimum_notional: Option<f64>,
	/// LegProvisionOptionExerciseMaximumNotional
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40462")]
	pub leg_provision_option_exercise_maximum_notional: Option<f64>,
	/// LegProvisionOptionMinimumNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40463")]
	pub leg_provision_option_minimum_number: Option<i32>,
	/// LegProvisionOptionMaximumNumber
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40464")]
	pub leg_provision_option_maximum_number: Option<i32>,
	/// LegProvisionOptionExerciseConfirmation
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40465")]
	pub leg_provision_option_exercise_confirmation: Option<crate::entities::Boolean>,
	/// LegProvisionCashSettlMethod
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40466")]
	pub leg_provision_cash_settl_method: Option<LegProvisionCashSettlMethod>,
	/// LegProvisionCashSettlCurrency
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40467")]
	pub leg_provision_cash_settl_currency: Option<String>,
	/// LegProvisionCashSettlCurrency2
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40468")]
	pub leg_provision_cash_settl_currency_2: Option<String>,
	/// LegProvisionCashSettlQuoteType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40469")]
	pub leg_provision_cash_settl_quote_type: Option<LegProvisionCashSettlQuoteType>,
	/// LegProvisionText
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40472")]
	pub leg_provision_text: Option<String>,
	/// Must be set if EncodedLegProvisionText(40981) field is specified and must immediately precede it.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40980")]
	pub encoded_leg_provision_text_len: Option<usize>,
	/// Encoded (non-ASCII characters) representation of the LegProvisionText(40472) field in the encoded format specified via the
	/// MessageEncoding(347) field.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40981")]
	pub encoded_leg_provision_text: Option<String>,
	/// LegProvisionBreakFeeElection
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42506")]
	pub leg_provision_break_fee_election: Option<i32>,
	/// LegProvisionBreakFeeRate
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42507")]
	pub leg_provision_break_fee_rate: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionType {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionDateBusinessDayConvention {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionDateTenorUnit {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionCalculationAgent {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionOptionSinglePartyBuyerSide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionOptionSinglePartySellerSide {
	/// Buy
	#[serde(rename = "1")]
	Buy,
	/// Sell
	#[serde(rename = "2")]
	Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionOptionExerciseStyle {
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
pub enum LegProvisionCashSettlMethod {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegProvisionCashSettlQuoteType {
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
