
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DividendFXTriggerDate {
	/// DividendFXTriggerDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42265")]
	pub dividend_fx_trigger_date_relative_to: Option<i32>,
	/// Conditionally required when DividendFXTriggerDateOffsetUnit(42267) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42266")]
	pub dividend_fx_trigger_date_offset_period: Option<i32>,
	/// Conditionally required when DividendFXTriggerDateOffsetPeriod(42266) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42267")]
	pub dividend_fx_trigger_date_offset_unit: Option<String>,
	/// DividendFXTriggerDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42268")]
	pub dividend_fx_trigger_date_offset_day_type: Option<i32>,
	/// DividendFXTriggerDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42269")]
	pub dividend_fx_trigger_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The value
	/// would be specific to this instance of DividendFXTriggerDate.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42270")]
	pub dividend_fx_trigger_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The values would
	/// be specific to this instance of DividendFXTriggerDate.
	#[serde(flatten)]
	pub dividend_fx_trigger_date_business_center_grp: Option<super::dividend_fx_trigger_date_business_center_grp::DividendFXTriggerDateBusinessCenterGrp>,
	/// DividendFXTriggerDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42271")]
	pub dividend_fx_trigger_date_adjusted: Option<crate::entities::LocalMktDate>,
}
