
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IndexRollMonthGrp {
	/// NoIndexRollMonths
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2734")]
	pub index_roll_months: Option<fix_common::RepeatingValues<IndexRollMonth>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IndexRollMonth {
	/// Required if NoIndexRollMonths(2734) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2733")]
	pub index_roll_month: Option<String>,
}
