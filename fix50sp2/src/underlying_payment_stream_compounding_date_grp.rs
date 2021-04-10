
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamCompoundingDateGrp {
	/// NoUnderlyingPaymentStreamCompoundingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42901")]
	pub underlying_payment_stream_compounding_dates: Option<crate::entities::RepeatingValues<UnderlyingPaymentStreamCompoundingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamCompoundingDate {
	/// Required if NoUnderlyingPaymentStreamCompoundingDates(42901) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42902")]
	pub underlying_payment_stream_compounding_date: Option<crate::entities::LocalMktDate>,
	/// When specified it applies not only to the current date instance but to all subsequent date instances in the group until overridden
	/// when a new type is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42903")]
	pub underlying_payment_stream_compounding_date_type: Option<i32>,
}
