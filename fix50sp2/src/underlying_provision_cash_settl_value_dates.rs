
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionCashSettlValueDates {
	/// UnderlyingProvisionCashSettlValueTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42104")]
	pub underlying_provision_cash_settl_value_time: Option<String>,
	/// UnderlyingProvisionCashSettlValueTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42105")]
	pub underlying_provision_cash_settl_value_time_business_center: Option<String>,
	/// When specified, this overrides the busienss day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the provisional cash settlement value date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42106")]
	pub underlying_provision_cash_settl_value_date_business_day_convention: Option<UnderlyingProvisionCashSettlValueDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the provisional cash settlement value date.
	#[serde(flatten)]
	pub underlying_provision_cash_settl_value_date_business_center_grp: Option<super::underlying_provision_cash_settl_value_date_business_center_grp::UnderlyingProvisionCashSettlValueDateBusinessCenterGrp>,
	/// UnderlyingProvisionCashSettlValueDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42107")]
	pub underlying_provision_cash_settl_value_date_relative_to: Option<UnderlyingProvisionCashSettlValueDateRelativeTo>,
	/// Conditionally required when UnderlyingProvisionCashSettlValueDateOffsetUnit(42109) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42108")]
	pub underlying_provision_cash_settl_value_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingProvisionCashSettlValueDateOffsetPeriod(42108) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42109")]
	pub underlying_provision_cash_settl_value_date_offset_unit: Option<UnderlyingProvisionCashSettlValueDateOffsetUnit>,
	/// UnderlyingProvisionCashSettlValueDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42110")]
	pub underlying_provision_cash_settl_value_date_offset_day_type: Option<UnderlyingProvisionCashSettlValueDateOffsetDayType>,
	/// UnderlyingProvisionCashSettlValueDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42111")]
	pub underlying_provision_cash_settl_value_date_adjusted: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnderlyingProvisionCashSettlValueDateBusinessDayConvention {
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
pub enum UnderlyingProvisionCashSettlValueDateRelativeTo {
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
pub enum UnderlyingProvisionCashSettlValueDateOffsetUnit {
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
pub enum UnderlyingProvisionCashSettlValueDateOffsetDayType {
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
