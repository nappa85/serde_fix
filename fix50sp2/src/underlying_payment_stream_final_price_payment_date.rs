
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamFinalPricePaymentDate {
	/// UnderlyingPaymentStreamFinalPricePaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42949")]
	pub underlying_payment_stream_final_price_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// UnderlyingPaymentStreamFinalPricePaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42950")]
	pub underlying_payment_stream_final_price_payment_date_relative_to: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamFinalPricePaymentDateOffsetUnit(42952) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42951")]
	pub underlying_payment_stream_final_price_payment_date_offset_period: Option<i32>,
	/// Conditionally required when UnderlyingPaymentStreamFinalPricePaymentDateOffsetPeriod(42951) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42952")]
	pub underlying_payment_stream_final_price_payment_date_offset_unit: Option<String>,
	/// UnderlyingPaymentStreamFinalPricePaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42953")]
	pub underlying_payment_stream_final_price_payment_date_offset_day_type: Option<i32>,
	/// UnderlyingPaymentStreamFinalPricePaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42954")]
	pub underlying_payment_stream_final_price_payment_date_adjusted: Option<fix_common::LocalMktDate>,
}
