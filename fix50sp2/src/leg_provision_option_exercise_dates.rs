
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionOptionExerciseDates {
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg provision option exercise dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40476")]
	pub leg_provision_option_exercise_business_day_convention: Option<LegProvisionOptionExerciseBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg provision option exercise dates.
	#[serde(flatten)]
	pub leg_provision_option_exercise_business_center_grp: Option<super::leg_provision_option_exercise_business_center_grp::LegProvisionOptionExerciseBusinessCenterGrp>,
	/// LegProvisionOptionExerciseFixedDateGrp
	#[serde(flatten)]
	pub leg_provision_option_exercise_fixed_date_grp: Option<super::leg_provision_option_exercise_fixed_date_grp::LegProvisionOptionExerciseFixedDateGrp>,
	/// Conditionally required when LegProvisionOptionExerciseEarliestDateUnit(40479) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40478")]
	pub leg_provision_option_exercise_earliest_date_period: Option<i32>,
	/// Conditionally required when LegProvisionOptionExerciseEarliestDatePeriod(40478) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40479")]
	pub leg_provision_option_exercise_earliest_date_unit: Option<LegProvisionOptionExerciseEarliestDateUnit>,
	/// Conditionally required when LegProvisionOptionExerciseFrequencyUnit(40481) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40480")]
	pub leg_provision_option_exercise_frequency_period: Option<i32>,
	/// Conditionally required when LegProvisionOptionExerciseFrequencyPeriod(40480) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40481")]
	pub leg_provision_option_exercise_frequency_unit: Option<LegProvisionOptionExerciseFrequencyUnit>,
	/// LegProvisionOptionExerciseStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40482")]
	pub leg_provision_option_exercise_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegProvisionOptionExerciseStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40483")]
	pub leg_provision_option_exercise_start_date_relative_to: Option<LegProvisionOptionExerciseStartDateRelativeTo>,
	/// Conditionally required when LegProvisionOptionExerciseStartDateOffsetUnit(40485) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40484")]
	pub leg_provision_option_exercise_start_date_offset_period: Option<i32>,
	/// Conditionally required when LegProvisionOptionExerciseStartDateOffsetPeriod(40484) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40485")]
	pub leg_provision_option_exercise_start_date_offset_unit: Option<LegProvisionOptionExerciseStartDateOffsetUnit>,
	/// LegProvisionOptionExerciseStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40486")]
	pub leg_provision_option_exercise_start_date_offset_day_type: Option<LegProvisionOptionExerciseStartDateOffsetDayType>,
	/// LegProvisionOptionExerciseStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40487")]
	pub leg_provision_option_exercise_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// LegProvisionOptionExercisePeriodSkip
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40488")]
	pub leg_provision_option_exercise_period_skip: Option<i32>,
	/// LegProvisionOptionExerciseBoundsFirstDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40489")]
	pub leg_provision_option_exercise_bounds_first_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegProvisionOptionExerciseBoundsLastDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40490")]
	pub leg_provision_option_exercise_bounds_last_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegProvisionOptionExerciseEarliestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40491")]
	pub leg_provision_option_exercise_earliest_time: Option<String>,
	/// LegProvisionOptionExerciseEarliestTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40492")]
	pub leg_provision_option_exercise_earliest_time_business_center: Option<String>,
	/// LegProvisionOptionExerciseLatestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40493")]
	pub leg_provision_option_exercise_latest_time: Option<String>,
	/// LegProvisionOptionExerciseLatestTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40494")]
	pub leg_provision_option_exercise_latest_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegProvisionOptionExerciseBusinessDayConvention {
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

impl Default for LegProvisionOptionExerciseBusinessDayConvention {
	fn default() -> Self {
		LegProvisionOptionExerciseBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegProvisionOptionExerciseEarliestDateUnit {
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

impl Default for LegProvisionOptionExerciseEarliestDateUnit {
	fn default() -> Self {
		LegProvisionOptionExerciseEarliestDateUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegProvisionOptionExerciseFrequencyUnit {
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

impl Default for LegProvisionOptionExerciseFrequencyUnit {
	fn default() -> Self {
		LegProvisionOptionExerciseFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegProvisionOptionExerciseStartDateRelativeTo {
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

impl Default for LegProvisionOptionExerciseStartDateRelativeTo {
	fn default() -> Self {
		LegProvisionOptionExerciseStartDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegProvisionOptionExerciseStartDateOffsetUnit {
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

impl Default for LegProvisionOptionExerciseStartDateOffsetUnit {
	fn default() -> Self {
		LegProvisionOptionExerciseStartDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegProvisionOptionExerciseStartDateOffsetDayType {
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

impl Default for LegProvisionOptionExerciseStartDateOffsetDayType {
	fn default() -> Self {
		LegProvisionOptionExerciseStartDateOffsetDayType::Business
	}
}
