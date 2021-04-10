
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamCompoundingStartDate {
	/// LegPaymentStreamCompoundingStartDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42445")]
	pub leg_payment_stream_compounding_start_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// LegPaymentStreamCompoundingStartDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42446")]
	pub leg_payment_stream_compounding_start_date_relative_to: Option<i32>,
	/// Conditionally required when LegPaymentStreamCompoundingStartDateOffsetUnit(42448) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42447")]
	pub leg_payment_stream_compounding_start_date_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStreamCompoundingStartDateOffsetPeriod(42447) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42448")]
	pub leg_payment_stream_compounding_start_date_offset_unit: Option<String>,
	/// LegPaymentStreamCompoundingStartDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42449")]
	pub leg_payment_stream_compounding_start_date_offset_day_type: Option<i32>,
	/// LegPaymentStreamCompoundingStartDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42450")]
	pub leg_payment_stream_compounding_start_date_adjusted: Option<crate::entities::LocalMktDate>,
}
