
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegProvisionCashSettlValueDates {
	/// LegProvisionCashSettlValueTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40524")]
	pub leg_provision_cash_settl_value_time: Option<String>,
	/// LegProvisionCashSettlValueTimeBusinessCenter
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40525")]
	pub leg_provision_cash_settl_value_time_business_center: Option<String>,
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to this instance of the leg provision cash settlement value date.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40526")]
	pub leg_provision_cash_settl_value_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to this instance of the leg provision cash settlement value date.
	#[serde(flatten)]
	pub leg_provision_cash_settl_value_date_business_center_grp: Option<super::leg_provision_cash_settl_value_date_business_center_grp::LegProvisionCashSettlValueDateBusinessCenterGrp>,
	/// LegProvisionCashSettlValueDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40528")]
	pub leg_provision_cash_settl_value_date_relative_to: Option<i32>,
	/// Conditionally required when LegProvisionCashSettlValueDateOffsetUnit(40530) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40529")]
	pub leg_provision_cash_settl_value_date_offset_period: Option<i32>,
	/// Conditionally required when LegProvisionCashSettlValueDateOffsetPeriod(40529) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40530")]
	pub leg_provision_cash_settl_value_date_offset_unit: Option<String>,
	/// LegProvisionCashSettlValueDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40531")]
	pub leg_provision_cash_settl_value_date_offset_day_type: Option<i32>,
	/// LegProvisionCashSettlValueDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40532")]
	pub leg_provision_cash_settl_value_date_adjusted: Option<crate::entities::LocalMktDate>,
}
