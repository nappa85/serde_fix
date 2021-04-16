
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventDates {
	/// NoUnderlyingComplexEventDates
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2053")]
	pub underlying_complex_event_dates: Option<fix_common::RepeatingValues<UnderlyingComplexEventDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnderlyingComplexEventDate {
	/// Required if NoUnderlyingComplexEventDates(2054) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2054")]
	pub underlying_complex_event_start_date: Option<fix_common::UTCTimestamp>,
	/// Required if NoUnderlyingComplexEventDates(2054) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "2055")]
	pub underlying_complex_event_end_date: Option<fix_common::UTCTimestamp>,
}
