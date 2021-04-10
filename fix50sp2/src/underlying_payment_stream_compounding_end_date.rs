
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamCompoundingEndDate {
	/// UnderlyingPaymentStreamCompoundingEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42917")]
	pub underlying_payment_stream_compounding_end_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingPaymentStreamCompoundingEndDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42918")]
	pub underlying_payment_stream_compounding_end_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamCompoundingEndDateOffsetUnit(42920) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42919")]
	pub underlying_payment_stream_compounding_end_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamCompoundingEndDateOffsetPeriod(42919) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42920")]
	pub underlying_payment_stream_compounding_end_date_offset_unit: Option<String>,
	/// UnderlyingPaymentStreamCompoundingEndDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42921")]
	pub underlying_payment_stream_compounding_end_date_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStreamCompoundingEndDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42922")]
	pub underlying_payment_stream_compounding_end_date_adjusted: Option<fix_common::LocalMktDate>,
}
