
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentScheduleRateSourceGrp {
	/// NoPaymentScheduleRateSources
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40868")]
	pub payment_schedule_rate_sources: Option<crate::entities::RepeatingValues<PaymentScheduleRateSource>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentScheduleRateSource {
	/// Required if NoPaymentScheduleRateSources(40868) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40869")]
	pub payment_schedule_rate_source: Option<i32>,
	/// Required if NoPaymentScheduleRateSources(40868) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40870")]
	pub payment_schedule_rate_source_type: Option<i32>,
	/// Conditionally required when PaymentScheduleRateSource(40869) = 99 (Other)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40871")]
	pub payment_schedule_reference_page: Option<String>,
}
