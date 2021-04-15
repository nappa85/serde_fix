
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamResetDates {
	/// LegPaymentStreamResetDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40303")]
	pub leg_payment_stream_reset_date_relative_to: Option<LegPaymentStreamResetDateRelativeTo>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg payment stream reset dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40304")]
	pub leg_payment_stream_reset_date_business_day_convention: Option<LegPaymentStreamResetDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg payment stream reset dates.
	#[serde(flatten)]
	pub leg_payment_stream_reset_date_business_center_grp: Option<super::leg_payment_stream_reset_date_business_center_grp::LegPaymentStreamResetDateBusinessCenterGrp>,
	/// Conditionally required when LegPaymentStreamResetFrequencyUnit(40307) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40306")]
	pub leg_payment_stream_reset_frequency_period: Option<i32>,
	/// Conditionally required when LegPaymentStreamResetFrequencyPeriod(40306) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40307")]
	pub leg_payment_stream_reset_frequency_unit: Option<LegPaymentStreamResetFrequencyUnit>,
	/// LegPaymentStreamResetWeeklyRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40308")]
	pub leg_payment_stream_reset_weekly_roll_convention: Option<LegPaymentStreamResetWeeklyRollConvention>,
	/// LegPaymentStreamInitialFixingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40309")]
	pub leg_payment_stream_initial_fixing_date_relative_to: Option<LegPaymentStreamInitialFixingDateRelativeTo>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg payment stream reset dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40310")]
	pub leg_payment_stream_initial_fixing_date_business_day_convention: Option<LegPaymentStreamInitialFixingDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg payment stream reset dates.
	#[serde(flatten)]
	pub leg_payment_stream_initial_fixing_date_business_center_grp: Option<super::leg_payment_stream_initial_fixing_date_business_center_grp::LegPaymentStreamInitialFixingDateBusinessCenterGrp>,
	/// Conditionally required when LegPaymentStreamInitialFixingDateOffsetUnit(40313) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40312")]
	pub leg_payment_stream_initial_fixing_date_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStreamInitialFixingDateOffsetPeriod(40312) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40313")]
	pub leg_payment_stream_initial_fixing_date_offset_unit: Option<LegPaymentStreamInitialFixingDateOffsetUnit>,
	/// LegPaymentStreamInitialFixingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40314")]
	pub leg_payment_stream_initial_fixing_date_offset_day_type: Option<LegPaymentStreamInitialFixingDateOffsetDayType>,
	/// LegPaymentStreamInitialFixingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40315")]
	pub leg_payment_stream_initial_fixing_date_adjusted: Option<fix_common::LocalMktDate>,
	/// LegPaymentStreamFixingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40316")]
	pub leg_payment_stream_fixing_date_relative_to: Option<LegPaymentStreamFixingDateRelativeTo>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg payment stream reset dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40317")]
	pub leg_payment_stream_fixing_date_business_day_convention: Option<LegPaymentStreamFixingDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg payment stream reset dates.
	#[serde(flatten)]
	pub leg_payment_stream_fixing_date_business_center_grp: Option<super::leg_payment_stream_fixing_date_business_center_grp::LegPaymentStreamFixingDateBusinessCenterGrp>,
	/// Conditionally required when LegPaymentStreamFixingDateOffsetUnit(40320) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40319")]
	pub leg_payment_stream_fixing_date_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStreamFixingDateOffsetPeriod(40319) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40320")]
	pub leg_payment_stream_fixing_date_offset_unit: Option<LegPaymentStreamFixingDateOffsetUnit>,
	/// LegPaymentStreamFixingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40321")]
	pub leg_payment_stream_fixing_date_offset_day_type: Option<LegPaymentStreamFixingDateOffsetDayType>,
	/// LegPaymentStreamFixingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40322")]
	pub leg_payment_stream_fixing_date_adjusted: Option<fix_common::LocalMktDate>,
	/// Conditionally required when LegPaymentStreamRateCutoffDateOffsetUnit(40324) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40323")]
	pub leg_payment_stream_rate_cutoff_date_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStreamRateCutoffDateOffsetPeriod(40323) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40324")]
	pub leg_payment_stream_rate_cutoff_date_offset_unit: Option<LegPaymentStreamRateCutoffDateOffsetUnit>,
	/// LegPaymentStreamRateCutoffDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40325")]
	pub leg_payment_stream_rate_cutoff_date_offset_day_type: Option<LegPaymentStreamRateCutoffDateOffsetDayType>,
	/// LegPaymentStreamFixingDateGrp
	#[serde(flatten)]
	pub leg_payment_stream_fixing_date_grp: Option<super::leg_payment_stream_fixing_date_grp::LegPaymentStreamFixingDateGrp>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamResetDateRelativeTo {
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

impl Default for LegPaymentStreamResetDateRelativeTo {
	fn default() -> Self {
		LegPaymentStreamResetDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamResetDateBusinessDayConvention {
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

impl Default for LegPaymentStreamResetDateBusinessDayConvention {
	fn default() -> Self {
		LegPaymentStreamResetDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamResetFrequencyUnit {
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

impl Default for LegPaymentStreamResetFrequencyUnit {
	fn default() -> Self {
		LegPaymentStreamResetFrequencyUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamResetWeeklyRollConvention {
	/// Monday
	#[serde(rename = "MON")]
	Monday,
	/// Tuesday
	#[serde(rename = "TUE")]
	Tuesday,
	/// Wednesday
	#[serde(rename = "WED")]
	Wednesday,
	/// Thursday
	#[serde(rename = "THU")]
	Thursday,
	/// Friday
	#[serde(rename = "FRI")]
	Friday,
	/// Saturday
	#[serde(rename = "SAT")]
	Saturday,
	/// Sunday
	#[serde(rename = "SUN")]
	Sunday,
}

impl Default for LegPaymentStreamResetWeeklyRollConvention {
	fn default() -> Self {
		LegPaymentStreamResetWeeklyRollConvention::Monday
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamInitialFixingDateRelativeTo {
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

impl Default for LegPaymentStreamInitialFixingDateRelativeTo {
	fn default() -> Self {
		LegPaymentStreamInitialFixingDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamInitialFixingDateBusinessDayConvention {
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

impl Default for LegPaymentStreamInitialFixingDateBusinessDayConvention {
	fn default() -> Self {
		LegPaymentStreamInitialFixingDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamInitialFixingDateOffsetUnit {
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

impl Default for LegPaymentStreamInitialFixingDateOffsetUnit {
	fn default() -> Self {
		LegPaymentStreamInitialFixingDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamInitialFixingDateOffsetDayType {
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

impl Default for LegPaymentStreamInitialFixingDateOffsetDayType {
	fn default() -> Self {
		LegPaymentStreamInitialFixingDateOffsetDayType::Business
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamFixingDateRelativeTo {
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

impl Default for LegPaymentStreamFixingDateRelativeTo {
	fn default() -> Self {
		LegPaymentStreamFixingDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamFixingDateBusinessDayConvention {
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

impl Default for LegPaymentStreamFixingDateBusinessDayConvention {
	fn default() -> Self {
		LegPaymentStreamFixingDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamFixingDateOffsetUnit {
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

impl Default for LegPaymentStreamFixingDateOffsetUnit {
	fn default() -> Self {
		LegPaymentStreamFixingDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamFixingDateOffsetDayType {
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

impl Default for LegPaymentStreamFixingDateOffsetDayType {
	fn default() -> Self {
		LegPaymentStreamFixingDateOffsetDayType::Business
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamRateCutoffDateOffsetUnit {
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

impl Default for LegPaymentStreamRateCutoffDateOffsetUnit {
	fn default() -> Self {
		LegPaymentStreamRateCutoffDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum LegPaymentStreamRateCutoffDateOffsetDayType {
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

impl Default for LegPaymentStreamRateCutoffDateOffsetDayType {
	fn default() -> Self {
		LegPaymentStreamRateCutoffDateOffsetDayType::Business
	}
}
