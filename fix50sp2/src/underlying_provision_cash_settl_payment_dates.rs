
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingProvisionCashSettlPaymentDates {
	/// When specified, this overrides the busienss day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the provisional cash settlement payment dates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42092")]
	pub underlying_provision_cash_settl_payment_date_business_day_convention: Option<UnderlyingProvisionCashSettlPaymentDateBusinessDayConvention>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the provisional cash settlement payment date.
	#[serde(flatten)]
	pub underlying_provision_cash_settl_payment_date_business_center_grp: Option<super::underlying_provision_cash_settl_payment_date_business_center_grp::UnderlyingProvisionCashSettlPaymentDateBusinessCenterGrp>,
	/// UnderlyingProvisionCashSettlPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42093")]
	pub underlying_provision_cash_settl_payment_date_relative_to: Option<UnderlyingProvisionCashSettlPaymentDateRelativeTo>,
	/// Conditionally required when UnderlyingProvisionCashSettlPaymentDateOffsetUnit(42095) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42094")]
	pub underlying_provision_cash_settl_payment_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingProvisionCashSettlPaymentDateOffsetPeriod(42094) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42095")]
	pub underlying_provision_cash_settl_payment_date_offset_unit: Option<UnderlyingProvisionCashSettlPaymentDateOffsetUnit>,
	/// UnderlyingProvisionCashSettlPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42096")]
	pub underlying_provision_cash_settl_payment_date_offset_day_type: Option<UnderlyingProvisionCashSettlPaymentDateOffsetDayType>,
	/// UnderlyingProvisionCashSettlPaymentDateRangeFirst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42097")]
	pub underlying_provision_cash_settl_payment_date_range_first: Option<fix_common::LocalMktDate>,
	/// UnderlyingProvisionCashSettlPaymentDateRangeLast
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42098")]
	pub underlying_provision_cash_settl_payment_date_range_last: Option<fix_common::LocalMktDate>,
	/// UnderlyingProvisionCashSettlPaymentFixedDateGrp
	#[serde(flatten)]
	pub underlying_provision_cash_settl_payment_fixed_date_grp: Option<super::underlying_provision_cash_settl_payment_fixed_date_grp::UnderlyingProvisionCashSettlPaymentFixedDateGrp>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionCashSettlPaymentDateBusinessDayConvention {
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

impl Default for UnderlyingProvisionCashSettlPaymentDateBusinessDayConvention {
	fn default() -> Self {
		UnderlyingProvisionCashSettlPaymentDateBusinessDayConvention::NotApplicable
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionCashSettlPaymentDateRelativeTo {
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

impl Default for UnderlyingProvisionCashSettlPaymentDateRelativeTo {
	fn default() -> Self {
		UnderlyingProvisionCashSettlPaymentDateRelativeTo::TradeDate
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionCashSettlPaymentDateOffsetUnit {
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

impl Default for UnderlyingProvisionCashSettlPaymentDateOffsetUnit {
	fn default() -> Self {
		UnderlyingProvisionCashSettlPaymentDateOffsetUnit::Day
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum UnderlyingProvisionCashSettlPaymentDateOffsetDayType {
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

impl Default for UnderlyingProvisionCashSettlPaymentDateOffsetDayType {
	fn default() -> Self {
		UnderlyingProvisionCashSettlPaymentDateOffsetDayType::Business
	}
}
