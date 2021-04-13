
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionOptionExerciseDates {
	/// When specified, this overrides the busienss day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the provisional option exercise date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42115")]
	pub underlying_provision_option_exercise_business_day_convention: Option<UnderlyingProvisionOptionExerciseBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the provisional option exercise date.
	#[serde(flatten)]
	pub underlying_provision_option_exercise_business_center_grp: Option<super::underlying_provision_option_exercise_business_center_grp::UnderlyingProvisionOptionExerciseBusinessCenterGrp>,
	/// UnderlyingProvisionOptionExerciseFixedDateGrp
	#[serde(flatten)]
	pub underlying_provision_option_exercise_fixed_date_grp: Option<super::underlying_provision_option_exercise_fixed_date_grp::UnderlyingProvisionOptionExerciseFixedDateGrp>,
	/// Conditionally required when UnderlyingProvisionOptionExerciseEarliestDateOffsetUnit(42117) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42116")]
	pub underlying_provision_option_exercise_earliest_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingProvisionOptionExerciseEasrliestDatePeriod(42116) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42117")]
	pub underlying_provision_option_exercise_earliest_date_offset_unit: Option<String>,
	/// Conditionally required when (42119) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42118")]
	pub underlying_provision_option_exercise_frequency_period: Option<i32>,
	/// Conditionally required when UnderlyingProvisionOptionExerciseFrequencyPeriod(42118) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42119")]
	pub underlying_provision_option_exercise_frequency_unit: Option<UnderlyingProvisionOptionExerciseFrequencyUnit>,
	/// UnderlyingProvisionOptionExerciseStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42120")]
	pub underlying_provision_option_exercise_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingProvisionOptionExerciseStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42121")]
	pub underlying_provision_option_exercise_start_date_relative_to: Option<UnderlyingProvisionOptionExerciseStartDateRelativeTo>,
	/// Conditionally required when UnderlyingProvisionOptionExerciseStartDateOffsetUnit(42123) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42122")]
	pub underlying_provision_option_exercise_start_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingProvisionOptionExerciseStartDateOffsetPeriod(42122) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42123")]
	pub underlying_provision_option_exercise_start_date_offset_unit: Option<UnderlyingProvisionOptionExerciseStartDateOffsetUnit>,
	/// UnderlyingProvisionOptionExerciseStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42124")]
	pub underlying_provision_option_exercise_start_date_offset_day_type: Option<UnderlyingProvisionOptionExerciseStartDateOffsetDayType>,
	/// UnderlyingProvisionOptionExerciseStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42125")]
	pub underlying_provision_option_exercise_start_date_adjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingProvisionOptionExercisePeriodSkip
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42126")]
	pub underlying_provision_option_exercise_period_skip: Option<i32>,
	/// UnderlyingProvisionOptionExerciseBoundsFirstDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42127")]
	pub underlying_provision_option_exercise_bounds_first_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingProvisionOptionExerciseBoundsLastDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42128")]
	pub underlying_provision_option_exercise_bounds_last_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingProvisionOptionExerciseEarliestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42129")]
	pub underlying_provision_option_exercise_earliest_time: Option<String>,
	/// UnderlyingProvisionOptionExerciseEarliestTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42130")]
	pub underlying_provision_option_exercise_earliest_time_business_center: Option<String>,
	/// UnderlyingProvisionOptionExerciseLatestTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42131")]
	pub underlying_provision_option_exercise_latest_time: Option<String>,
	/// UnderlyingProvisionOptionExerciseLatestTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42132")]
	pub underlying_provision_option_exercise_latest_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExerciseBusinessDayConvention {
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

impl Default for UnderlyingProvisionOptionExerciseBusinessDayConvention {
	fn default() -> Self {
		UnderlyingProvisionOptionExerciseBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExerciseFrequencyUnit {
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

impl Default for UnderlyingProvisionOptionExerciseFrequencyUnit {
	fn default() -> Self {
		UnderlyingProvisionOptionExerciseFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExerciseStartDateRelativeTo {
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

impl Default for UnderlyingProvisionOptionExerciseStartDateRelativeTo {
	fn default() -> Self {
		UnderlyingProvisionOptionExerciseStartDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExerciseStartDateOffsetUnit {
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

impl Default for UnderlyingProvisionOptionExerciseStartDateOffsetUnit {
	fn default() -> Self {
		UnderlyingProvisionOptionExerciseStartDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionOptionExerciseStartDateOffsetDayType {
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

impl Default for UnderlyingProvisionOptionExerciseStartDateOffsetDayType {
	fn default() -> Self {
		UnderlyingProvisionOptionExerciseStartDateOffsetDayType::Business
	}
}
