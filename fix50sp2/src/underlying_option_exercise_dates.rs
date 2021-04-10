
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingOptionExerciseDates {
	/// UnderlyingOptionExerciseBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41822")]
	pub underlying_option_exercise_business_day_convention: Option<UnderlyingOptionExerciseBusinessDayConvention>,
	/// UnderlyingOptionExerciseBusinessCenterGrp
	#[serde(flatten)]
	pub underlying_option_exercise_business_center_grp: Option<super::underlying_option_exercise_business_center_grp::UnderlyingOptionExerciseBusinessCenterGrp>,
	/// UnderlyingOptionExerciseDateGrp
	#[serde(flatten)]
	pub underlying_option_exercise_date_grp: Option<super::underlying_option_exercise_date_grp::UnderlyingOptionExerciseDateGrp>,
	/// UnderlyingOptionExerciseEarliestDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41823")]
	pub underlying_option_exercise_earliest_date_offset_day_type: Option<UnderlyingOptionExerciseEarliestDateOffsetDayType>,
	/// Conditionally required when UnderlyingOptionExerciseEarliestDateUnit(41825) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41824")]
	pub underlying_option_exercise_earliest_date_period: Option<i32>,
	/// Conditionally required when UnderlyingOptionExerciseEarliestDatePeriod(41824) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41825")]
	pub underlying_option_exercise_earliest_date_unit: Option<UnderlyingOptionExerciseEarliestDateUnit>,
	/// Conditinally required when UnderlyingOptionExerciseFrequencyUnit(41827) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41826")]
	pub underlying_option_exercise_frequency_period: Option<i32>,
	/// Conditinally required when UnderlyingOptionExerciseFrequencyPeriod(41826) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41827")]
	pub underlying_option_exercise_frequency_unit: Option<UnderlyingOptionExerciseFrequencyUnit>,
	/// UnderlyingOptionExerciseStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41828")]
	pub underlying_option_exercise_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingOptionExerciseStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41829")]
	pub underlying_option_exercise_start_date_relative_to: Option<UnderlyingOptionExerciseStartDateRelativeTo>,
	/// Conditionally required when UnderlyingOptionExerciseStartDateOffsetUnit(41831) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41830")]
	pub underlying_option_exercise_start_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingOptionExerciseStartDateOffsetPeriod(41830) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41831")]
	pub underlying_option_exercise_start_date_offset_unit: Option<UnderlyingOptionExerciseStartDateOffsetUnit>,
	/// UnderlyingOptionExerciseStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41832")]
	pub underlying_option_exercise_start_date_offset_day_type: Option<UnderlyingOptionExerciseStartDateOffsetDayType>,
	/// UnderlyingOptionExerciseStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41833")]
	pub underlying_option_exercise_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingOptionExerciseSkip
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41834")]
	pub underlying_option_exercise_skip: Option<i32>,
	/// UnderlyingOptionExerciseNominationDeadline
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41835")]
	pub underlying_option_exercise_nomination_deadline: Option<fix_common::LocalMktDate>,
	/// UnderlyingOptionExerciseFirstDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41836")]
	pub underlying_option_exercise_first_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingOptionExerciseLastDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41837")]
	pub underlying_option_exercise_last_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingOptionExerciseEarliestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41838")]
	pub underlying_option_exercise_earliest_time: Option<String>,
	/// UnderlyingOptionExerciseLatestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41839")]
	pub underlying_option_exercise_latest_time: Option<String>,
	/// UnderlyingOptionExerciseTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41840")]
	pub underlying_option_exercise_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingOptionExerciseBusinessDayConvention {
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
pub enum UnderlyingOptionExerciseEarliestDateOffsetDayType {
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
pub enum UnderlyingOptionExerciseEarliestDateUnit {
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
pub enum UnderlyingOptionExerciseFrequencyUnit {
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
pub enum UnderlyingOptionExerciseStartDateRelativeTo {
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
pub enum UnderlyingOptionExerciseStartDateOffsetUnit {
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
pub enum UnderlyingOptionExerciseStartDateOffsetDayType {
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
