
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamCompoundingDateGrp {
	/// NoPaymentStreamCompoundingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42606")]
	pub payment_stream_compounding_dates: Option<fix_common::RepeatingValues<PaymentStreamCompoundingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamCompoundingDate {
	/// Required if NoPaymentStreamCompoundingDates(42606) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42607")]
	pub payment_stream_compounding_date: Option<fix_common::LocalMktDate>,
	/// When specified it applies not only to the current date instance but to all subsequent date instances in the group until overridden
	/// when a new type is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42608")]
	pub payment_stream_compounding_date_type: Option<i32>,
}
