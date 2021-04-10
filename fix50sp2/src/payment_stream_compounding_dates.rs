
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamCompoundingDates {
	/// When specified, this overrides the business day convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to payment stream compounding dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42609")]
	pub payment_stream_compounding_dates_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the DateAdjustment component in Instrument. The specified values
	/// would be specific to payment stream compounding dates.
	#[serde(flatten)]
	pub payment_stream_compounding_dates_business_center_grp: Option<super::payment_stream_compounding_dates_business_center_grp::PaymentStreamCompoundingDatesBusinessCenterGrp>,
	/// PaymentStreamCompoundingDateGrp
	#[serde(flatten)]
	pub payment_stream_compounding_date_grp: Option<super::payment_stream_compounding_date_grp::PaymentStreamCompoundingDateGrp>,
	/// PaymentStreamCompoundingDatesRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42610")]
	pub payment_stream_compounding_dates_relative_to: Option<i32>,
	/// Conditionally required when PaymentStreamCompoundingDatesOffsetUnit(42612) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42611")]
	pub payment_stream_compounding_dates_offset_period: Option<i32>,
	/// Conditionally required when PaymentCompoundingDatesOffsetPeriod(42611) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42612")]
	pub payment_stream_compounding_dates_offset_unit: Option<String>,
	/// PaymentStreamCompoundingDatesOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42613")]
	pub payment_stream_compounding_dates_offset_day_type: Option<i32>,
	/// PaymentStreamCompoundingPeriodSkip
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42614")]
	pub payment_stream_compounding_period_skip: Option<i32>,
	/// PaymentStreamCompoundingStartDate
	#[serde(flatten)]
	pub payment_stream_compounding_start_date: Option<super::payment_stream_compounding_start_date::PaymentStreamCompoundingStartDate>,
	/// PaymentStreamCompoundingEndDate
	#[serde(flatten)]
	pub payment_stream_compounding_end_date: Option<super::payment_stream_compounding_end_date::PaymentStreamCompoundingEndDate>,
	/// Conditionally required when PayamentStreamCompoundingFrequencyUnit(42616) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42615")]
	pub payment_stream_compounding_frequency_period: Option<i32>,
	/// Conditionally required when PayamentStreamCompoundingFrequencyPeriod(42615) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42616")]
	pub payment_stream_compounding_frequency_unit: Option<String>,
	/// When specified, this overrides the date roll convention defined in the DateAdjustment component in Instrument. The specified
	/// value would be specific to this instance of the payment stream compounding dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42617")]
	pub payment_stream_compounding_roll_convention: Option<String>,
	/// PaymentStreamBoundsFirstDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42618")]
	pub payment_stream_bounds_first_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// PaymentStreamBoundsLastDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42619")]
	pub payment_stream_bounds_last_date_unadjusted: Option<fix_common::LocalMktDate>,
}
