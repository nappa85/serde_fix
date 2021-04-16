
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamCompoundingStartDate {
	/// UnderlyingPaymentStreamCompoundingStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42941")]
	pub underlying_payment_stream_compounding_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingPaymentStreamCompoundingStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42942")]
	pub underlying_payment_stream_compounding_start_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamCompoundingStartDateOffsetUnit(42944) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42943")]
	pub underlying_payment_stream_compounding_start_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamCompoundingStartDateOffsetPeriod(42943) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42944")]
	pub underlying_payment_stream_compounding_start_date_offset_unit: Option<String>,
	/// UnderlyingPaymentStreamCompoundingStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42945")]
	pub underlying_payment_stream_compounding_start_date_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42946")]
	pub underlying_payment_stream_compounding_start_date_adjusted: Option<fix_common::LocalMktDate>,
}
