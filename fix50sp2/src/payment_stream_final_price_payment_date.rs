
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFinalPricePaymentDate {
	/// PaymentStreamFinalPricePaymentDateUnadjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42654")]
	pub payment_stream_final_price_payment_date_unadjusted: Option<fix_common::LocalMktDate>,
	/// PaymentStreamFinalPricePaymentDateRelativeTo
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42655")]
	pub payment_stream_final_price_payment_date_relative_to: Option<i32>,
	/// Conditionally required when PaymentStreamFinalPricePaymentDateOffsetUnit(42657) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42656")]
	pub payment_stream_final_price_payment_date_offsetf_period: Option<i32>,
	/// Conditionally required when PaymentStreamFinalPricePaymentDateOffsetPeriod(42656) is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42657")]
	pub payment_stream_final_price_payment_date_offset_unit: Option<String>,
	/// PaymentStreamFinalPricePaymentDateOffsetDayType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42658")]
	pub payment_stream_final_price_payment_date_offset_day_type: Option<i32>,
	/// PaymentStreamFinalPricePaymentDateAdjusted
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42659")]
	pub payment_stream_final_price_payment_date_adjusted: Option<fix_common::LocalMktDate>,
}
