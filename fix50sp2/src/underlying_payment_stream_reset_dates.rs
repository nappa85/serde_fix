
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamResetDates {
	/// UnderlyingPaymentStreamResetDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40592")]
	pub underlying_payment_stream_reset_date_relative_to: Option<i32>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the underlying instrument's payment stream's reset dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40593")]
	pub underlying_payment_stream_reset_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the underlying instrument's payment stream's reset dates.
	#[serde(flatten)]
	pub underlying_payment_stream_reset_date_business_center_grp: Option<super::underlying_payment_stream_reset_date_business_center_grp::UnderlyingPaymentStreamResetDateBusinessCenterGrp>,
	/// Conditionally required when UnderlyingPaymentStreamResetFrequencyUnit(40596) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40595")]
	pub underlying_payment_stream_reset_frequency_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamResetFrequencyPeriod(40595) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40596")]
	pub underlying_payment_stream_reset_frequency_unit: Option<String>,
	/// UnderlyingPaymentStreamResetWeeklyRollConvention
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40597")]
	pub underlying_payment_stream_reset_weekly_roll_convention: Option<String>,
	/// UnderlyingPaymentStreamInitialFixingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40598")]
	pub underlying_payment_stream_initial_fixing_date_relative_to: Option<i32>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the underlying instrument's payment stream's reset dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40599")]
	pub underlying_payment_stream_initial_fixing_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the underlying instrument's payment stream's reset dates.
	#[serde(flatten)]
	pub underlying_payment_stream_initial_fixing_date_business_center_grp: Option<super::underlying_payment_stream_initial_fixing_date_business_center_grp::UnderlyingPaymentStreamInitialFixingDateBusinessCenterGrp>,
	/// Conditionally required when UnderlyingPaymentStreamInitialFixingDateOffsetUnit(40602) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40601")]
	pub underlying_payment_stream_initial_fixing_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamInitialFixingDateOffsetPeriod(40601) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40602")]
	pub underlying_payment_stream_initial_fixing_date_offset_unit: Option<String>,
	/// UnderlyingPaymentStreamInitialFixingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40603")]
	pub underlying_payment_stream_initial_fixing_date_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStreamInitialFixingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40604")]
	pub underlying_payment_stream_initial_fixing_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// UnderlyingPaymentStreamFixingDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40605")]
	pub underlying_payment_stream_fixing_date_relative_to: Option<i32>,
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the underlying instrument's payment stream's reset dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40606")]
	pub underlying_payment_stream_fixing_date_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to this instance of the underlying instrument's payment stream's reset dates.
	#[serde(flatten)]
	pub underlying_payment_stream_fixing_date_business_center_grp: Option<super::underlying_payment_stream_fixing_date_business_center_grp::UnderlyingPaymentStreamFixingDateBusinessCenterGrp>,
	/// Conditionally required when UnderlyingPaymentStreamFixingDateOffsetUnit(40609) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40608")]
	pub underlying_payment_stream_fixing_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamFixingDateOffsetPeriod(40608) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40609")]
	pub underlying_payment_stream_fixing_date_offset_unit: Option<String>,
	/// UnderlyingPaymentStreamFixingDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40610")]
	pub underlying_payment_stream_fixing_date_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStreamFixingDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40611")]
	pub underlying_payment_stream_fixing_date_adjusted: Option<crate::entities::LocalMktDate>,
	/// Conditionally required when UnderlyingPaymentStreamRateCutoffDateOffsetUnit(40613) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40612")]
	pub underlying_payment_stream_rate_cutoff_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamRateCutoffDateOffsetPeriod(40612) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40613")]
	pub underlying_payment_stream_rate_cutoff_date_offset_unit: Option<String>,
	/// UnderlyingPaymentStreamRateCutoffDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40614")]
	pub underlying_payment_stream_rate_cutoff_date_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStreamFixingDateGrp
	#[serde(flatten)]
	pub underlying_payment_stream_fixing_date_grp: Option<super::underlying_payment_stream_fixing_date_grp::UnderlyingPaymentStreamFixingDateGrp>,
}
