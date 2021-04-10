
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventTimes {
	/// Number of complex event date occurrences for a given complex event.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1494")]
	pub complex_event_times: Option<crate::entities::RepeatingValues<ComplexEventTime>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ComplexEventTime {
	/// Required if NoComplexEventTimes(1494) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1495")]
	pub complex_event_start_time: Option<crate::entities::UTCTimeOnly>,
	/// Required if NoComplexEventTimes(1494) &gt; 0.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "1496")]
	pub complex_event_end_time: Option<crate::entities::UTCTimeOnly>,
}
