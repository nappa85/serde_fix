
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventRelativeDate {
	/// UnderlyingComplexEventDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41739")]
	pub underlying_complex_event_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingComplexEventDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41740")]
	pub underlying_complex_event_date_relative_to: Option<UnderlyingComplexEventDateRelativeTo>,
	/// Conditionally required when UnderlyingComplexEventDateOffsetUnit(41742) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "41741")]
	pub underlying_complex_event_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingComplexEventDateOffsetPeriod(41741) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41742")]
	pub underlying_complex_event_date_offset_unit: Option<UnderlyingComplexEventDateOffsetUnit>,
	/// UnderlyingComplexEventDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41743")]
	pub underlying_complex_event_day_type: Option<UnderlyingComplexEventDayType>,
	/// UnderlyingComplexEventDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41744")]
	pub underlying_complex_event_date_business_day_convention: Option<UnderlyingComplexEventDateBusinessDayConvention>,
	/// UnderlyingComplexEventDateBusinessCenterGrp
	#[serde(flatten)]
	pub underlying_complex_event_date_business_center_grp: Option<super::underlying_complex_event_date_business_center_grp::UnderlyingComplexEventDateBusinessCenterGrp>,
	/// UnderlyingComplexEventDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41745")]
	pub underlying_complex_event_date_adjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingComplexEventFixingTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41746")]
	pub underlying_complex_event_fixing_time: Option<String>,
	/// UnderlyingComplexEventFixingTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "41747")]
	pub underlying_complex_event_fixing_time_business_center: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingComplexEventDateRelativeTo {
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

impl Default for UnderlyingComplexEventDateRelativeTo {
	fn default() -> Self {
		UnderlyingComplexEventDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingComplexEventDateOffsetUnit {
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

impl Default for UnderlyingComplexEventDateOffsetUnit {
	fn default() -> Self {
		UnderlyingComplexEventDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingComplexEventDayType {
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

impl Default for UnderlyingComplexEventDayType {
	fn default() -> Self {
		UnderlyingComplexEventDayType::Business
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingComplexEventDateBusinessDayConvention {
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

impl Default for UnderlyingComplexEventDateBusinessDayConvention {
	fn default() -> Self {
		UnderlyingComplexEventDateBusinessDayConvention::NotApplicable
	}
}
