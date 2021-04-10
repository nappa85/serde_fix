
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegComplexEventRelativeDate {
	/// LegComplexEventDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41389")]
	pub leg_complex_event_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// LegComplexEventDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41390")]
	pub leg_complex_event_date_relative_to: Option<LegComplexEventDateRelativeTo>,
	/// Conditionally required when LegComplexEventDateOffsetUnit(41392) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41391")]
	pub leg_complex_event_date_offset_period: Option<i32>,
	/// Conditionally required when LegComplexEventDateOffsetPeriod(41391) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41392")]
	pub leg_complex_event_date_offset_unit: Option<LegComplexEventDateOffsetUnit>,
	/// LegComplexEventDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41393")]
	pub leg_complex_event_day_type: Option<LegComplexEventDayType>,
	/// LegComplexEventDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41394")]
	pub leg_complex_event_date_business_day_convention: Option<LegComplexEventDateBusinessDayConvention>,
	/// LegComplexEventDateBusinessCenterGrp
	#[serde(flatten)]
	pub leg_complex_event_date_business_center_grp: Option<super::leg_complex_event_date_business_center_grp::LegComplexEventDateBusinessCenterGrp>,
	/// LegComplexEventDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41395")]
	pub leg_complex_event_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// LegComplexEventFixingTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41396")]
	pub leg_complex_event_fixing_time: Option<String>,
	/// LegComplexEventFixingTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41397")]
	pub leg_complex_event_fixing_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventDateRelativeTo {
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
	/// Cash settlement vaulation date
	#[serde(rename = "8")]
	CashSettlementVaulationDate,
	/// Option exercise start date
	#[serde(rename = "9")]
	OptionExerciseStartDate,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LegComplexEventDateOffsetUnit {
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
pub enum LegComplexEventDayType {
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
pub enum LegComplexEventDateBusinessDayConvention {
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
