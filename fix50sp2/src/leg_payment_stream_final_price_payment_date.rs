
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamFinalPricePaymentDate {
	/// LegPaymentStreamFinalPricePaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42453")]
	pub leg_payment_stream_final_price_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// LegPaymentStreamFinalPricePaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42454")]
	pub leg_payment_stream_final_price_payment_date_relative_to: Option<i32>,
	/// Conditionally required when LegPaymentStreamFinalPricePaymentDateOffsetUnit(42456) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42455")]
	pub leg_payment_stream_final_price_payment_date_offset_period: Option<i32>,
	/// Conditionally required when LegPaymentStreamFinalPricePaymentDateOffsetPeriod(42455) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42456")]
	pub leg_payment_stream_final_price_payment_date_offset_unit: Option<String>,
	/// LegPaymentStreamFinalPricePaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42457")]
	pub leg_payment_stream_final_price_payment_date_offset_day_type: Option<i32>,
	/// LegPaymentStreamFinalPricePaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42458")]
	pub leg_payment_stream_final_price_payment_date_adjusted: Option<fix_common::LocalMktDate>,
}
