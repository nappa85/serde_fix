
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamCompoundingEndDate {
	/// PaymentStreamCompoundingEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42622")]
	pub payment_stream_compounding_end_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// PaymentStreamCompoundingEndDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42623")]
	pub payment_stream_compounding_end_date_relative_to: Option<i32>,
	/// Conditionally required when PaymentStreamCompoundingEndDateOffsetUnit(42625) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42624")]
	pub payment_stream_compounding_end_date_offset_period: Option<i32>,
	/// Conditionally required when PaymentStreamCompoundingEndDateOffsetPeriod(42624) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42625")]
	pub payment_stream_compounding_end_date_offset_unit: Option<String>,
	/// PaymentStreamCompoundingEndDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42626")]
	pub payment_stream_compounding_end_date_offset_day_type: Option<i32>,
	/// PaymentStreamCompoundingEndDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42627")]
	pub payment_stream_compounding_end_date_adjusted: Option<crate::entities::LocalMktDate>,
}
