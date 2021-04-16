
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingCashSettlDate {
	/// UnderlyingCashSettlDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42790")]
	pub underlying_cash_settl_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in the Instrument component.
	/// The specified value would be specific to this instance of the cash settlement provision.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42791")]
	pub underlying_cash_settl_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in the Instrument component. The
	/// specified values would be specific to this instance of the cash settlement provision.
	#[serde(flatten)]
	pub underlying_cash_settl_date_business_center_grp: Option<super::underlying_cash_settl_date_business_center_grp::UnderlyingCashSettlDateBusinessCenterGrp>,
	/// UnderlyingCashSettlDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42792")]
	pub underlying_cash_settl_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingCashSettlDateOffsetUnit(42794) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42793")]
	pub underlying_cash_settl_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingCashSettlDateOffsetPeriod(42793) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42794")]
	pub underlying_cash_settl_date_offset_unit: Option<String>,
	/// UnderlyingCashSettlDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42795")]
	pub underlying_cash_settl_date_offset_day_type: Option<i32>,
	/// UnderlyingCashSettlDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42796")]
	pub underlying_cash_settl_date_adjusted: Option<fix_common::LocalMktDate>,
}
