
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamCompoundingEndDate {
	/// LegPaymentStreamCompoundingEndDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42421")]
	pub leg_payment_stream_compounding_end_date_unadjusted: Option<crate::entities::LocalMktDate>,
	/// LegPaymentStreamCompoundingEndDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42422")]
	pub leg_payment_stream_compounding_end_date_relative_to: Option<i32>,
	/// Conditionally required when LegPaymentStreamCompoundingEndDateOffsetUnit(42424) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42423")]
	pub leg_payment_stream_compounding_end_date_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStreamCompoundingEndDateOffsetPeriod(42423) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42424")]
	pub leg_payment_stream_compounding_end_date_offset_unit: Option<String>,
	/// LegPaymentStreamCompoundingEndDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42425")]
	pub leg_payment_stream_compounding_end_date_offset_day_type: Option<i32>,
	/// LegPaymentStreamCompoundingEndDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42426")]
	pub leg_payment_stream_compounding_end_date_adjusted: Option<crate::entities::LocalMktDate>,
}
