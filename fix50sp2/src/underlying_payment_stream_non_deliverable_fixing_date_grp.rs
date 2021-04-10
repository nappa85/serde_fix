
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingPaymentStreamNonDeliverableFixingDateGrp {
	/// NoUnderlyingNonDeliverableFixingDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40656")]
	pub underlying_non_deliverable_fixing_dates: Option<fix_common::RepeatingValues<UnderlyingNonDeliverableFixingDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingNonDeliverableFixingDate {
	/// Required if NoUnderlyingNonDeliverableFixingDates(40656) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "40657")]
	pub underlying_non_deliverable_fixing_date: Option<fix_common::LocalMktDate>,
	/// UnderlyingNonDeliverableFixingDateType
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "fix_common::workarounds::from_opt_str")]// https://github.com/serde-rs/serde/issues/1183
	#[serde(default)]
	#[serde(rename = "40658")]
	pub underlying_non_deliverable_fixing_date_type: Option<i32>,
}
