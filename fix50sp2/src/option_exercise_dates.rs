
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OptionExerciseDates {
	/// OptionExerciseBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41118")]
	pub option_exercise_business_day_convention: Option<OptionExerciseBusinessDayConvention>,
	/// OptionExerciseBusinessCenterGrp
	#[serde(flatten)]
	pub option_exercise_business_center_grp: Option<super::option_exercise_business_center_grp::OptionExerciseBusinessCenterGrp>,
	/// OptionExerciseDateGrp
	#[serde(flatten)]
	pub option_exercise_date_grp: Option<super::option_exercise_date_grp::OptionExerciseDateGrp>,
	/// OptionExerciseEarliestDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41119")]
	pub option_exercise_earliest_date_offset_day_type: Option<OptionExerciseEarliestDateOffsetDayType>,
	/// Conditionally required when OptionExerciseEarliestDateUnit(41121) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41120")]
	pub option_exercise_earliest_date_period: Option<i32>,
	/// Conditionally required when OptionExerciseEarliestDatePeriod(41120) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41121")]
	pub option_exercise_earliest_date_unit: Option<OptionExerciseEarliestDateUnit>,
	/// Conditionally required when OptionExerciseFrequencyUnit(41123) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41122")]
	pub option_exercise_frequency_period: Option<i32>,
	/// Conditionally required when OptionExerciseFrequencyPeriod(41122) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41123")]
	pub option_exercise_frequency_unit: Option<OptionExerciseFrequencyUnit>,
	/// OptionExerciseStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41124")]
	pub option_exercise_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// OptionExerciseStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41125")]
	pub option_exercise_start_date_relative_to: Option<OptionExerciseStartDateRelativeTo>,
	/// Conditionally required when OptionExerciseStartDateOffsetUnit(41127) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41126")]
	pub option_exercise_start_date_offset_period: Option<i32>,
	/// Conditionally required when OptionExerciseStartDateOffsetPeriod(41126) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41127")]
	pub option_exercise_start_date_offset_unit: Option<OptionExerciseStartDateOffsetUnit>,
	/// OptionExerciseStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41128")]
	pub option_exercise_start_date_offset_day_type: Option<OptionExerciseStartDateOffsetDayType>,
	/// OptionExerciseStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41129")]
	pub option_exercise_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// OptionExerciseSkip
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41130")]
	pub option_exercise_skip: Option<i32>,
	/// OptionExerciseNominationDeadline
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41131")]
	pub option_exercise_nomination_deadline: Option<fix_common::LocalMktDate>,
	/// OptionExerciseFirstDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41132")]
	pub option_exercise_first_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// OptionExerciseLastDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41133")]
	pub option_exercise_last_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// OptionExerciseEarliestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41134")]
	pub option_exercise_earliest_time: Option<String>,
	/// OptionExerciseLatestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41135")]
	pub option_exercise_latest_time: Option<String>,
	/// OptionExerciseTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41136")]
	pub option_exercise_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OptionExerciseBusinessDayConvention {
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
pub enum OptionExerciseEarliestDateOffsetDayType {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OptionExerciseEarliestDateUnit {
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
pub enum OptionExerciseFrequencyUnit {
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
pub enum OptionExerciseStartDateRelativeTo {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OptionExerciseStartDateOffsetUnit {
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
pub enum OptionExerciseStartDateOffsetDayType {
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
