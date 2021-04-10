
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionOptionExerciseDates {
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the provisional option exercise dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40123")]
	pub provision_option_exercise_business_day_convention: Option<ProvisionOptionExerciseBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the provisional option exercise dates.
	#[serde(flatten)]
	pub provision_option_exercise_business_center_grp: Option<super::provision_option_exercise_business_center_grp::ProvisionOptionExerciseBusinessCenterGrp>,
	/// ProvisionOptionExerciseFixedDateGrp
	#[serde(flatten)]
	pub provision_option_exercise_fixed_date_grp: Option<super::provision_option_exercise_fixed_date_grp::ProvisionOptionExerciseFixedDateGrp>,
	/// Conditionally required when ProvisionOptionExerciseEarliestDateUnit(40126) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40125")]
	pub provision_option_exercise_earliest_date_period: Option<i32>,
	/// Conditionally required when ProvisionOptionExerciseEasrliestDatePeriod(40125) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40126")]
	pub provision_option_exercise_earliest_date_unit: Option<ProvisionOptionExerciseEarliestDateUnit>,
	/// Conditionally required when ProvisionOptionExerciseFrequencyUnit(40128) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40127")]
	pub provision_option_exercise_frequency_period: Option<i32>,
	/// Conditionally required when ProvisionOptionExerciseFrequencyPeriod(40127) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40128")]
	pub provision_option_exercise_frequency_unit: Option<ProvisionOptionExerciseFrequencyUnit>,
	/// ProvisionOptionExerciseStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40129")]
	pub provision_option_exercise_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// ProvisionOptionExerciseStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40130")]
	pub provision_option_exercise_start_date_relative_to: Option<ProvisionOptionExerciseStartDateRelativeTo>,
	/// Conditionally required when ProvisionOptionExerciseStartDateOffsetUnit(40132) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40131")]
	pub provision_option_exercise_start_date_offset_period: Option<i32>,
	/// Conditionally required when ProvisionOptionExerciseStartDateOffsetPeriod(40131) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40132")]
	pub provision_option_exercise_start_date_offset_unit: Option<ProvisionOptionExerciseStartDateOffsetUnit>,
	/// ProvisionOptionExerciseStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40133")]
	pub provision_option_exercise_start_date_offset_day_type: Option<ProvisionOptionExerciseStartDateOffsetDayType>,
	/// ProvisionOptionExerciseStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40134")]
	pub provision_option_exercise_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// ProvisionOptionExercisePeriodSkip
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40135")]
	pub provision_option_exercise_period_skip: Option<i32>,
	/// ProvisionOptionExerciseBoundsFirstDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40136")]
	pub provision_option_exercise_bounds_first_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// ProvisionOptionExerciseBoundsLastDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40137")]
	pub provision_option_exercise_bounds_last_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// ProvisionOptionExerciseEarliestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40138")]
	pub provision_option_exercise_earliest_time: Option<String>,
	/// ProvisionOptionExerciseEarliestTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40139")]
	pub provision_option_exercise_earliest_time_business_center: Option<String>,
	/// ProvisionOptionExerciseLatestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40140")]
	pub provision_option_exercise_latest_time: Option<String>,
	/// ProvisionOptionExerciseLatestTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40141")]
	pub provision_option_exercise_latest_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionOptionExerciseBusinessDayConvention {
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
pub enum ProvisionOptionExerciseEarliestDateUnit {
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
pub enum ProvisionOptionExerciseFrequencyUnit {
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
pub enum ProvisionOptionExerciseStartDateRelativeTo {
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
pub enum ProvisionOptionExerciseStartDateOffsetUnit {
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
pub enum ProvisionOptionExerciseStartDateOffsetDayType {
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
