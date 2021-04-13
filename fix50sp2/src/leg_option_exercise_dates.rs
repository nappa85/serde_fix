
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegOptionExerciseDates {
	/// LegOptionExerciseBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41493")]
	pub leg_option_exercise_business_day_convention: Option<LegOptionExerciseBusinessDayConvention>,
	/// LegOptionExerciseBusinessCenterGrp
	#[serde(flatten)]
	pub leg_option_exercise_business_center_grp: Option<super::leg_option_exercise_business_center_grp::LegOptionExerciseBusinessCenterGrp>,
	/// LegOptionExerciseDateGrp
	#[serde(flatten)]
	pub leg_option_exercise_date_grp: Option<super::leg_option_exercise_date_grp::LegOptionExerciseDateGrp>,
	/// LegOptionExerciseEarliestDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41494")]
	pub leg_option_exercise_earliest_date_offset_day_type: Option<LegOptionExerciseEarliestDateOffsetDayType>,
	/// Conditionally required when LegOptionExerciseEarliestDateUnit(41496) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41495")]
	pub leg_option_exercise_earliest_date_period: Option<i32>,
	/// Conditionally required when LegOptionExerciseEarliestDatePeriod(41495) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41496")]
	pub leg_option_exercise_earliest_date_unit: Option<LegOptionExerciseEarliestDateUnit>,
	/// Conditionally required when LegOptionExerciseFrequencyUnit(41498) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41497")]
	pub leg_option_exercise_frequency_period: Option<i32>,
	/// Conditionally required when LegOptionExerciseFequencyPeriod(41497) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41498")]
	pub leg_option_exercise_frequency_unit: Option<LegOptionExerciseFrequencyUnit>,
	/// LegOptionExerciseStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41499")]
	pub leg_option_exercise_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegOptionExerciseStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41500")]
	pub leg_option_exercise_start_date_relative_to: Option<LegOptionExerciseStartDateRelativeTo>,
	/// Conditionally required when LegOptionExerciseStartDateOffsetUnit(41502) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41501")]
	pub leg_option_exercise_start_date_offset_period: Option<i32>,
	/// Conditionally required when LegOptionExerciseStartDateOffsetPeriod(41501) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41502")]
	pub leg_option_exercise_start_date_offset_unit: Option<LegOptionExerciseStartDateOffsetUnit>,
	/// LegOptionExerciseStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41503")]
	pub leg_option_exercise_start_date_offset_day_type: Option<LegOptionExerciseStartDateOffsetDayType>,
	/// LegOptionExerciseStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41504")]
	pub leg_option_exercise_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// LegOptionExerciseSkip
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41505")]
	pub leg_option_exercise_skip: Option<i32>,
	/// LegOptionExerciseNominationDeadline
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41506")]
	pub leg_option_exercise_nomination_deadline: Option<fix_common::LocalMktDate>,
	/// LegOptionExerciseFirstDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41507")]
	pub leg_option_exercise_first_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegOptionExerciseLastDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41508")]
	pub leg_option_exercise_last_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegOptionExerciseEarliestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41509")]
	pub leg_option_exercise_earliest_time: Option<String>,
	/// LegOptionExerciseLatestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41510")]
	pub leg_option_exercise_latest_time: Option<String>,
	/// LegOptionExerciseTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41511")]
	pub leg_option_exercise_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegOptionExerciseBusinessDayConvention {
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

impl Default for LegOptionExerciseBusinessDayConvention {
	fn default() -> Self {
		LegOptionExerciseBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegOptionExerciseEarliestDateOffsetDayType {
	/// Business
	#[serde(rename = "0")]
	Business,
	/// Calendar
	#[serde(rename = "1")]
	Calendar,
	/// Commodity business
	#[serde(rename = "2")]
	CommodityBusiness,
	/// Currency business
	#[serde(rename = "3")]
	CurrencyBusiness,
	/// Exchange business
	#[serde(rename = "4")]
	ExchangeBusiness,
	/// Scheduled trading day
	#[serde(rename = "5")]
	ScheduledTradingDay,
}

impl Default for LegOptionExerciseEarliestDateOffsetDayType {
	fn default() -> Self {
		LegOptionExerciseEarliestDateOffsetDayType::Business
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegOptionExerciseEarliestDateUnit {
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

impl Default for LegOptionExerciseEarliestDateUnit {
	fn default() -> Self {
		LegOptionExerciseEarliestDateUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegOptionExerciseFrequencyUnit {
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

impl Default for LegOptionExerciseFrequencyUnit {
	fn default() -> Self {
		LegOptionExerciseFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegOptionExerciseStartDateRelativeTo {
	/// Trade date
	#[serde(rename = "0")]
	TradeDate,
	/// Settlement date
	#[serde(rename = "1")]
	SettlementDate,
	/// Effective date
	#[serde(rename = "2")]
	EffectiveDate,
	/// Calculation period start date
	#[serde(rename = "3")]
	CalculationPeriodStartDate,
	/// Calculation period end date
	#[serde(rename = "4")]
	CalculationPeriodEndDate,
	/// Reset date
	#[serde(rename = "5")]
	ResetDate,
	/// Last pricing date
	#[serde(rename = "6")]
	LastPricingDate,
	/// Valuation date
	#[serde(rename = "7")]
	ValuationDate,
	/// Cash settlement date
	#[serde(rename = "8")]
	CashSettlementDate,
	/// Option exercise start date
	#[serde(rename = "9")]
	OptionExerciseStartDate,
}

impl Default for LegOptionExerciseStartDateRelativeTo {
	fn default() -> Self {
		LegOptionExerciseStartDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegOptionExerciseStartDateOffsetUnit {
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

impl Default for LegOptionExerciseStartDateOffsetUnit {
	fn default() -> Self {
		LegOptionExerciseStartDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegOptionExerciseStartDateOffsetDayType {
	/// Business
	#[serde(rename = "0")]
	Business,
	/// Calendar
	#[serde(rename = "1")]
	Calendar,
	/// Commodity business
	#[serde(rename = "2")]
	CommodityBusiness,
	/// Currency business
	#[serde(rename = "3")]
	CurrencyBusiness,
	/// Exchange business
	#[serde(rename = "4")]
	ExchangeBusiness,
	/// Scheduled trading day
	#[serde(rename = "5")]
	ScheduledTradingDay,
}

impl Default for LegOptionExerciseStartDateOffsetDayType {
	fn default() -> Self {
		LegOptionExerciseStartDateOffsetDayType::Business
	}
}
