
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateValuationDateGrp {
	/// NoLegReturnRateValuationDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42571")]
	pub leg_return_rate_valuation_dates: Option<fix_common::RepeatingValues<LegReturnRateValuationDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LegReturnRateValuationDate {
	/// Required if NoLegReturnRateValuationDates(42571) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "42572")]
	pub leg_return_rate_valuation_date: Option<fix_common::LocalMktDate>,
	/// When specified it applies not only to the current date instance but to all subsequent date instances in the group until overridden
	/// when a new type is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "42573")]
	pub leg_return_rate_valuation_date_type: Option<i32>,
}
