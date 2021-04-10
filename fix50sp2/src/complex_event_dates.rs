
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventDates {
	/// Number of complex event date occurrences for a given complex event.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1491")]
	pub complex_event_dates: Option<fix_common::RepeatingValues<ComplexEventDate>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventDate {
	/// Required if NoComplexEventDates(1491) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1492")]
	pub complex_event_start_date: Option<fix_common::UTCTimestamp>,
	/// Required if NoComplexEventDates(1491) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1493")]
	pub complex_event_end_date: Option<fix_common::UTCTimestamp>,
}
