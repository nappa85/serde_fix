
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionCashSettlPaymentDates {
	/// LegProvisionCashSettlPaymentDateBusinessDayConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40516")]
	pub leg_provision_cash_settl_payment_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg provision cash settlement payment dates.
	#[serde(flatten)]
	pub leg_provision_cash_settl_payment_date_business_center_grp: Option<super::leg_provision_cash_settl_payment_date_business_center_grp::LegProvisionCashSettlPaymentDateBusinessCenterGrp>,
	/// LegProvisionCashSettlPaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40518")]
	pub leg_provision_cash_settl_payment_date_relative_to: Option<i32>,
	/// Conditionally required when LegProvisionCashSettlPaymentDateOffsetUnit(40520) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40519")]
	pub leg_provision_cash_settl_payment_date_offset_period: Option<i32>,
	/// Conditionally required when LegProvisionCashSettlPaymentDateOffsetPeriod(40519) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40520")]
	pub leg_provision_cash_settl_payment_date_offset_unit: Option<String>,
	/// LegProvisionCashSettlPaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40521")]
	pub leg_provision_cash_settl_payment_date_offset_day_type: Option<i32>,
	/// LegProvisionCashSettlPaymentDateRangeFirst
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40522")]
	pub leg_provision_cash_settl_payment_date_range_first: Option<fix_common::LocalMktDate>,
	/// LegProvisionCashSettlPaymentDateRangeLast
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40523")]
	pub leg_provision_cash_settl_payment_date_range_last: Option<fix_common::LocalMktDate>,
	/// LegProvisionCashSettlPaymentFixedDateGrp
	#[serde(flatten)]
	pub leg_provision_cash_settl_payment_fixed_date_grp: Option<super::leg_provision_cash_settl_payment_fixed_date_grp::LegProvisionCashSettlPaymentFixedDateGrp>,
}
