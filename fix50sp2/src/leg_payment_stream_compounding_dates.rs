
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamCompoundingDates {
	/// When specified, this overrides the business day convention defined in the LegDateAdjustment component in InstrumentLeg. The
	/// specified value would be specific to payment stream compounding dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42408")]
	pub leg_payment_stream_compounding_dates_business_day_convention: Option<i32>,
	/// When specified, this overrides the business centers defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// values would be specific to payment stream compounding dates.
	#[serde(flatten)]
	pub leg_payment_stream_compounding_dates_business_center_grp: Option<super::leg_payment_stream_compounding_dates_business_center_grp::LegPaymentStreamCompoundingDatesBusinessCenterGrp>,
	/// LegPaymentStreamCompoundingDateGrp
	#[serde(flatten)]
	pub leg_payment_stream_compounding_date_grp: Option<super::leg_payment_stream_compounding_date_grp::LegPaymentStreamCompoundingDateGrp>,
	/// LegPaymentStreamCompoundingDatesRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42409")]
	pub leg_payment_stream_compounding_dates_relative_to: Option<i32>,
	/// Conditionally required when LegPaymentStreamCompoundingDatesOffsetUnit(42411) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42410")]
	pub leg_payment_stream_compounding_dates_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStreamCompoundingDatesOffsetPeriod(42410) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42411")]
	pub leg_payment_stream_compounding_dates_offset_unit: Option<String>,
	/// LegPaymentStreamCompoundingDatesOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42412")]
	pub leg_payment_stream_compounding_dates_offset_day_type: Option<i32>,
	/// LegPaymentStreamCompoundingPeriodSkip
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42413")]
	pub leg_payment_stream_compounding_period_skip: Option<i32>,
	/// LegPaymentStreamCompoundingStartDate
	#[serde(flatten)]
	pub leg_payment_stream_compounding_start_date: Option<super::leg_payment_stream_compounding_start_date::LegPaymentStreamCompoundingStartDate>,
	/// LegPaymentStreamCompoundingEndDate
	#[serde(flatten)]
	pub leg_payment_stream_compounding_end_date: Option<super::leg_payment_stream_compounding_end_date::LegPaymentStreamCompoundingEndDate>,
	/// Conditionally required when LegPayamentStreamCompoundingFrequencyUnit(42415) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42414")]
	pub leg_payment_stream_compounding_frequency_period: Option<i32>,
	/// Conditionally required when LegPayamentStreamCompoundingFrequencyPeriod(42414) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42415")]
	pub leg_payment_stream_compounding_frequency_unit: Option<String>,
	/// When specified, this overrides the date roll convention defined in the LegDateAdjustment component in InstrumentLeg. The specified
	/// value would be specific to this instance of payment stream compounding dates.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42416")]
	pub leg_payment_stream_compounding_roll_convention: Option<String>,
	/// LegPaymentStreamBoundsFirstDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42417")]
	pub leg_payment_stream_bounds_first_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// LegPaymentStreamBoundsLastDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42418")]
	pub leg_payment_stream_bounds_last_date_unadjusted: Option<crate::entities::LocalMktDate>,
}
