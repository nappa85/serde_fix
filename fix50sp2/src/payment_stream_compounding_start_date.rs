
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamCompoundingStartDate {
	/// PaymentStreamCompoundingStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42646")]
	pub payment_stream_compounding_start_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// PaymentStreamCompoundingStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42647")]
	pub payment_stream_compounding_start_date_relative_to: Option<i32>,
	/// Conditionally required when PaymentStreamCompoundingStartDateOffsetUnit(42649) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42648")]
	pub payment_stream_compounding_start_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentStreamCompoundingStartDateOffsetPeriod(42648) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42649")]
	pub payment_stream_compounding_start_date_offset_unit: Option<String>,
	/// PaymentStreamCompoundingStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42650")]
	pub payment_stream_compounding_start_date_offset_day_type: Option<i32>,
	/// PaymentStreamCompoundingStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42651")]
	pub payment_stream_compounding_start_date_adjusted: Option<fix_common::LocalMktDate>,
}
