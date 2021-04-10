
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentScheduleRateSourceGrp {
	/// NoUnderlyingPaymentScheduleRateSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40704")]
	pub underlying_payment_schedule_rate_sources: Option<crate::entities::RepeatingValues<UnderlyingPaymentScheduleRateSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentScheduleRateSource {
	/// Required if NoUnderlyingPaymentScheduleRates(40704) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40705")]
	pub underlying_payment_schedule_rate_source: Option<i32>,
	/// Required if NoUnderlyingPaymentScheduleRates(40704) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40706")]
	pub underlying_payment_schedule_rate_source_type: Option<i32>,
	/// Conditionally required when UnderlyingPaymentScheduleRateSource(40705) = 99 (Other).
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40707")]
	pub underlying_payment_schedule_reference_page: Option<String>,
}
