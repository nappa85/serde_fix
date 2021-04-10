
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFixingDateGrp {
	/// NoPaymentStreamFixingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42660")]
	pub payment_stream_fixing_dates: Option<fix_common::RepeatingValues<PaymentStreamFixingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PaymentStreamFixingDate {
	/// Required if NoPaymentStreamFixingDates(42660) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42661")]
	pub payment_stream_fixing_date: Option<fix_common::LocalMktDate>,
	/// When specified it applies not only to the current date instance but to all subsequent date instances in the group until overridden
	/// when a new type is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42662")]
	pub payment_stream_fixing_date_type: Option<i32>,
}
