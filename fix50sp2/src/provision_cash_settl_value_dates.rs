
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionCashSettlValueDates {
	/// ProvisionCashSettlValueTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40114")]
	pub provision_cash_settl_value_time: Option<String>,
	/// ProvisionCashSettlValueTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40115")]
	pub provision_cash_settl_value_time_business_center: Option<String>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the provisional cash settlement value date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40116")]
	pub provision_cash_settl_value_date_business_day_convention: Option<ProvisionCashSettlValueDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the provisional cash settlement value date.
	#[serde(flatten)]
	pub provision_cash_settl_value_date_business_center_grp: Option<super::provision_cash_settl_value_date_business_center_grp::ProvisionCashSettlValueDateBusinessCenterGrp>,
	/// ProvisionCashSettlValueDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40118")]
	pub provision_cash_settl_value_date_relative_to: Option<ProvisionCashSettlValueDateRelativeTo>,
	/// Conditionally required when ProvisionCashSettlValueDateOffsetUnit(40120) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40119")]
	pub provision_cash_settl_value_date_offset_period: Option<i32>,
	/// Conditionally required when ProvisionCashSettlValueDateOffsetPeriod(40119) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40120")]
	pub provision_cash_settl_value_date_offset_unit: Option<ProvisionCashSettlValueDateOffsetUnit>,
	/// ProvisionCashSettlValueDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40121")]
	pub provision_cash_settl_value_date_offset_day_type: Option<ProvisionCashSettlValueDateOffsetDayType>,
	/// ProvisionCashSettlValueDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40122")]
	pub provision_cash_settl_value_date_adjusted: Option<fix_common::LocalMktDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCashSettlValueDateBusinessDayConvention {
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

impl Default for ProvisionCashSettlValueDateBusinessDayConvention {
	fn default() -> Self {
		ProvisionCashSettlValueDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCashSettlValueDateRelativeTo {
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

impl Default for ProvisionCashSettlValueDateRelativeTo {
	fn default() -> Self {
		ProvisionCashSettlValueDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCashSettlValueDateOffsetUnit {
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

impl Default for ProvisionCashSettlValueDateOffsetUnit {
	fn default() -> Self {
		ProvisionCashSettlValueDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCashSettlValueDateOffsetDayType {
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

impl Default for ProvisionCashSettlValueDateOffsetDayType {
	fn default() -> Self {
		ProvisionCashSettlValueDateOffsetDayType::Business
	}
}
