
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegStreamTerminationDate {
	/// LegStreamTerminationDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40257")]
	pub leg_stream_termination_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg stream termination date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40258")]
	pub leg_stream_termination_date_business_day_convention: Option<LegStreamTerminationDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg stream termination date.
	#[serde(flatten)]
	pub leg_stream_termination_date_business_center_grp: Option<super::leg_stream_termination_date_business_center_grp::LegStreamTerminationDateBusinessCenterGrp>,
	/// LegStreamTerminationDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40260")]
	pub leg_stream_termination_date_relative_to: Option<LegStreamTerminationDateRelativeTo>,
	/// Conditionally required when LegStreamTerminationDateOffsetUnit(40262) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40261")]
	pub leg_stream_termination_date_offset_period: Option<i32>,
	/// Conditionally required when LegStreamTerminationDateOffsetPeriod(40261) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40262")]
	pub leg_stream_termination_date_offset_unit: Option<LegStreamTerminationDateOffsetUnit>,
	/// LegStreamTerminationDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40263")]
	pub leg_stream_termination_date_offset_day_type: Option<LegStreamTerminationDateOffsetDayType>,
	/// LegStreamTerminationDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40264")]
	pub leg_stream_termination_date_adjusted: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegStreamTerminationDateBusinessDayConvention {
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

impl Default for LegStreamTerminationDateBusinessDayConvention {
	fn default() -> Self {
		LegStreamTerminationDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegStreamTerminationDateRelativeTo {
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

impl Default for LegStreamTerminationDateRelativeTo {
	fn default() -> Self {
		LegStreamTerminationDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegStreamTerminationDateOffsetUnit {
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

impl Default for LegStreamTerminationDateOffsetUnit {
	fn default() -> Self {
		LegStreamTerminationDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegStreamTerminationDateOffsetDayType {
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

impl Default for LegStreamTerminationDateOffsetDayType {
	fn default() -> Self {
		LegStreamTerminationDateOffsetDayType::Business
	}
}
