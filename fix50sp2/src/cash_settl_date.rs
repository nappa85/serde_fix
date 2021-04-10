
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CashSettlDate {
	/// CashSettlDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42207")]
	pub cash_settl_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in the Instrument component.
	/// The specified value would be specific to this instance of the cash settlement provision.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42208")]
	pub cash_settl_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in the Instrument component. The
	/// specified values would be specific to this instance of the cash settlement provision.
	#[serde(flatten)]
	pub cash_settl_date_business_center_grp: Option<super::cash_settl_date_business_center_grp::CashSettlDateBusinessCenterGrp>,
	/// CashSettlDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42209")]
	pub cash_settl_date_relative_to: Option<i32>,
	/// Conditionally required when CashSettlDateOffsetUnit(42211) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42210")]
	pub cash_settl_date_offset_period: Option<i32>,
	/// Conditionally required when CashSettlDateOffsetPeriod(42210) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42211")]
	pub cash_settl_date_offset_unit: Option<String>,
	/// CashSettlDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42212")]
	pub cash_settl_date_offset_day_type: Option<i32>,
	/// CashSettlDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42213")]
	pub cash_settl_date_adjusted: Option<crate::entities::LocalMktDate>,
}
