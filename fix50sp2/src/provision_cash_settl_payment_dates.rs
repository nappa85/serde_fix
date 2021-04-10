
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisionCashSettlPaymentDates {
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the provisional cash settlement payment dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40163")]
	pub provision_cash_settl_payment_date_business_day_convention: Option<ProvisionCashSettlPaymentDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to this instance of the provisional cash settlement payment dates.
	#[serde(flatten)]
	pub provision_cash_settl_payment_date_business_center_grp: Option<super::provision_cash_settl_payment_date_business_center_grp::ProvisionCashSettlPaymentDateBusinessCenterGrp>,
	/// ProvisionCashSettlPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40165")]
	pub provision_cash_settl_payment_date_relative_to: Option<ProvisionCashSettlPaymentDateRelativeTo>,
	/// Conditionally required when ProvisionCashSettlPaymentDateOffsetUnit(40167) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40166")]
	pub provision_cash_settl_payment_date_offset_period: Option<i32>,
	/// Conditionally required when ProvisionCashSettlPaymentDateOffsetPeriod(40166) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40167")]
	pub provision_cash_settl_payment_date_offset_unit: Option<ProvisionCashSettlPaymentDateOffsetUnit>,
	/// ProvisionCashSettlPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40168")]
	pub provision_cash_settl_payment_date_offset_day_type: Option<ProvisionCashSettlPaymentDateOffsetDayType>,
	/// ProvisionCashSettlPaymentDateRangeFirst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40169")]
	pub provision_cash_settl_payment_date_range_first: Option<fix_common::LocalMktDate>,
	/// ProvisionCashSettlPaymentDateRangeLast
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40170")]
	pub provision_cash_settl_payment_date_range_last: Option<fix_common::LocalMktDate>,
	/// ProvisionCashSettlPaymentFixedDateGrp
	#[serde(flatten)]
	pub provision_cash_settl_payment_fixed_date_grp: Option<super::provision_cash_settl_payment_fixed_date_grp::ProvisionCashSettlPaymentFixedDateGrp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProvisionCashSettlPaymentDateBusinessDayConvention {
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
pub enum ProvisionCashSettlPaymentDateRelativeTo {
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
pub enum ProvisionCashSettlPaymentDateOffsetUnit {
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
pub enum ProvisionCashSettlPaymentDateOffsetDayType {
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
