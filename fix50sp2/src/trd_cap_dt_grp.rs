
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrdCapDtGrp {
	/// Number of date ranges provided (must be 1 or 2 if specified)
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "580")]
	pub dates: Option<fix_common::RepeatingValues<Date>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Date {
	/// Used when reporting other than current day trades. Conditionally required if NoDates &gt; 0
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "75")]
	pub trade_date: Option<fix_common::LocalMktDate>,
	/// LastUpdateTime
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "779")]
	pub last_update_time: Option<fix_common::UTCTimestamp>,
	/// To request trades for a specific time.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "60")]
	pub transact_time: Option<fix_common::UTCTimestamp>,
}
