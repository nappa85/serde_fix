
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamCompoundingDates {
	/// When specified, this overrides the business day convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to payment stream compounding dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42904")]
	pub underlying_payment_stream_compounding_dates_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified values would be specific to payment stream compounding dates.
	#[serde(flatten)]
	pub underlying_payment_stream_compounding_dates_business_center_grp: Option<super::underlying_payment_stream_compounding_dates_business_center_grp::UnderlyingPaymentStreamCompoundingDatesBusinessCenterGrp>,
	/// UnderlyingPaymentStreamCompoundingDateGrp
	#[serde(flatten)]
	pub underlying_payment_stream_compounding_date_grp: Option<super::underlying_payment_stream_compounding_date_grp::UnderlyingPaymentStreamCompoundingDateGrp>,
	/// UnderlyingPaymentStreamCompoundingDatesRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42905")]
	pub underlying_payment_stream_compounding_dates_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamCompoundingDatesOffsetUnit(42907) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42906")]
	pub underlying_payment_stream_compounding_dates_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamCompoundingDatesOffsetPeriod(42906) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42907")]
	pub underlying_payment_stream_compounding_dates_offset_unit: Option<String>,
	/// UnderlyingPaymentStreamCompoundingDatesOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42908")]
	pub underlying_payment_stream_compounding_dates_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingPeriodSkip
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42909")]
	pub underlying_payment_stream_compounding_period_skip: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingStartDate
	#[serde(flatten)]
	pub underlying_payment_stream_compounding_start_date: Option<super::underlying_payment_stream_compounding_start_date::UnderlyingPaymentStreamCompoundingStartDate>,
	/// UnderlyingPaymentStreamCompoundingEndDate
	#[serde(flatten)]
	pub underlying_payment_stream_compounding_end_date: Option<super::underlying_payment_stream_compounding_end_date::UnderlyingPaymentStreamCompoundingEndDate>,
	/// Conditionally required when UnderlyingPaymentStreamCompoundingFrequencyUnit(42911) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42910")]
	pub underlying_payment_stream_compounding_frequency_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamCompoundingFrequencyPeriod(42910) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42911")]
	pub underlying_payment_stream_compounding_frequency_unit: Option<String>,
	/// When specified, this overrides the date roll convention defined in the UnderlyingDateAdjustment component in UnderlyingInstrument.
	/// The specified value would be specific to this instance of the payment stream dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42912")]
	pub underlying_payment_stream_compounding_roll_convention: Option<String>,
	/// UnderlyingPaymentStreamBoundsFirstDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42913")]
	pub underlying_payment_stream_bounds_first_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingPaymentStreamBoundsLastDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42914")]
	pub underlying_payment_stream_bounds_last_date_unadjusted: Option<fix_common::LocalMktDate>,
}
