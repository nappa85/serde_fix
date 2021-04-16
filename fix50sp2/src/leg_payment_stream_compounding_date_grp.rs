
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamCompoundingDateGrp {
	/// NoLegPaymentStreamCompoundingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42405")]
	pub leg_payment_stream_compounding_dates: Option<fix_common::RepeatingValues<LegPaymentStreamCompoundingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegPaymentStreamCompoundingDate {
	/// Required if NoLegPaymentStreamCompoundingDates(42405) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42406")]
	pub leg_payment_stream_compounding_date: Option<fix_common::LocalMktDate>,
	/// When specified it applies not only to the current date instance but to all subsequent date instances in the group until overridden
	/// when a new type is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42407")]
	pub leg_payment_stream_compounding_date_type: Option<i32>,
}
