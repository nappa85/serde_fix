
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventRelativeDate {
	/// ComplexEventDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41020")]
	pub complex_event_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// ComplexEventDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41021")]
	pub complex_event_date_relative_to: Option<ComplexEventDateRelativeTo>,
	/// Conditionally required when ComplexEventDateOffsetUnit(41023) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41022")]
	pub complex_event_date_offset_period: Option<i32>,
	/// Conditionally required when ComplexEventDateOffsetPeriod(41022) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41023")]
	pub complex_event_date_offset_unit: Option<ComplexEventDateOffsetUnit>,
	/// ComplexEventDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41024")]
	pub complex_event_day_type: Option<ComplexEventDayType>,
	/// ComplexEventDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41025")]
	pub complex_event_date_business_day_convention: Option<ComplexEventDateBusinessDayConvention>,
	/// ComplexEventDateBusinessCenterGrp
	#[serde(flatten)]
	pub complex_event_date_business_center_grp: Option<super::complex_event_date_business_center_grp::ComplexEventDateBusinessCenterGrp>,
	/// ComplexEventDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41026")]
	pub complex_event_date_adjusted: Option<fix_common::LocalMktDate>,
	/// ComplexEventFixingTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41027")]
	pub complex_event_fixing_time: Option<String>,
	/// ComplexEventFixingTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41028")]
	pub complex_event_fixing_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ComplexEventDateRelativeTo {
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
pub enum ComplexEventDateOffsetUnit {
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
pub enum ComplexEventDayType {
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
pub enum ComplexEventDateBusinessDayConvention {
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
