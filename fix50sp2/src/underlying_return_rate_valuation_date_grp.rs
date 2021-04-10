
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateValuationDateGrp {
	/// NoUnderlyingReturnRateValuationDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43071")]
	pub underlying_return_rate_valuation_dates: Option<crate::entities::RepeatingValues<UnderlyingReturnRateValuationDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingReturnRateValuationDate {
	/// Required if NoUnderlyingReturnRateValuationDates(43071) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "43072")]
	pub underlying_return_rate_valuation_date: Option<crate::entities::LocalMktDate>,
	/// When specified it applies not only to the current date instance but to all subsequent date instances in the group until overridden
	/// when a new type is specified.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "crate::entities::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "43073")]
	pub underlying_return_rate_valuation_date_type: Option<i32>,
}
